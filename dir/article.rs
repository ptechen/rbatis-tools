use serde_derive;
use chrono::prelude::*;

/// 用户信息
#[crud_table]
#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Article {
	pub article_id: Option<u64>,
	/// 用户id
	pub user_id: Option<String>,
	/// 用户类型
	pub user_type: Option<i8>,
	/// 文章名
	pub article_title: Option<String>,
	/// 内容简述
	pub article_content: Option<String>,
	/// 头像
	pub article_url: Option<String>,
	/// 点赞数
	pub likes: Option<u32>,
	pub is_deleted: Option<u8>,
	pub updated_at: Option<NaiveDateTime>,
	pub created_at: Option<NaiveDateTime>,
}

// ***************************************以下是自定义代码区域******************************************