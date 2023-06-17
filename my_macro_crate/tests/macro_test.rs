#![feature(trace_macros)]
#[cfg(test)]
mod tests {
    trace_macros!(true);
    use super::*;
    use my_macro_crate::SQLTable;
    use proc_macro2::TokenStream;
    use quote::quote;
    use std::collections::HashMap;
    use std::any::Any;
    use syn::punctuated::Punctuated;
    use syn::parse_macro_input;
    use syn::token::Comma;
    use syn::parse_quote;
    // use crate::schema::{Field, Schema};

    #[derive(SQLTable)]
    struct User {
        name: String,
        age: i32,
    }

    #[derive(Debug)]
    pub struct Field {
        pub name: String,
        pub i: i32,
        pub ty: String,
        pub table_column: String,
        pub tag: String,
    }
    
    #[derive(Debug)]
    pub struct Schema {
        pub field_names: Vec<String>,
        pub field_map: HashMap<String, Field>,
    }

    pub trait SchemaProvider {
        fn to_schema(&self) -> Schema;
        fn get_field_value<'a>(&'a self, field_name: &str) -> Option<&dyn std::any::Any>;
    }

    #[test]
    fn test_derive_field_names_types_as_array() {
        

        let a: [&'static str; 3] = ["10","11","12"];
        // println!("{:?}", a);

        let b = User::toSchema();

        // println!("{:?}", b);

        #[derive(SQLTable)]
        struct Payment {
            id: i32,
            amount: i32,
            customer_name: String,
        }

        let user = User {
            name: "John Doe".to_string(),
            age: 25,
        };
    
        let field_name = "name";
        let field_value = user.get_field_value(field_name).expect("REASON").downcast_ref::<String>();

        if let Some(value) = field_value {
            eprintln!("Field value: {:?}", field_value);
        } else {
            eprintln!("Field not found");
        }

        assert_eq!(1,0);
    
    }

    fn test_user() {

        // struct User {
        //     name: String,
        //     age: u32,
        // }
        
        // impl SchemaProvider for User {
        //     fn get_field_value(&self, field_name: &str) -> Option<&dyn Any> {
        //         match field_name {
        //             "name" => Some(&self.name as &dyn Any),
        //             "age" => Some(&self.age as &dyn Any),
        //             _ => None,
        //         }
        //     }
        // }

        #[derive(SQLTable)]
        struct Payment {
            customer_id: i32,
            amount: i32,
            account_name: String,
        }

        // SQLTable generates the following code
        // impl SchemaProvider for Payment {
        //     fn get_field_value(&self, field_name: &str) -> Option<&dyn Any> {
        //         match field_name {
        //             "customer_id" => Some(&self.customer_id as &dyn Any),
        //             "amount" => Some(&self.amount as &dyn Any),
        //             "account_name" => Some(&self.account_name as &dyn Any)
        //             _ => None,
        //         }
        //     }
        // }

        #[derive(SQLTable)]
        struct User {
            name: String,
            age: u32,
        }

        // SQLTable generates the following code
        // impl SchemaProvider for User {
        //     fn get_field_value(&self, field_name: &str) -> Option<&dyn Any> {
        //         match field_name {
        //             "name" => Some(&self.name as &dyn Any),
        //             "age" => Some(&self.age as &dyn Any),
        //             _ => None,
        //         }
        //     }
        // }
        
        let user = User {
            name: "John Doe".to_string(),
            age: 25,
        };
    
        let field_name = "name";
        let field_value = user.get_field_value(field_name).expect("REASON").downcast_ref::<String>();
    
        if let Some(value) = field_value {
            eprintln!("Field value: {:?}", field_value);
        } else {
            eprintln!("Field not found");
        }

        assert_eq!(1,0);
    }
    fn test_schema_provider() {
        // Create an instance of MyStruct
        let user = User {
            name: "Bob".to_string(),
            age: 80,
        };

        // Call the `to_schema` method on the instance
        let schema = user.to_schema();

        println!("{:?}", schema);

        // Assert the expected values from the schema
        assert_eq!(schema.field_names.len(), 3);  // Assuming MyStruct has 2 fields
        // Add more assertions based on the expected schema values

        // Add more assertions to test other field values
    }
}
