use serde_derive::{Deserialize, Serialize};

#[derive(Default, Debug, Deserialize, Serialize, Clone)]
pub struct CustomConfig {
    pub database: String,
    pub conn_str: String,
    pub include_tables: Option<Vec<String>>,
    pub exclude_tables: Option<Vec<String>>,
    pub output_dir: String,
    pub struct_head: Option<String>,
}