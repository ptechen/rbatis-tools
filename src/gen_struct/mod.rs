use async_std::fs::File;
use async_std::fs::OpenOptions;
use async_std::path::Path;
use async_std::prelude::*;
use async_trait::async_trait;
use mysql::Row;
use quicli::prelude::*;
use tera::{Result, Tera};

pub mod mysql_struct;
pub mod template;

pub struct GenTemplateData {
    pub table_name: String,
    pub struct_name: String,
    pub mysql_rows: Option<Vec<Row>>,
    pub table_comment: String,
    pub tera: Tera,
    pub template_name: String,
}

#[async_trait]
pub trait GenStruct {
    async fn run(&self) -> CliResult;

    async fn get_rust_type(&self, field_type: &str) -> Result<String>;

    async fn gen_template_data(&self, gen_template_data: GenTemplateData) -> Result<String>;

    async fn write_to_file(&self, filepath: &str, content: &str) -> CliResult {
        let filepath = Path::new(&filepath);
        let s = write_to_file(filepath, content)?;
        Ok(s)
    }

    async fn append_to_file(
        &self,
        mod_array: Vec<String>,
        file_content: &str,
        filepath: &str,
    ) -> CliResult {
        for v in mod_array.iter() {
            if !file_content.contains(v) {
                let mut file = OpenOptions::new().append(true).open(filepath).await?;
                file.write(v.as_bytes()).await?;
            };
        }
        Ok(())
    }

    async fn file_content(&self, filepath: &String) -> Result<String> {
        let s = read_file(filepath);
        let s = match s {
            Ok(d) => d,
            Err(_e) => {
                File::create(filepath)
                    .await
                    .with_context(|_| format!("Could not create/open file {:?}", &filepath))
                    .unwrap();
                String::new()
            }
        };
        Ok(s)
    }

    /// 字符串首字母大写
    async fn first_char_to_uppercase(&self, params: &str) -> Result<String> {
        let mut v: Vec<char> = params.chars().collect();
        v[0] = v[0].to_uppercase().nth(0).unwrap();
        let res = v.into_iter().collect();
        Ok(res)
    }
}
