# Minimal Rust ORM for Mysql
## Installation
Create a new cargo package:
```
cargo new my_proj
cd my_proj
```
Clone this repository inside your Rust project. The structure of your project should look like this:
```
my_proj
├── Cargo.lock
├── Cargo.toml
├── rust_to_mysql (this repo)
├── src
└── target
```
Add this repository to your project dependencies. Open Cargo.toml and add the following line:
```
[dependencies]
rust_to_mysql = { path = "rust_to_mysql" }
```
## Usage
When writing your rust code, keep in mind the following points:
1. Import the necessary libraries:
  ```rust
  use rust_to_mysql::generators::SmallormEngine;
  use rust_to_mysql::my_macro_crate::SQLTable;
  use rust_to_mysql::schema::{Schema, Field, SchemaProvider};
  use std::collections::HashMap;
  ```
2. For each struct that corresponds to a table in MySQL, derive the SQLTable macro. For example:
  ```rust
  #[derive(SQLTable)]
  struct Customer {
      id: i32,
      name: String,
      age: i32,
  }
  ```
3. The return type of a select query is a vector of hashmaps. A hashmap correspond to one of the rows returned by the query.
4. To run the program, use the following command:
```
cargo run
```
Here's an example usage of rust_to_mysql to interact with your MySQL databases. Open src/main.rs and add the following code:
```rust
use rust_to_mysql::generators::SmallormEngine;
use rust_to_mysql::my_macro_crate::SQLTable;
use rust_to_mysql::schema::{Schema, Field, SchemaProvider};
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize the ORM engine
    let mut engine = SmallormEngine::new_mysql("root", "password", "localhost:3306", "practice")?;

    // Define the Payment and Customer struct using the SQLTable procedure macro
    #[derive(SQLTable)]
    struct Payment {
        id: i32,
        customer_id: i32,
        amount: i32,
    }

    #[derive(SQLTable)]
    struct Customer {
        id: i32,
        name: String,
        age: i32,
    }

    // Create table according to the struct definition
    // Usage: engine.create_table(schema, table_name)
    let _ = engine.create_table(Payment::toSchema(), "payment")?;
    let _ = engine.create_table(Customer::toSchema(), "customer")?;

    let p1 = Payment { id: 1, customer_id: 15, amount: 25 };
    let p2 = Payment { id: 2, customer_id: 11, amount: 10 };
    let p3 = Payment { id: 3, customer_id: 303, amount: 100 };
    let p4 = Payment { id: 4, customer_id: 11, amount: 200 };

    let c1 = Customer { id: 15, name: String::from("bob"), age: 18 };
    let c2 = Customer { id: 11, name: String::from("jack"), age: 20 };
    let c3 = Customer { id: 303, name: String::from("amy"), age: 40 };

    // Insert records into the "payment" and "customer" tables
    // Usage: engine.table(table_name).insert(struct_instance)
    let _ = engine.table("payment").insert(&p1);
    let _ = engine.table("payment").insert(&p2);
    let _ = engine.table("payment").insert(&p3);
    let _ = engine.table("payment").insert(&p4);

    let _ = engine.table("customer").insert(&c1);
    let _ = engine.table("customer").insert(&c2);
    let _ = engine.table("customer").insert(&c3);

    // Perform various operations on the "payment" table
    let rows = engine.table("payment")
        .where_("amount", ">=", 100)
        .column(vec!["customer_id", "amount"])
        .select()?;
    println!("{:?}", rows);

    let a = engine.table("payment")
        .where_("amount", "<=", 10)
        .avg("amount")?;
    println!("{:?}", a);

    let g = engine.table("payment")
        .column(vec!["customer_id", "sum(amount)"])
        .group(vec!["customer_id"])
        .select()?;
    println!("{:?}", g);

    let _ = engine.table("payment")
        .where_("customer_id", "!=", 303)
        .delete();

    let s = engine.table("payment").select()?;
    println!("{:?}", s);

    let _ = engine.table("payment")
        .where_("customer_id", "=", 303)
        .update("customer_id", 5);

    // Perform various operations on the "customer" table
    let rows2 = engine.table("customer")
        .select()?;
    println!("{:?}", rows2);
    
    // Drop table "payment" and "customer"
    let _ = engine.table("payment").drop()?;
    let _ = engine.table("customer").drop()?;

    Ok(())

}
```
Feel free to modify the code according to your specific use case. Run ```cargo run``` to execute the program and interact with your MySQL databases using the ORM functionalities provided by rust_to_mysql. Make sure to adjust the MySQL connection details (```new_mysql``` arguments) and table structures according to your requirements.
