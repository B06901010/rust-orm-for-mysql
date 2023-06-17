use std::collections::HashMap;

pub struct Field {
    pub name: String,
    pub i: i32,
    pub ty: String,
    pub table_column: String,
    pub tag: String,
}

pub struct Schema {
    pub field_names: Vec<String>,
    pub field_map: HashMap<String, Field>,
}

pub trait SchemaProvider {
    fn to_schema() -> Schema;
    fn get_field_value(&self, field_name: &str) -> Option<&dyn std::any::Any>;
}