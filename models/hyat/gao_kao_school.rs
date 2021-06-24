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
