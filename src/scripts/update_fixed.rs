use actix_web::web;
use reqwest;
use serde::{ Deserialize, Serialize };
use std::fs;
use chrono::{ FixedOffset, TimeZone };
use dotenv::dotenv;
use std::env;
use serde_json::{ json };
use log;

use crate::{lmdb::{order::DBOrder, utils::DB}, schema::order_api::{ Orders}};

const BASE_URL: &str = "https://eu-ext.linnworks.net";

#[derive(Debug, Serialize, Deserialize)]
struct AuthResponse {
    Token: String,
    Server: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]

struct OrderItem {
    SKU: String,
    Quantity: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct GeneralInfo {
    ReferenceNum: String,
    SubSource: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct MarketplaceData {
    linnwork_id: String,
    marketplace: String,
    marketplace_id: String,
    shopify_id: String,
    items: Vec<MarketplaceItem>,
}

#[derive(Debug, Serialize, Deserialize)]
struct MarketplaceItem {
    sku: String,
    quantity: i32,
}

async fn authorize() -> Result<AuthResponse, Box<dyn std::error::Error>> {
    // dotenv().ok();

    let client = reqwest::Client::new();
    let res = client
        .post("https://api.linnworks.net/api/Auth/AuthorizeByApplication")
        .json(
            &json!({
            // "ApplicationId":"f4fba1a1-36d5-472b-b903-7a3c1f1fe1e0",
            // "ApplicationSecret":"4e55d5ce-9cc5-45d5-a563-e8a9800c781f",
            // "Token":"4f5b1e4f127d1bacde9468078589abe9",
        })
        )
        .send().await?;

    let auth_response: AuthResponse = res.json().await?;
    Ok(auth_response)
}

fn get_utc_date_time(dt: &str) -> String {
    let parts: Vec<&str> = dt.split(' ').collect();
    if parts.len() != 2 {
        return String::new();
    }

    let date_str = parts[0];
    let time_str = parts[1];

    let naive_datetime = chrono::NaiveDateTime
        ::parse_from_str(&format!("{} {}", date_str, time_str), "%Y-%m-%d %H:%M")
        .unwrap();

    let london_offset = FixedOffset::east(0); // GMT in winter
    let london_time = london_offset.from_local_datetime(&naive_datetime).unwrap();

    london_time.to_rfc3339()
}

async fn get_num_order(id: &str, token: &str) -> Result<Orders, Box<dyn std::error::Error>> {
    println!("Fetching order details for ID: {}", id);
    let client = reqwest::Client::new();
    let url = format!("{}/api/Orders/GetOrderDetailsByNumOrderId?OrderId={}", BASE_URL, id);

    let res = client.get(&url).header("Authorization", token).send().await?;
    println!("Response status: {}", res.status());
    
    // Add debugging to see the raw response
    let text = res.text().await?;
    println!("Raw response: {}", text);
    
    let order:Orders  = serde_json::from_str(&text)?;
    println!("Fetched order: {:?}", order);
    Ok(order)
}

fn get_debenhams_data(order: &Orders) -> Option<MarketplaceData> {
    let mut is_debenham = false;
    let mut deb_id = String::new();

    for note in &order.notes {
        if note.note.contains("DUX") {
            deb_id = note.note.split("Marketplace Order ID -").nth(1)?.trim().to_string();
            is_debenham = true;
            break;
        }
    }

    if !is_debenham {
        return None;
    }

    let items: Vec<MarketplaceItem> = order.items
        .iter()
        .map(|item| MarketplaceItem {
            sku: item.sku.to_lowercase(),
            quantity: item.quantity,
        })
        .collect();

    let shopify_id = if order.general_info.reference_num.trim().is_empty() {
        "000".to_string()
    } else {
        order.general_info.reference_num.trim().to_string()
    };

    Some(MarketplaceData {
        linnwork_id: order.num_order_id.clone().to_string(),
        marketplace: "Debenhams".to_string(),
        marketplace_id: deb_id,
        shopify_id,
        items,
    })
}

fn get_secret_sales_data(order: &Orders) -> Option<MarketplaceData> {
    let mut is_secret_sales = false;
    let mut ss_id = String::new();

    for note in &order.notes {
        if note.note.contains("Marketplace Order ID -") {
            ss_id = note.note.split("Marketplace Order ID -").nth(1)?.trim().to_string();
            is_secret_sales = true;
            break;
        }
    }

    if !is_secret_sales {
        return None;
    }

    let items: Vec<MarketplaceItem> = order.items
        .iter()
        .map(|item| MarketplaceItem {
            sku: item.sku.to_lowercase(),
            quantity: item.quantity,
        })
        .collect();

    let shopify_id = if order.general_info.reference_num.trim().is_empty() {
        "000".to_string()
    } else {
        order.general_info.reference_num.trim().to_string()
    };

    Some(MarketplaceData {
        linnwork_id: order.num_order_id.clone().to_string(),
        marketplace: "Secret Sales".to_string(),
        marketplace_id: ss_id,
        shopify_id,
        items,
    })
}

fn get_matalan_data(order: &Orders) -> Option<MarketplaceData> {
    let is_matalan = order.general_info.sub_source.trim().to_lowercase() == "mirakl matalan";

    if !is_matalan {
        return None;
    }

    let mat_id = order.general_info.reference_num.clone();

    let items: Vec<MarketplaceItem> = order.items
        .iter()
        .map(|item| MarketplaceItem {
            sku: item.sku.to_lowercase(),
            quantity: item.quantity,
        })
        .collect();

    Some(MarketplaceData {
        linnwork_id: order.num_order_id.clone().to_string(),
        marketplace: "Matalan".to_string(),
        marketplace_id: mat_id,
        shopify_id: "000".to_string(),
        items,
    })
}


pub async fn update(db:web::Data<DB>,order_id: &str, row_number: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("Updating order: {}", order_id);
    let auth: AuthResponse = authorize().await?;
    println!("Authorization successful, token: {}", auth.Token);
    let order = get_num_order(order_id, &auth.Token).await?;
    println!("Fetched order: {:?}", order);
    fn update_if_match(
        db: web::Data<DB>,
        row_number: &str,
        data: Option<MarketplaceData>,
        order_id:String
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let mut breaks: bool = false;
        if let Some(data) = data {
            if let Some(mut db_order) = db.get_single(order_id.to_string())? {
                println!("Found order in database: {}", row_number);
                println!("returned_sku: {:?} and sku: {:?}", db_order.returned_sku, data.items[0].sku);
                if db_order.returned_sku == Some(data.items[0].sku.clone()) {
                    println!("Order matched in database with same SKU: {}", data.items[0].sku);
                    db_order.marketplace = data.marketplace.clone();
                    db_order.market_place_code = Some(data.marketplace_id.clone());
                    db_order.shopify_id = Some(data.shopify_id.clone());
                    db_order.returned_sku = Some(data.items[0].sku.clone());
                    db_order.match_type = Some("Full Match".to_string());
                    println!("Successfully updated order in database: {}", row_number);
                    println!("Order details: {:?}", db_order);
                    db.put(db_order.clone())?;
                    breaks = true;
                } else {
                    db_order.match_type = Some("None".to_string());
                }
                db.put(db_order)?;
            } else {
                println!("Order not found in database: {}", row_number);
                breaks = true;
            }
        }
        Ok(breaks)
    }
    let ans = update_if_match(db.clone(), &row_number, get_debenhams_data(&order),order_id.to_string())?;
    if ans {
        return Ok(());
    }
    let ans = update_if_match(db.clone(), &row_number, get_secret_sales_data(&order),order_id.to_string())?;
    if ans {
        return Ok(());
    }
    let ans = update_if_match(db, &row_number, get_matalan_data(&order),order_id.to_string())?;
    if ans {
        return Ok(());
    }

    Ok(())
}
