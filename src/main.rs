mod schema;
mod generators;

use mysql::*;
use mysql::prelude::*;
use serde::Serialize;
use serde_json::json;
use syn::{Item, parse_quote, ItemStruct, Fields, Data, DataStruct};
use quote::{quote, ToTokens};


#[derive(Debug, PartialEq, Eq)]
struct Payment {
    customer_id: i32,
    amount: i32,
    account_name: Option<String>,
}

// // impl Payment {
// //     fn from_row(row: &Row) -> Payment {
// //         Payment {
// //             customer_id: row.get("customer_id"),
// //             amount: row.get("amount"),
// //             account_name: row.get("account_name"),
// //         }
// //     }

// //     fn to_row(payment: &Payment) -> (i32, i32, Option<String>) {
// //         let customer_id = payment.customer_id;
// //         let amount = payment.amount;
// //         let account_name = payment.account_name;
        
// //         (customer_id, amount, account_name)
// //     }
// // }

fn main() -> std::result::Result<(), Box<dyn std::error::Error>> {
    
    #[derive(Serialize)]
    struct User {
        name: String,
        age: i32,
    }

    let url = "mysql://wsl:password@Julia_vivo.local:3306/practice";
    let pool = Pool::new(url)?;
    let mut conn = pool.get_conn()?;

    // create table
    conn.query_drop(
        r"create temporary table payment (
            customer_id int not null,
            amount int not null,
            account_name varchar(20)
        )")?;

    let payments = vec![
        Payment { customer_id: 1, amount: 2, account_name: None },
        Payment { customer_id: 3, amount: 4, account_name: Some("foo".into()) },
        Payment { customer_id: 5, amount: 6, account_name: None },
        Payment { customer_id: 7, amount: 8, account_name: None },
        Payment { customer_id: 9, amount: 10, account_name: Some("bar".into()) },
    ];

    // insert payments into the db
    conn.exec_batch(
        r"insert into payment (customer_id, amount, account_name)
        values (:customer_id, :amount, :account_name)",
        payments.iter().map(|p| params! {
            "customer_id" => p.customer_id,
            "amount" => p.amount,
            "account_name" => &p.account_name,
        })
    )?;

    // select query
    // query_map: text query, closure
    let selected_payments = conn.query_map(
        "SELECT customer_id, amount, account_name from payment",
        |(customer_id, amount, account_name)| {
            Payment { customer_id, amount, account_name }
        },
    )?;

    // Let's make sure, that `payments` equals to `selected_payments`.
    // Mysql gives no guaranties on order of returned rows without `ORDER BY`, so assume we are lucky.
    assert_eq!(payments, selected_payments);
    println!("Yay!");

    // update query
    conn.exec_drop(
        r"update payment set amount=:amount, account_name=:account_name where customer_id=1",
        params!{
            "amount" => 0,
            "account_name" => "hi",
        }
    )?;

    let all_payments = conn.query_map(
        "select * from payment",
        |(customer_id, amount, account_name)| {
            Payment { customer_id, amount, account_name }
        },
    )?;
    println!("{:#?}", all_payments);

    let p1 = Payment {
        customer_id: 1,
        amount: 20,
        account_name: Some("hi".into()),
    };

    let r: Vec<(String, String, Option<String>, Option<String>, Option<String>, Option<String>)> = conn.query("describe payment")?;
    println!("{:?}", r);

    // let sql_row = to_row(&p1);
    

    Ok(())
}
