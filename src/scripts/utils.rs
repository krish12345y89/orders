use jsonwebtoken::{ Algorithm, EncodingKey, Header };
use reqwest::Client;
use serde::{ Deserialize, Serialize };
use serde_json::{ Value };
use std::error::Error;
use std::time::{ SystemTime, UNIX_EPOCH };

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    iss: String,
    scope: String,
    aud: String,
    exp: usize,
    iat: usize,
}
use std::sync::Mutex;
use once_cell::sync::Lazy;

use crate::schema::order::Order;

static TOKEN_CACHE: Lazy<Mutex<Option<(String, usize)>>> = Lazy::new(|| Mutex::new(None));

pub async fn generate_token(
    client_email: &str,
    private_key: &str
) -> Result<String, Box<dyn Error>> {
    let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as usize;

    let claims = Claims {
        iss: client_email.to_string(),
        scope: "https://www.googleapis.com/auth/spreadsheets".to_string(),
        aud: "https://oauth2.googleapis.com/token".to_string(),
        exp: now + 3600,
        iat: now,
    };

    let jwt = jsonwebtoken::encode(
        &Header::new(Algorithm::RS256),
        &claims,
        &EncodingKey::from_rsa_pem(private_key.as_bytes())?
    )?;

    let response = Client::new()
        .post("https://oauth2.googleapis.com/token")
        .form(
            &[
                ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
                ("assertion", &jwt),
            ]
        )
        .send().await?
        .json::<Value>().await?;

    response["access_token"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or_else(|| "Failed to get access token".into())
}

pub async fn get_or_generate_token(
    client_email: &str,
    private_key: &str
) -> Result<String, Box<dyn std::error::Error>> {
    let now = std::time::SystemTime
        ::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs() as usize;

    {
        let cache = TOKEN_CACHE.lock().unwrap();
        if let Some((token, exp)) = &*cache {
            if *exp > now + 60 {
                // 60 sec buffer before expiry
                return Ok(token.clone());
            }
        }
    }

    // Generate new token
    let token = generate_token(client_email, private_key).await?;

    // Save with expiry
    {
        let mut cache = TOKEN_CACHE.lock().unwrap();
        *cache = Some((token.clone(), now + 3600));
    }

    Ok(token)
}

impl Order {
    pub fn to_sheet1_row(&self) -> Vec<String> {
        vec![
            "".to_string(), // CHANNEL VLOOKUP
            "".to_string(), // #REF!
            "".to_string(), // SKU
            "".to_string(), // #REF!
            "".to_string(), // BIN RACK
            self.order_id.clone(),
            "".to_string(), // RETURN REASON
            "".to_string(), // REFUND YES
            self.date.clone(),
            "".to_string(), // REFUNDED checkbox
            "".to_string(), // STOCK ADDED
            "".to_string(), // Refund Date
            self.match_type.clone().unwrap_or_default(),
            "".to_string() // Fraser Classification
        ]
    }

    pub fn to_sheet2_row(&self) -> Vec<String> {
        vec![
            self.return_order.map(|v| v.to_string()).unwrap_or_default(),
            self.shopify_id.clone().unwrap_or_default(),
            self.marketplace.clone(),
            self.returned_sku.clone().unwrap_or_default(),
            self.offer_sku.clone().unwrap_or_default(),
            self.matched_sku.clone().unwrap_or_default(),
            self.match_type.clone().unwrap_or_default(),
            self.row_number.map(|v| v.to_string()).unwrap_or_default(),
            self.manual_confirmation.clone().unwrap_or_default(),
            self.status.clone().unwrap_or_default(),
            "".to_string(), // MARKETPLACE col duplicate
            self.qty.map(|v| v.to_string()).unwrap_or_default(),
            self.main_updated.clone().unwrap_or_default()
        ]
    }
}

impl Order {
    pub async fn from_sheets(sheet1_row: &[String], sheet2_row: Option<&[String]>) -> Option<Self> {
        let mut order = Order {
            id: uuid::Uuid::new_v4().to_string(),
            marketplace: "shopify".to_string(),
            order_id: sheet1_row.get(5).cloned().unwrap_or_default(),
            return_order: None,
            shopify_id: None,
            market_place_code: None,
            returned_sku: None,
            offer_sku: None,
            matched_sku: None,
            match_type: sheet1_row.get(12).cloned(),
            row_number: None,
            manual_confirmation: None,
            status: None,
            qty: None,
            main_updated: None,
            date: sheet1_row.get(8).cloned().unwrap_or_default(),
            created_at: chrono::Utc::now().to_rfc3339(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        };

        if let Some(row2) = sheet2_row {
            order.marketplace = row2.get(2).cloned().unwrap_or(order.marketplace);
            order.return_order = row2.get(0).and_then(|s| s.parse().ok());
            order.shopify_id = row2.get(1).cloned();
            order.returned_sku = row2.get(3).cloned();
            order.offer_sku = row2.get(4).cloned();
            order.matched_sku = row2.get(5).cloned();
            order.match_type = row2.get(6).cloned().or(order.match_type);
            order.row_number = row2.get(7).and_then(|s| s.parse().ok());
            order.manual_confirmation = row2.get(8).cloned();
            order.status = row2.get(9).cloned();
            order.qty = row2.get(11).and_then(|s| s.parse().ok());
            order.main_updated = row2.get(12).cloned();
        }

        Some(order)
    }
}
