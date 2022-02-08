use serde_derive;
use chrono::prelude::*;

/// 用户信息
#[crud_table]
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct User {
	pub id: Option<u64>,
	/// 用户id
	pub user_id: Option<String>,
	/// 用户名
	pub username: Option<String>,
	/// 邮箱
	pub email: Option<String>,
	/// 用户类型
	pub user_type: Option<i8>,
	/// 头像
	pub avatar_url: Option<String>,
	pub is_deleted: Option<u8>,
	pub updated_at: Option<NaiveDateTime>,
	pub created_at: Option<NaiveDateTime>,
}

// ***************************************以下是自定义代码区域******************************************