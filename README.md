# rbatis-tools

# Generate the RUST structure based on the MySQL table structure
[![Version info](https://img.shields.io/crates/v/rbatis-tools.svg)](https://crates.io/crates/capricorn)
[![Downloads](https://img.shields.io/crates/d/rbatis-tools.svg?style=flat-square)](https://crates.io/crates/capricorn)
[![docs](https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square)](https://docs.rs/rbatis-tools)
[![dependency status](https://deps.rs/crate/rbatis-tools/0.1.9/status.svg)](https://deps.rs/crate/capricorn)

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
        pub struct_name: String,
        pub fields: Vec<Field>, 
        pub comment: String,
    }

    #[derive(Serialize, Clone)]
    pub struct Field {
        pub field_name: String,
        pub field_type: String,
        pub comment: String,
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
        pub {{ v.field_name }}: {{ v.field_type }},
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