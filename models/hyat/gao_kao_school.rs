use serde_derive;
use chrono::prelude::*;

#[crud_table]
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct GaoKaoSchool {
	pub id: Option<u32>,
	pub school_name: Option<String>,
	/// uuid
	pub uuid: Option<String>,
	/// 数据
	pub content: Option<String>,
	/// 版本
	pub version: Option<i8>,
	/// 1:删除, 0:未删除
	pub is_deleted: Option<u8>,
	/// 更新时间
	pub updated_at: Option<NaiveDateTime>,
	/// 创建时间
	pub created_at: Option<NaiveDateTime>,
}

impl GaoKaoSchool {
    pub fn get_id(&self) -> Option<u32> {
    	self.id
    }
    pub fn get_school_name(&self) -> Option<String> {
    	self.school_name
    }
    pub fn get_uuid(&self) -> Option<String> {
    	self.uuid
    }
    pub fn get_content(&self) -> Option<String> {
    	self.content
    }
    pub fn get_version(&self) -> Option<i8> {
    	self.version
    }
    pub fn get_is_deleted(&self) -> Option<u8> {
    	self.is_deleted
    }
    pub fn get_updated_at(&self) -> Option<NaiveDateTime> {
    	self.updated_at
    }
    pub fn get_created_at(&self) -> Option<NaiveDateTime> {
    	self.created_at
    }
}