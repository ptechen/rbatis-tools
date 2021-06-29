use serde_derive;
use chrono::prelude::*;

/// 北京老爬虫数据字段和grab ETL字段映射关系
#[crud_table]
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Comfieldsmap {
	pub id: Option<i32>,
	pub etl_field: Option<String>,
	pub old_field: Option<String>,
	pub remarks: Option<String>,
}
