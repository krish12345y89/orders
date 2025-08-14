#[allow(unused_imports)]
use std::error::Error;
use std::fs;
use serde_json::Value;

use crate::{
    lmdb::utils::DB,
    schema::order::Order,
    scripts::{ order::fetch_sheet_data, utils::get_or_generate_token },
};
#[allow(dead_code)]
pub trait DBOrder {
    async fn insert(&self, order: Order) -> Result<(), Box<dyn Error>>;
    async fn insert_all(&self) -> Result<(), Box<dyn Error>>;
    fn get_single(&self, id: String) -> Result<Option<Order>, Box<dyn Error>>;
    fn get(&self) -> Result<Option<Vec<Order>>, Box<dyn Error>>;
    fn put(&self, order: Order) -> Result<(), Box<dyn Error>>;
    fn delete(&self, id: String) -> Result<(), Box<dyn Error>>;
}
#[derive(serde::Deserialize)]
struct ServiceAccount {
    client_email: String,
    private_key: String,
}

impl DBOrder for DB {
    async fn insert(&self, order: Order) -> Result<(), Box<dyn Error>> {
        println!("Inserting order: {:?}", &order);
        let file_content = fs::read_to_string("./src/service_account.json")?;
        println!("Service Account JSON: {}", file_content.len());
        let sa: ServiceAccount = serde_json::from_str(&file_content)?;
        let access_token = get_or_generate_token(&sa.client_email, &sa.private_key).await.unwrap();
        let sheet1_value = fetch_sheet_data(
            &access_token,
            "16pzLDZosE9HIhrWRrxc8ZkWERhWf0LVnqx0SI4e_eas",
            "Sheet1"
        ).await.unwrap(); // Now `Value`
        // Fetch sheet2 row
        let sheet2_value = fetch_sheet_data(
            &access_token,
            "16pzLDZosE9HIhrWRrxc8ZkWERhWf0LVnqx0SI4e_eas",
            "Sheet2"
        ).await.unwrap();

        // Convert JSON Value → Vec<String>
        let sheet1_row: Vec<String> = sheet1_value["values"][0] // first row
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap_or_default().to_string())
            .collect();

        let sheet2_row: Vec<String> = sheet2_value["values"][0]
            .as_array()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap_or_default().to_string())
            .collect();
        println!("Sheet1 Row: {:?}", &sheet1_row);
        println!("Sheet2 Row: {:?}", &sheet2_row);
        // Call your function
        let i = 0; // for nothing
        let order: Order = Order::from_sheets(i, &sheet1_row, Some(&sheet2_row)).await.unwrap();
        println!("Order created from sheets: {:?}", &order);
        let mut txn = self.env.write_txn()?;
        self.order_db.put(&mut txn, &order.id, &order)?;
        self.order_db.put(&mut txn, &order.row_number.unwrap().to_string(), &order)?;
        txn.commit()?;
        Ok(())
    }

    async fn insert_all(&self) -> Result<(), Box<dyn Error>> {
        let file_content = fs::read_to_string("./src/service_account.json")?;
        let sa: ServiceAccount = serde_json::from_str(&file_content)?;
        let access_token = get_or_generate_token(&sa.client_email, &sa.private_key).await?;

        // Saara Sheet1 data lo
        let sheet1_value = fetch_sheet_data(
            &access_token,
            "16pzLDZosE9HIhrWRrxc8ZkWERhWf0LVnqx0SI4e_eas",
            "Sheet1!A:Z" // full range
        ).await?;

        // Saara Sheet2 data lo
        let sheet2_value = fetch_sheet_data(
            &access_token,
            "16pzLDZosE9HIhrWRrxc8ZkWERhWf0LVnqx0SI4e_eas",
            "Sheet2!A:Z"
        ).await?;

        // Vec<Vec<String>> me convert karo
        let sheet1_rows: Vec<Vec<String>> = sheet1_value["values"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .map(|row|
                row
                    .as_array()
                    .unwrap_or(&vec![])
                    .iter()
                    .map(|v| v.as_str().unwrap_or_default().to_string())
                    .collect()
            )
            .collect();

        let sheet2_rows: Vec<Vec<String>> = sheet2_value["values"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .map(|row|
                row
                    .as_array()
                    .unwrap_or(&vec![])
                    .iter()
                    .map(|v| v.as_str().unwrap_or_default().to_string())
                    .collect()
            )
            .collect();

        println!("Total Sheet1 Rows: {}", sheet1_rows.len());
        println!("Total Sheet2 Rows: {}", sheet2_rows.len());

        let mut txn = self.env.write_txn()?;

        // Row by row process karo (skip headers)
        for i in 1..sheet1_rows.len() {
            let sheet1_row = &sheet1_rows[i];
            let sheet2_row = sheet2_rows.get(i); // same index ka row2
            println!("Processing row {}: {:?}", i, sheet1_row);
            println!("Sheet2 Row: {:?}", sheet2_row);
            let order_opt = Order::from_sheets(
                i,
                sheet1_row,
                sheet2_row.map(|r| r.as_slice())
            ).await;
            println!("Order from row {}: {:?}", i, &order_opt);
            if let Some(order) = order_opt {
                let order_id = order.order_id.clone();
                let exists = self.order_db.get(&txn, &order_id)?;
                if exists.is_none() {
                    println!("Inserting new order: {}", order_id);
                    // convert row_number to string for key
                    let row_number = order.row_number.map(|v| v.to_string()).unwrap_or_default();
                    println!("Inserting order from row {}: {:?}", i, &order);
                    self.order_db.put(&mut txn, &order_id, &order)?;
                    self.order_db.put(&mut txn, &row_number, &order)?;
                } else {
                    println!("Order already exists, updating: {}", order_id);
                    self.order_db.put(&mut txn, &order_id, &order)?;
                }
            }
        }

        txn.commit()?;
        Ok(())
    }

    fn get_single(&self, id: String) -> Result<Option<Order>, Box<dyn Error>> {
        let txn = self.env.read_txn()?;
        if let Some(order) = self.order_db.get(&txn, &id)? {
            Ok(Some(order))
        } else {
            Ok(None)
        }
    }

    fn get(&self) -> Result<Option<Vec<Order>>, Box<dyn Error>> {
        let txn = self.env.read_txn()?;
        let mut orders = Vec::new();
        for result in self.order_db.iter(&txn)? {
            let (_, order) = result?;
            orders.push(order);
        }
        if orders.is_empty() {
            Ok(None)
        } else {
            Ok(Some(orders.clone()))
        }
    }

    fn put(&self, order: Order) -> Result<(), Box<dyn Error>> {
        let mut txn = self.env.write_txn()?;
        let row_number = order.row_number.map(|v| v.to_string()).unwrap_or_default();
        self.order_db.put(&mut txn, &order.order_id, &order)?;
        self.order_db.put(&mut txn, &row_number, &order)?;
        txn.commit()?;
        Ok(())
    }

    fn delete(&self, id: String) -> Result<(), Box<dyn Error>> {
        let mut txn = self.env.write_txn()?;
        self.order_db.delete(&mut txn, &id)?;
        txn.commit()?;
        Ok(())
    }
}
