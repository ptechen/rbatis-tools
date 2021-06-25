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

impl Comfieldsmap {
    pub fn get_id(&self) -> Option<i32> {
    	self.id
    }
    pub fn get_etl_field(&self) -> Option<String> {
    	self.etl_field
    }
    pub fn get_old_field(&self) -> Option<String> {
    	self.old_field
    }
    pub fn get_remarks(&self) -> Option<String> {
    	self.remarks
    }
}