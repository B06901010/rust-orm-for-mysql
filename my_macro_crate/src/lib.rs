extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{parse_macro_input, AttrStyle, Attribute, Data, DeriveInput, Fields, Meta, Visibility, Type};
use std::collections::HashMap;
use syn::__private::TokenStream2;
// use crate::schema::SchemaProvider;
// mod schema;
// use schema::{SchemaProvider, Field, Schema};

const ERR_MSG: &str = "Derive(FieldNamesAsArray) only applicable to named structs";

mod attrs;
use attrs::{ContainerAttribute, FieldAttribute, ParseAttribute};

mod schema;
use schema::{Field, Schema, SchemaProvider};

/// Adds the `FIELD_NAMES_AS_ARRAY` constant to the deriving struct.
///
/// # Panics
///
/// If the token stream is not coming from a named struct or if
/// the `field_names_as_array` attribute is used wrongfully, deriving
/// this macro will fail.
///
/// # Examples
///
/// ```
/// use struct_field_names_as_array::FieldNamesAsArray;
///
/// #[derive(FieldNamesAsArray)]
/// struct Foo {
///     bar: String,
///     baz: String,
///     bat: String,
/// }
///
/// assert_eq!(Foo::FIELD_NAMES_AS_ARRAY, ["bar", "baz", "bat"]);
/// ```
///

#[proc_macro_derive(SQLTable, attributes(field_names_as_array))]
pub fn derive_field_names_types_as_array(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;
    let (impl_generics, type_generics, where_clause) = &input.generics.split_for_impl();

    let c_attrs = attributes::<ContainerAttribute>(&input.attrs);

    let f_names: Punctuated<String, Comma> = match input.data {
        Data::Struct(ref data_struct) => match &data_struct.fields {
            Fields::Named(fields) => fields
                .named
                .clone().into_iter()
                .filter_map(|f| {
                    let attrs = attributes::<FieldAttribute>(&f.attrs);

                    if let Some(attr) = attrs.first() {
                        match attr {
                            FieldAttribute::Skip => return None,
                        }
                    }

                    let mut res = f.ident.unwrap().to_string();

                    for t in &c_attrs {
                        res = t.apply(&res);
                    }

                    Some(res)
                })
                .collect(),
            _ => panic!("{}", ERR_MSG),
        },
        _ => panic!("{}", ERR_MSG),
    };

    
    let f_types: Punctuated<String, Comma> = match input.data {
        Data::Struct(ref data_struct) => match &data_struct.fields {
            Fields::Named(fields) => fields
                .named
                .clone().into_iter()
                .map(|f| {
                    let field_type = f.ty.clone();
                    quote::quote! { #field_type }.to_string()
                })
                .collect(),
            _ => panic!("{}", ERR_MSG),
        },
        _ => panic!("{}", ERR_MSG),
    };


    let len = f_names.len();

    let it: Vec<(syn::Expr, String)> = f_names.iter()
    .enumerate().map(|(i,field_name)|{
        (syn::parse_str(field_name).expect("Unable to parse"),field_name.to_string())
    }).collect();
    // println!("{:?}", it);

    let field_value_tokens: Vec<TokenStream2> = it
        .iter()
        .enumerate()
        .map(|(i, (field_name, field_name_2))| {
            quote! {
                #field_name_2 => Some(&self.#field_name as &dyn std::any::Any),
            }
        })
        .collect();

    // eprintln!("{:#?}", field_value_tokens);
    // for i in 0..field_value_tokens.len() {
    //     eprintln!("{}", field_value_tokens[i])
    // }

    let vis = c_attrs
        .into_iter()
        .rev()
        .find_map(|a| match a {
            ContainerAttribute::Visibility(v) => Some(v),
            _ => None,
        })
        .unwrap_or(Visibility::Inherited);

    let result = quote! {
        impl #impl_generics #name #type_generics #where_clause {
            fn toSchema() -> Schema {

                let mut sch = Schema {
                    field_names: Vec::new(),
                    field_map: HashMap::new(),
                };

                const FIELD_NAMES_AS_ARRAY: [&'static str; #len] = [#f_names];
                const FIELD_TYPES_AS_ARRAY: [&'static str; #len] = [#f_types];
                
                for i in 0..FIELD_NAMES_AS_ARRAY.len() {
                    let field_name = FIELD_NAMES_AS_ARRAY[i].to_string();
                    let field_type = FIELD_TYPES_AS_ARRAY[i].to_string();

                    let field = Field {
                        name: field_name.clone(),
                        i: i as i32,
                        ty: field_type.clone(),
                        table_column: field_name.clone(),
                        tag: String::new(),
                    };

                    sch.field_names.push(field_name.clone());
                    sch.field_map.insert(field_name.clone(), field);
                }

                sch
            }
        }

        impl SchemaProvider for #name #type_generics #where_clause {
            fn to_schema(&self) -> Schema {
                Self::toSchema()
            }
            fn get_field_value<'a>(&'a self, field_name: &str) -> Option<&dyn std::any::Any> {
                match field_name {
                    #(#field_value_tokens)*
                    _ => None,
                }
            }
        }
    };

    TokenStream::from(result)
}

fn attributes<A: ParseAttribute>(attrs: &[Attribute]) -> Vec<A> {
    let mut res = Vec::new();

    for attr in attrs {
        if attr.style != AttrStyle::Outer {
            continue;
        }

        let attr_name = attr
            .path
            .segments
            .iter()
            .last()
            .cloned()
            .expect("attribute is badly formatted");

        if attr_name.ident != "field_names_types_as_array" {
            continue;
        }

        let meta = attr
            .parse_meta()
            .expect("unable to parse attribute to meta");

        if let Meta::List(l) = meta {
            for arg in l.nested {
                res.push(A::parse(&arg));
            }
        }
    }

    res
}
