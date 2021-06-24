# rbatis-tools

# Generate the RUST structure based on the MySQL table structure
[![Version info](https://img.shields.io/crates/v/rbatis-tools.svg)](https://crates.io/crates/capricorn)
[![Downloads](https://img.shields.io/crates/d/rbatis-tools.svg?style=flat-square)](https://crates.io/crates/capricorn)
[![docs](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/rbatis-tools)
[![dependency status](https://deps.rs/crate/rbatis-tools/0.1.6/status.svg)](https://deps.rs/crate/capricorn)

## Install
    cargo install rbatis-tools
## Exec
    rbatis-tools mysql -f reverse.yml

## Example
    use serde_derive;
    use chrono::prelude::*;

    #[crud_table]
    #[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
    /// Test
    pub struct Test {
        pub id: Option<u32>,
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