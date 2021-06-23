use regex::Regex;
use std::collections::HashMap;
use mysql::Row;
use lazy_static::lazy_static;
use mysql::*;
use mysql::prelude::*;
use quicli::prelude::*;
use inflector::Inflector;
use async_std::path::Path;
use async_std::fs::OpenOptions;
use async_std::prelude::*;
use crate::common::{file_content, first_char_to_uppercase};
use crate::custom_config::CustomConfig;

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

    static ref USE_LIB:String = {
        String::from("use serde_derive;\nuse chrono::prelude::*;\n")
    };

    static ref STRUCT_HEAD:String = {
        String::from("#[crud_table]\n#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]")
    };

}

async fn gen_struct(table_name: String, fields: Vec<Row>, struct_head: &String) -> std::io::Result<String> {
    let mut struct_str = format!("pub struct {} {{\n", table_name);
    for field in fields.iter() {
        let field_name: String = field.get(0).unwrap();
        let mut field_type: String = field.get(1).unwrap();
        field_type = mysql2rust_type(field_type).await?;
        struct_str = format!("{}{}", struct_str, format!("\tpub {}: {},\n", field_name, field_type));
    }

    struct_str = format!("{}\n{}\n{}{}\n", USE_LIB.clone(), struct_head, struct_str, "}");
    Ok(struct_str)
}

async fn mysql2rust_type(field_type: String) -> std::io::Result<String> {
    for (k, v) in FIELD_TYPE.iter() {
        let r = Regex::new(k.trim()).unwrap();
        if r.is_match(&field_type) {
            return Ok(format!("Option<{}>", v));
        }
    }
    Ok(String::new())
}

pub async fn run(config: CustomConfig) -> CliResult {
    let url = format!("mysql://{}", config.conn_str);
    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;
    let tables;
    if config.include_tables.is_some() {
        tables = config.include_tables.unwrap();
    } else {
        let sql = format!("select table_name from information_schema.tables where table_schema= '{}'", config.database);
        tables = conn
            .query(sql)
            .unwrap();
    }

    create_dir(&config.output_dir)?;
    let mut mod_array = vec![];
    let mut exclude_tables: Vec<String> = vec![];
    if config.exclude_tables.is_some() {
        exclude_tables = config.exclude_tables.unwrap();
    }
    for table in tables.iter() {
        if exclude_tables.contains(table) {
            continue;
        }
        mod_array.push(format!("pub mod {};\n", table));
        let sql = format!("show full columns from {}", table);
        let r: Vec<Row> = conn.query(&sql)?;
        let default = &STRUCT_HEAD.to_owned();
        let struct_head = config.struct_head.as_ref().unwrap_or(default);
        let mut table_name = table.to_camel_case();
        table_name = first_char_to_uppercase(table_name).await?;
        let struct_str = gen_struct(table_name, r, struct_head).await?;
        let filepath = format!("{}/{}.rs", &config.output_dir, table);
        let filepath = Path::new(&filepath);
        write_to_file(filepath, &struct_str)?;
    }
    let filepath = format!("{}/{}.rs", &config.output_dir, "mod");
    let file_content = file_content(&filepath).await?;
    for v in mod_array.iter() {
        if !file_content.contains(v) {
            let mut file = OpenOptions::new().append(true).open(&filepath).await?;
            file.write(v.as_bytes()).await?;
        }
    };
    Ok(())
}