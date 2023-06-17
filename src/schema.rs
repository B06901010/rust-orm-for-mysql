use std::collections::HashMap;
use std::any::Any;

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
    fn to_schema(&self) -> Schema;
    fn get_field_value<'a>(&'a self, field_name: &str) -> Option<&dyn std::any::Any>;
}

// pub trait Argument {

// }

// impl Argument<T> for [Strig,String, T]
// where T: SchemaProvider
// {

// }

// impl Argument<T> for [Strig,String, Vec<T>]
// where T: SchemaProvider
// {

// }

// impl Argument<T> for T
// where T: SchemaProvider
// {

// }
// extern crate proc_macro;
// use proc_macro::TokenStream;

// use std::collections::HashMap;
// use std::sync::{Arc, Mutex};
// use lazy_static::lazy_static;
// use std::any::type_name;
// use syn::{parse2, ItemStruct, DeriveInput, Generics, Ident, Data, Fields};
// use serde::{Serialize, Deserialize};
// use std::any::Any;
// use proc_macro2::TokenStream;
// use quote::{quote, ToTokens};
// use my_macro_crate::FieldNamesTypesAsArra;


// // #[proc_macro_derive(StructAsSchema)]
// // pub fn aa(input: TokenStream) {
// //     println!("Input Token: {:?}", input); // Print the input token
    
// //     let input = parse_macro_input!(input as DeriveInput);
// // }

// // #[derive(Debug)]
// pub struct Field {
//     pub name: String,
//     pub i: i32,
//     pub ty: String,
//     pub table_column: String,
//     pub tag: String,
// }

// pub struct Schema {
//     pub field_names: Vec<String>,
//     pub field_map: HashMap<String, Field>,
// }

// lazy_static! {
//     static ref STRUCT_CACHE: Mutex<HashMap<String, Arc<Schema>>> = Mutex::new(HashMap::new());
// }

// pub fn struct_for_type<T: 'static + serde::Serialize + serde::Deserialize<'static>>() -> Arc<Schema> {
//     let cache = STRUCT_CACHE.lock().unwrap();
        
//     if let Some(st) = cache.get(type_name::<T>()) {
//         return Arc::clone(st);
//     }

//     drop(cache); // Release the lock

//     let new_schema = create_schema::<T>();
//     let new_schema_arc = Arc::new(new_schema);
    
//     let mut cache = STRUCT_CACHE.lock().unwrap();
//     cache.insert(type_name::<T>().to_string(), Arc::clone(&new_schema_arc));

//     new_schema_arc
// }


// fn create_schema(struct_def: DeriveInput) {
//     let field_names = DeriveInput::FIELD_NAMES_AS_ARRAY;
//     let field_types = get_field_types(&struct_def, &field_names);
//     for (name, ty) in field_names.iter().zip(field_types) {
//         println!("Type of field '{}': {:?}", name, ty);
//     }
// }



// // Function to get the types of fields by their names in a struct
// fn get_field_types(struct_def: &DeriveInput, field_names: &[String]) -> Vec<Option<String>> {
//     if let Data::Struct(data) = &struct_def.data {
//         if let Fields::Named(fields) = &data.fields {
//             return field_names
//                 .iter()
//                 .map(|field_name| {
//                     fields.named.iter().find_map(|field| {
//                         if let Some(ident) = &field.ident {
//                             if ident.to_string() == *field_name {
//                                 return Some(field.ty.to_token_stream().to_string());
//                             }
//                         }
//                         None
//                     })
//                 })
//                 .collect();
//         }
//     }
//     vec![None; field_names.len()]
// }


// fn create_schema<'a, T: Serialize + Deserialize<'a> + Any>() -> Schema {
//     println!("Input Struct: {}", type_name::<T>());

//     let struct_name = type_name::<T>().rsplit("::").next().unwrap();

//     let struct_def = quote! {
//         struct #struct_name;
//     };

//     println!("struct_def: {:?}", struct_def);

//     let fields = parse2::<ItemStruct>(struct_def).unwrap();

//     let mut field_names = Vec::new();
//     let mut field_map = HashMap::new();

//     for field in fields.fields.iter() {
//         let name = field.ident.as_ref().unwrap().to_string();
//         field_names.push(name.clone());
        
//         let i = field.attrs.len() as i32;
//         let ty = format!("{}", field.ty.to_token_stream());

//         let mut table_column = String::new();
//         let mut tag = String::new();
        
//         for attr in field.attrs.iter() {
//             let attr_str = attr.to_token_stream().to_string();
//             if attr_str.contains("table_column") {
//                 table_column = attr_str;
//             } else if attr_str.contains("tag") {
//                 tag = attr_str;
//             }
//         }
        
//         let field = Field {
//             name: name.clone(),
//             i,
//             ty,
//             table_column,
//             tag,
//         };

//         field_map.insert(name, field);
//     }

//     Schema {
//         field_names,
//         field_map,
//     }
// }

