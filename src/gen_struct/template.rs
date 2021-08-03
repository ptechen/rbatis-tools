use quicli::prelude::*;
use tera::{Context, Result, Tera};

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
}

impl Template {
    pub fn new(table_name: String, struct_name: String, fields: Vec<Field>, comment: String) -> Template {
        Template {
            table_name,
            struct_name,
            fields,
            comment,
        }
    }

    pub async fn render(&self, tera: &Tera, template_path: &str) -> Result<String> {
        let mut context = Context::new();
        context.insert("template", self);
        let s = tera.render(template_path, &context)?;
        Ok(s)
    }
}
