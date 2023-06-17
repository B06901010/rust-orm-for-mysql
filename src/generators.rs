use mysql::prelude::*;
use mysql::{Pool, PooledConn, Value, Row};
use std::error::Error;
use std::any::Any;
use my_macro_crate::SQLTable as SQLTable;
// use my_macro_crate::schema::{Schema, Field, SchemaProvider};
use std::collections::HashMap;
use quote::ToTokens;

use crate::schema::{Field, Schema, SchemaProvider};

struct DB {
    pub pool: Pool,
    pub conn: PooledConn,
}

pub struct SmallormEngine<> {
    db: DB,                         // connection pool
    table_name: String,             // table that the current query is operating on
    all_exec: Vec<mysql::Value>,    // all values of parameters for the current prepared statement
    prepare: String,                // prepared statement for the current query
    where_param: String,            // string consisting of all (WHERE ...) conditions
    where_exec: Vec<mysql::Value>,  // all values of parameters for the WHERE conditions
    update_param: String,           // string (UPDATE [table] SET ...)
    update_exec: Vec<mysql::Value>, // all values of parameters for the UPDATE operations
    select_param: String,
    select_exec: Vec<String>,
    group_param: String,
    // group_exec: Vec<String>,
}

impl SmallormEngine {
    pub fn new_mysql(username: &str, password: &str, address: &str, dbname: &str) -> Result<Self, Box<dyn Error>> {
        let url = format!("mysql://{}:{}@{}/{}", username, password, address, dbname);
        let p = Pool::new(url)?;
        let engine = SmallormEngine {
            db: DB {pool: p.clone(), conn: p.get_conn().unwrap()},
            table_name: String::new(),
            all_exec: Vec::new(),
            prepare: String::new(),
            where_param: String::new(),
            where_exec: Vec::new(),
            update_param: String::new(),
            update_exec: Vec::new(),
            select_param: String::new(),
            select_exec: Vec::new(),
            group_param: String::new(),
            // group_exec: Vec::new(),
        };
        Ok(engine)
    }

    pub fn table(&mut self, name: &str) -> &mut Self {
        self.table_name = String::from(name);
        self
    }

    pub fn where_not_in<T>(&mut self, field_name: &str, value_set: Vec<T>) -> &mut Self
    where
        T: mysql::prelude::ToValue + std::fmt::Display
    {
        if !self.where_param.is_empty() {
            self.where_param += " and (";
        } else {
            self.where_param += "(";
        }

        let placeholder = vec!["?"; value_set.len()];

        self.where_param += &format!("{} not in ({}) ", field_name, placeholder.join(",") + ") ");
        for i in 0..value_set.len() {
            self.where_exec.push(value_set[i].to_value());
        }

        // println!("where_exec = {:?}", self.where_exec);
        // println!("where_param = {:?}", self.where_param);
        self
    }

    pub fn where_in<T>(&mut self, field_name: &str, value_set: Vec<T>) -> &mut Self
    where
        T: mysql::prelude::ToValue + std::fmt::Display
    {
        if !self.where_param.is_empty() {
            self.where_param += " and (";
        } else {
            self.where_param += "(";
        }

        let placeholder = vec!["?"; value_set.len()];

        self.where_param += &format!("{} in ({}) ", field_name, placeholder.join(",") + ") ");
        for i in 0..value_set.len() {
            self.where_exec.push(value_set[i].to_value());
        }

        // println!("where_exec = {:?}", self.where_exec);
        // println!("where_param = {:?}", self.where_param);
        self
    }

    pub fn where_<T>(&mut self, field_name: &str, operator: &str, val: T) -> &mut Self
    where
        T: mysql::prelude::ToValue + std::fmt::Display
    {

        if !self.where_param.is_empty() {
            self.where_param += " and (";
        } else {
            self.where_param += "(";
        }

        self.where_param += &format!("{} {} ?) ", field_name, operator);
        self.where_exec.push(val.to_value());
        // println!("where_exec = {:?}", self.where_exec);
        // println!("where_param = {:?}", self.where_param);
        self
    }

    pub fn where_row<T>(&mut self, data: &T) -> &mut Self
    where
        T: SchemaProvider,
    {
        let sch = data.to_schema(); // Assuming you have a function to retrieve the schema information

        // self.all_exec = vec![None; sch.field_names.len()];
        let mut v = vec![None; sch.field_names.len()];
        let mut field_name_array = vec![String::new(); sch.field_names.len()];

        for (f_name, f) in &sch.field_map {
            let field_value = data.get_field_value(f_name).clone();
            // self.all_exec[f.i as usize] = field_value;
            v[f.i as usize] = field_value;
            // self.where_exec.push(field_value.clone());
        }

        for i in 0..sch.field_names.len() {
            field_name_array[i] = format!("{}=?", &sch.field_names[i]);
        }

        if !self.where_param.is_empty() {
            self.where_param += " and (";
        } else {
            self.where_param += "(";
        }

        self.where_param += &(field_name_array.join(" and ") + ") ");

        let v_value : Vec<mysql::Value> = v
            .iter()
            .enumerate()
            .map(|(i, item)| {
                match item {
                    Some(value) => {
                        // Convert the inner value to mysql::Value
                        let f_name = &sch.field_names[i];
                        let ty = &sch.field_map[&f_name as &str].ty;
                        if ty == "i32" {
                            value.downcast_ref::<i32>().to_value()
                        }
                        else if ty == "i64" {
                            value.downcast_ref::<i64>().to_value()
                        }
                        else if ty == "f32" {
                            value.downcast_ref::<f32>().to_value()
                        }
                        else if ty == "f64" {
                            value.downcast_ref::<f64>().to_value()
                        }
                        else if ty == "String" {
                            value.downcast_ref::<String>().to_value()
                        }
                        else {
                            mysql::Value::NULL
                        }
                    },
                    None => {
                        // Handle the case when the inner value is None
                        // For example, you can return a default value or handle it in an appropriate way
                        // Here, we return mysql::Value::NULL as an example
                        mysql::Value::NULL
                    }
                }
            }).collect();

        self.where_exec.extend(v_value);
        // println!("where_exec = {:?}", self.where_exec);
        // println!("where_param = {:?}", self.where_param);
        self
    }

    pub fn update<T>(&mut self, field_name: &str, value: T) -> Result<(), Box<dyn Error>>
    where
        T: mysql::prelude::ToValue + std::fmt::Display
    {
        // println!("self.all_exec: {:?}", self.all_exec);
        self.update_param += &format!("{} = ?", field_name);
        self.update_exec.push(value.to_value());
        self.all_exec.extend(self.update_exec.clone());

        self.prepare = "UPDATE ".to_owned() + self.get_table() + " SET " + &self.update_param;

        if !self.where_param.is_empty() {
            self.prepare += &(" WHERE ".to_owned() + &self.where_param.to_string());
            self.all_exec.extend(self.where_exec.clone());
        }

        // println!();
        // println!("Executing the following UPDATE query ...");

        // println!("{:?} ", &self.prepare);
        // println!("with input {:?} ", &self.all_exec);

        let stmt = self.db.conn.prep(&self.prepare)?;
        let result = self.db.conn.exec_drop(stmt, &self.all_exec)?;

        self.reset_query();

        Ok(result)
    }

    pub fn delete(&mut self) -> Result<(), Box<dyn Error>> {
        self.prepare = "DELETE FROM ".to_owned() + self.get_table();

        if !self.where_param.is_empty() {
            self.prepare += &(" WHERE ".to_owned() + &self.where_param.to_string());
        }

        // println!();
        // println!("Executing the following DELETE query ...");
        // println!("{:?} ", &self.prepare);
        // println!("with input {:?} ", &self.where_exec);

        self.all_exec = self.where_exec.clone();
        let stmt = self.db.conn.prep(&self.prepare)?;
        let result = self.db.conn.exec_drop(stmt, &self.all_exec)?;

        self.reset_query();

        Ok(result)
    }

    pub fn drop(&mut self) -> Result<(), Box<dyn Error>> {
        let result = self.db.conn.query_drop(format!("DROP TABLE {}", self.get_table()))?;
        self.reset_query();

        Ok(result)
    }

    pub fn create_table(&mut self, schema: Schema, table_name: &str) -> Result<(), Box<dyn Error>> {

        let mut field_array = vec![String::new(); schema.field_names.len()];

        let mut create_param = "CREATE TEMPORARY TABLE ".to_owned() + table_name + " (";
        for (f_name, f) in &schema.field_map {
            let mut type_string = String::new();
            if f.ty == "i32" {
                type_string = "INT".to_string();
            } else if f.ty == "String" {
                type_string = "VARCHAR(20)".to_string();
            }
            field_array[f.i as usize] = f_name.clone() + " " + &type_string;
        }
        create_param += &(field_array.join(",") + ")");

        // println!();
        // println!("Executing the following CREATE TABLE query ...");
        // println!("{}", create_param);

        let result = self.db.conn.query_drop(create_param)?;

        Ok(result)
    }

    pub fn insert<T>(&mut self, data: & T) -> Result<(), Box<dyn Error>>
    where
        T: SchemaProvider,
    {
        let sch = data.to_schema(); // Assuming you have a function to retrieve the schema information
        let mut v = vec![None; sch.field_names.len()];
        let placeholder = vec!["?"; sch.field_names.len()];

        for (f_name, f) in &sch.field_map {
            let field_value = data.get_field_value(f_name).clone();
            v[f.i as usize] = field_value;
        }

        // self.all_exec = v;
        // eprintln!("{:?}", self.all_exec);

        self.prepare = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            self.get_table(),
            sch.field_names.join(","),
            placeholder.join(",")
        );

        // println!("{:?}", self.prepare);
        let v_value : Vec<mysql::Value> = v
        .iter()
        .enumerate()
        .map(|(i, item)| {
            match item {
                Some(value) => {
                    // Convert the inner value to mysql::Value
                    // Assuming you have a conversion function `convert_to_mysql_value`
                    let f_name = &sch.field_names[i];
                    let ty = &sch.field_map[&f_name as &str].ty;
                    if ty == "i32" {
                        value.downcast_ref::<i32>().to_value()
                    }
                    else if ty == "i64" {
                        value.downcast_ref::<i64>().to_value()
                    }
                    else if ty == "f32" {
                        value.downcast_ref::<f32>().to_value()
                    }
                    else if ty == "f64" {
                        value.downcast_ref::<f64>().to_value()
                    }
                    else if ty == "String" {
                        value.downcast_ref::<String>().to_value()
                    }
                    else {
                        mysql::Value::NULL
                    }
                },
                None => {
                    // Handle the case when the inner value is None
                    // For example, you can return a default value or handle it in an appropriate way
                    // Here, we return mysql::Value::NULL as an example
                    mysql::Value::NULL
                }
            }
        })
        .collect();

        self.all_exec.extend(v_value);

        // println!();
        // println!("Executing the following INSERT query ...");

        // println!("{:?} ", &self.prepare);
        // println!("with input {:?} ", &self.all_exec);

        let stmt = self.db.conn.prep(&self.prepare)?;
        let result = self.db.conn.exec_drop(stmt, &self.all_exec)?;

        self.reset_query();

        Ok(result)
    }

    pub fn column(&mut self, f_names: Vec<&str>) -> &mut Self
    {
        self.select_exec = f_names.iter().map(|&s|s.into()).collect();
        self
    }

    pub fn select(&mut self) -> Result<Vec<HashMap<String, Value>>, Box<dyn Error>>
    {
        if self.select_exec.is_empty() {
            let keys = self.get_column_names();
            self.select_param = format!("SELECT * FROM {}", self.get_table());    
            self.select_exec = keys.clone();
        } else {
            self.select_param = format!("SELECT {} FROM {}", self.select_exec.join(","), self.get_table());
        }

        let result = self.exec_select_query()?;
        let mut results: Vec<HashMap<String, mysql::Value>> = Vec::new();

        for row in &result {
            
            let mut hashmap: HashMap<String, mysql::Value> = HashMap::new();

            for key in &self.select_exec {
                if let Some(value) = row.get::<mysql::Value, &str>(key) {
                    hashmap.insert(key.clone(), value.clone());
                }
            }
            // println!("{:?}", hashmap);
            results.push(hashmap);
        }

        self.reset_query();
        Ok(results)
    }

    pub fn max(&mut self, f_name: &str) -> Result<Value, Box<dyn Error>> {

        self.select_param = format!("SELECT max({}) FROM {}", f_name, self.get_table());
        let result = self.exec_select_query()?;
        self.reset_query();

        Ok(result[0].get(0).unwrap())
    }
    
    pub fn min(&mut self, f_name: &str) -> Result<Value, Box<dyn Error>> {

        self.select_param = format!("SELECT min({}) FROM {}", f_name, self.get_table());
        let result = self.exec_select_query()?;
        self.reset_query();

        Ok(result[0].get(0).unwrap())
    }

    pub fn avg(&mut self, f_name: &str) -> Result<Value, Box<dyn Error>> {

        self.select_param = format!("SELECT avg({}) FROM {}", f_name, self.get_table());
        let result = self.exec_select_query()?;
        self.reset_query();

        Ok(result[0].get(0).unwrap())
    }

    pub fn sum(&mut self, f_name: &str) -> Result<Value, Box<dyn Error>> {

        self.select_param = format!("SELECT sum({}) FROM {}", f_name, self.get_table());
        let result = self.exec_select_query()?;
        self.reset_query();

        Ok(result[0].get(0).unwrap())
    }

    pub fn count(&mut self) -> Result<Value, Box<dyn Error>> {
        self.select_param = format!("SELECT count(*) FROM {}", self.get_table());
        let result = self.exec_select_query()?;
        self.reset_query();

        Ok(result[0].get(0).unwrap())
    }

    pub fn group(&mut self, f_names: Vec<&str>) -> &mut Self {
        self.group_param = f_names.join(",");
        self
    }

    fn exec_select_query(&mut self) -> Result<Vec<mysql::Row>, Box<dyn Error>> {
        if !self.where_param.is_empty() {
            self.select_param += &format!(" WHERE {}", self.where_param);
            self.all_exec.extend(self.where_exec.clone());
        }

        if self.group_param != "" {
            self.select_param += &format!(" GROUP BY {}", self.group_param);
        }

        // println!();
        // println!("Executing the following SELECT query ...");

        // println!("{:?} ", &self.select_param);
        // println!("with input {:?} ", &self.all_exec);

        let stmt = self.db.conn.prep(&self.select_param)?;
        let result: Vec<mysql::Row> = self.db.conn.exec(stmt, &self.all_exec)?;

        Ok(result)
    }

    fn reset_query(&mut self) {
        self.where_exec = Vec::new();
        self.where_param = String::new();
        self.all_exec = Vec::new();
        self.select_exec = Vec::new();
        self.select_param = String::new();
        self.group_param = String::new();
        self.table_name = String::new();
    }

    fn get_table(&self) -> &str {
        &self.table_name
    }

    fn get_column_names(&mut self) -> Vec<String> {
        let describe: Vec<(String, String, Option<String>, Option<String>, Option<String>, Option<String>)> = self.db.conn.query(format!("describe {}", self.get_table())).unwrap();
        let column_names: Vec<String> = describe.iter().map(|(name, _, _, _, _, _)| name.clone()).collect();
        // println!("{:?}", column_names);
        column_names
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_query() -> Result<(), Box<dyn std::error::Error>> {
        // username, password, url, db_name
        let mut engine = SmallormEngine::new_mysql("wsl", "password", "Julia_vivo.local:3306", "practice")?;

        #[derive(Debug, SQLTable)]   // derive SQLTable so that custom functions are generated
        struct Payment {
            customer_id: i32, 
            amount: i32,
            month: i32,
            account_name: String,
        }

        // Create table according to the struct definition
        let schema = Payment::toSchema();
        let _ = engine.create_table(schema, "payment")?;

        // insert
        let p1 = Payment {customer_id: 1, amount: 2, month: 1, account_name: "hello".to_string()};
        let p2 = Payment {customer_id: 2, amount: 10, month: 1, account_name: "foo".to_string()};
        let p3 = Payment {customer_id: 3, amount: 100, month: 2, account_name: "hi".to_string()};
        let _ = engine.table("payment").insert(&p1);
        let _ = engine.table("payment").insert(&p2);
        let _ = engine.table("payment").insert(&p3);
        
        // select
        let rows = engine.table("payment")
            .where_("amount", ">=", 3)
            .column(vec!["customer_id", "account_name"])
            .select()?;

        println!("{:?}", rows);

        let rows2 = engine.table("payment")
            .where_("amount", ">=", 3)
            .select()?;

        println!("{:?}", rows);
        println!("{:?}", rows2);

        let m = engine.table("payment")
            .where_("amount", "<=", 10)
            .max("customer_id")?;

        println!("{:?}", m);

        let s = engine.table("payment")
            .where_("amount", "<=", 10)
            .sum("customer_id")?;

        println!("{:?}", s);

        let a = engine.table("payment")
            .where_("amount", "<=", 10)
            .avg("amount")?;

        println!("{:?}", a);

        let g = engine.table("payment")
            .column(vec!["month", "count(*)"])
            .group(vec!["month"])
            .select()?;

        let mut r = engine.table("payment").select()?;
        println!("{:?}", g);
        
        // delete
        engine.table("payment")
            .where_("customer_id", "!=", 2)
            .delete();
        r = engine.table("payment").select()?;
        println!("{:?}", r);

        // update
        engine.table("payment")
            .where_("customer_id", "=", 2)
            .update("customer_id", 5);
        r = engine.table("payment").select()?;
        println!("{:?}", r);

        // Clean up the temporary table if needed
        engine.table("payment").drop()?;
        Ok(())
    }
}
