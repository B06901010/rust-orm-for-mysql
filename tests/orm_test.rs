#[cfg(test)]
mod tests {
    use rust_to_mysql::generators::SmallormEngine;
    use rust_to_mysql::my_macro_crate::SQLTable;
    use rust_to_mysql::schema::{Schema, Field, SchemaProvider};
    use std::collections::HashMap;

    #[test]
    fn test_orm() -> Result<(), Box<dyn std::error::Error>> {
        // Initialize the ORM engine
        let mut engine = SmallormEngine::new_mysql("wsl", "password", "Julia_vivo.local:3306", "practice")?;

        // Define the Payment struct using the SQLTable procedure macro
        #[derive(Debug, SQLTable)]
        struct Payment {
            id: i32,
            customer_id: i32,
            amount: i32,
        }

        #[derive(Debug, SQLTable)]
        struct Customer {
            id: i32,
            name: String,
            age: i32,
        }

        // Create table according to the struct definition
        let _ = engine.create_table(Payment::toSchema(), "payment")?;
        let _ = engine.create_table(Customer::toSchema(), "customer")?;

        // Insert records into the "payment" table
        let p1 = Payment { id: 1, customer_id: 15, amount: 25 };
        let p2 = Payment { id: 2, customer_id: 11, amount: 10 };
        let p3 = Payment { id: 3, customer_id: 303, amount: 100 };
        let p4 = Payment { id: 4, customer_id: 11, amount: 200 };

        let c1 = Customer { id: 15, name: String::from("bob"), age: 18 };
        let c2 = Customer { id: 11, name: String::from("jack"), age: 20 };
        let c3 = Customer { id: 303, name: String::from("amy"), age: 40 };


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

        Ok(())
    }
}
