## New package supports Mysql and PostgresSQL...
[![Version info](https://img.shields.io/crates/v/sql_reverse.svg)](https://crates.io/crates/sql_reverse)
[![Downloads](https://img.shields.io/crates/d/sql_reverse.svg?style=flat-square)](https://crates.io/crates/sql_reverse)
[![docs](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/sql_reverse)
[![dependency status](https://deps.rs/crate/sql_reverse/0.1.0/status.svg)](https://deps.rs/crate/sql_reverse)
[![B站](https://img.shields.io/badge/B站-latest-blue.svg?style=flat-square)](https://www.bilibili.com/video/BV1dF411L7P7?spm_id_from=333.337.search-card.all.click)

## Install
    cargo install sql_reverse

# rbatis-tools

# Generate the RUST structure based on the MySQL table structure
[![Version info](https://img.shields.io/crates/v/rbatis-tools.svg)](https://crates.io/crates/rbatis-tools)
[![Downloads](https://img.shields.io/crates/d/rbatis-tools.svg?style=flat-square)](https://crates.io/crates/rbatis-tools)
[![docs](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/rbatis-tools)
[![dependency status](https://deps.rs/crate/rbatis-tools/0.1.11/status.svg)](https://deps.rs/crate/rbatis-tools)

## Install
    cargo install rbatis-tools

## Exec，you need to make sure you're in the same directory as templates.
    rbatis-tools mysql -f reverse.yml
## Custom Exec
    rbatis-tools mysql -f reverse.yml -p 'templates/*' -n base.tera

## reverse.yml
    database: db_name
    conn_str: root:password@127.0.0.1:3306/db_name
    include_tables: # Include tables, can be ignored.
    #  - table_name
    exclude_tables: # Exclude, tables, can be ignored.
    #  - table_name
    output_dir: ./dir # code output directory

## Template Struct:
    #[derive(Serialize)]
    pub struct Template {
        pub table_name: String,
        pub struct_name: String,
        pub fields: Vec<Field>, 
        pub comment: String,
    }

    #[derive(Serialize, Clone)]
    pub struct Field {
        pub field_name: String,
        pub field_type: String,
        pub comment: String,
        /// NO, YES
        pub is_null: String,
    }

## Template:
    use serde_derive;
    use chrono::prelude::*;

    {% if template.comment -%}
        /// {{ template.comment }}
    {% endif -%}
    #[crud_table]
    #[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
    pub struct {{ template.struct_name }} {
    {%- for v in template.fields %}
        {% if v.comment -%}
            /// {{ v.comment }}
        {% endif -%}
        pub {{ v.field_name }}: Option<{{ v.field_type }}>,
    {%- endfor %}
    }

## Gen Struct Example:
    use serde_derive;
    use chrono::prelude::*;
    
    /// Test
    #[crud_table]
    #[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)
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
