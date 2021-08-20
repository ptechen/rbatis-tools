use crate::custom_config::CustomConfig;
use crate::gen_struct::template::{Field, Template};
use crate::gen_struct::{GenStruct, GenTemplateData};
use async_trait::async_trait;
use inflector::Inflector;
use lazy_static::lazy_static;
use mysql::prelude::*;
use mysql::Row;
use mysql::*;
use quicli::prelude::*;
use regex::Regex;
use std::collections::HashMap;
use tera::{Result, Tera};
use std::fs;

const FLAG: &'static str = "// ***************************************以下是自定义代码区域******************************************";

lazy_static! {
    static ref FIELD_TYPE: HashMap<&'static str, &'static str> = {
        let mut map = HashMap::new();
        map.insert(r"^int\(\d+\)$", "i32");
        map.insert(r"^int\(\d+\) unsigned$", "u32");
        map.insert(r"^integer\(\d+\)$", "i32");
        map.insert(r"^integer\(\d+\) unsigned$", "u32");
        map.insert(r"^tinyint\(\d+\)$", "i8");
        map.insert(r"^tinyint\(\d+\) unsigned$", "u8");
        map.insert(r"^smallint\(\d+\)$", "i16");
        map.insert(r"^smallint\(\d+\) unsigned$", "u16");
        map.insert(r"^mediumint\(\d+\)$", "i32");
        map.insert(r"^mediumint\(\d+\) unsigned$", "u32");
        map.insert(r"^bigint\(\d+\)$", "i64");
        map.insert(r"^bigint\(\d+\) unsigned$", "u64");
        map.insert(r"^float", "f32");
        map.insert(r"^double", "f64");
        map.insert(r"^decimal", "String");
        map.insert(r"^date$", "Date");
        map.insert(r"^datetime$", "NaiveDateTime");
        map.insert(r"^timestamp$", "NaiveDateTime");
        map.insert(r"year", "Year");
        map.insert(r"char", "String");
        map.insert(r"text", "String");
        map.insert(r"blob", "Vec<u8>");
        map
    };
}

#[derive(Clone)]
pub struct MysqlStruct {
    pub config: CustomConfig,
    pub tera: Tera,
    pub template_name: String,
}

impl MysqlStruct {
    pub fn new(
        config: CustomConfig,
        template_path: String,
        template_name: String,
    ) -> Result<MysqlStruct> {
        let tera = Tera::new(&template_path)?;
        Ok(MysqlStruct {
            config,
            tera,
            template_name,
        })
    }
}

#[async_trait]
impl GenStruct for MysqlStruct {
    async fn run(&self) -> CliResult {
        let url = format!("mysql://{}", self.config.conn_str);
        let pool = Pool::new(url)?;
        let mut conn = pool.get_conn()?;
        let tables;
        let include_tables = self.config.include_tables.as_ref();
        if include_tables.is_some() {
            tables = include_tables.unwrap().to_owned();
        } else {
            let sql = format!(
                "select table_name from information_schema.tables where table_schema= '{}'",
                self.config.database
            );
            tables = conn.query(sql)?;
        }
        let tables_status: Vec<Row> = conn.query("show table status").unwrap();
        let mut table_comment_map = HashMap::new();
        for row in tables_status.iter() {
            let table_name: String = row.get(0).unwrap();
            let table_comment: Option<String> = row.get(17);
            if table_comment.is_some() {
                let table_comment = table_comment.unwrap();
                if table_comment != "" {
                    table_comment_map.insert(table_name, table_comment);
                };
            };
        }
        create_dir(&self.config.output_dir)?;
        let mut mod_array = vec![];
        let mut exclude_tables: Vec<String> = vec![];
        if self.config.exclude_tables.is_some() {
            exclude_tables = self.config.exclude_tables.as_ref().unwrap().to_owned();
        };
        for table_name in tables.iter() {
            if exclude_tables.contains(table_name) {
                continue;
            };
            mod_array.push(format!("pub mod {};\n", table_name));
            let sql = format!("show full columns from {}", table_name);
            let mysql_rows: Vec<Row> = conn.query(&sql)?;
            let mut struct_name = table_name.to_camel_case();
            struct_name = self.first_char_to_uppercase(&struct_name).await?;
            let default = &String::new();
            let table_comment = table_comment_map.get(table_name).unwrap_or(default).to_string();
            let mysql_rows = Some(mysql_rows);
            let gen_template_data = GenTemplateData {
                table_name: table_name.to_owned(),
                struct_name,
                mysql_rows,
                table_comment,
                tera: self.tera.to_owned(),
                template_name: self.template_name.to_owned(),
            };
            let mut struct_str = self.gen_template_data(gen_template_data).await?;
            let filepath = format!("{}/{}.rs", self.config.output_dir, table_name);
            let mut custom = String::new();
            match fs::read_to_string(&filepath) {
                Ok(d) => {
                    let vv:Vec<&str> = d.split(FLAG).collect();
                    custom = vv.get(1).unwrap_or(&"").to_string();
                }
                _ => {}
            }
            struct_str = struct_str + "\n" + FLAG + custom.as_str();
            println!("{}", struct_str);
            self.write_to_file(&filepath, &struct_str).await?;
        }
        let filepath = format!("{}/{}.rs", self.config.output_dir, "mod");
        let file_content = self.file_content(&filepath).await?;
        self.append_to_file(mod_array, &file_content, &filepath)
            .await?;
        Ok(())
    }

    async fn get_rust_type(&self, field_type: &str) -> Result<String> {
        for (k, v) in FIELD_TYPE.iter() {
            let r = Regex::new(k.trim()).unwrap();
            if r.is_match(&field_type) {
                return Ok(v.to_string());
            }
        }
        Ok(String::new())
    }

    async fn gen_template_data(&self, gen_template_data: GenTemplateData) -> Result<String> {
        let mut fields = vec![];
        let mysql_rows = gen_template_data.mysql_rows.unwrap();
        for row in mysql_rows.iter() {
            let field_name: String = row.get(0).unwrap();
            let mut field_type: String = row.get(1).unwrap();
            field_type = self.get_rust_type(&field_type).await?;

            let comment: String = row.get(8).unwrap_or(String::new());
            let field = Field {
                field_name,
                field_type,
                comment,
            };
            fields.push(field);
        }
        let temp = Template::new(
            gen_template_data.table_name,
            gen_template_data.struct_name,
            fields,
            gen_template_data.table_comment,
        );
        let struct_str = temp
            .render(&gen_template_data.tera, &gen_template_data.template_name)
            .await?;
        Ok(struct_str)
    }
}
