use std::error::Error;
use reqwest::Client;
use serde_json::{json, Value};

pub async fn append_to_google_sheets(
    access_token: &str,
    spreadsheet_id: &str,
    range: &str,
    values: Vec<Vec<String>>, // Multiple rows
) -> Result<(), Box<dyn Error>> {
    let url = format!(
        "https://sheets.googleapis.com/v4/spreadsheets/{}/values/{}:append?valueInputOption=USER_ENTERED",
        spreadsheet_id, range
    );

    let body = json!({
        "values": values
    });

    let client = Client::new();
    let res = client
        .post(&url)
        .bearer_auth(access_token)
        .json(&body)
        .send()
        .await?;

    if res.status().is_success() {
        println!("✅ Rows appended successfully");
    } else {
        println!("❌ Error: {}", res.text().await?);
    }

    Ok(())
}

pub async fn update_order_in_sheets(
    access_token: &str,
    sheet_id: &str,
    sheet1_range: &str,
    sheet2_range: &str,
    sheet1_values: Vec<Vec<String>>,
    sheet2_values: Vec<Vec<String>>,
) -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    // Sheet1 update
    let url1 = format!(
        "https://sheets.googleapis.com/v4/spreadsheets/{}/values/{}?valueInputOption=USER_ENTERED",
        sheet_id, sheet1_range
    );

    client
        .put(&url1)
        .bearer_auth(access_token)
        .json(&json!({ "values": sheet1_values }))
        .send()
        .await?
        .error_for_status()?; // Agar error aaya to throw karega

    // Sheet2 update
    let url2 = format!(
        "https://sheets.googleapis.com/v4/spreadsheets/{}/values/{}?valueInputOption=USER_ENTERED",
        sheet_id, sheet2_range
    );

    client
        .put(&url2)
        .bearer_auth(access_token)
        .json(&json!({ "values": sheet2_values }))
        .send()
        .await?
        .error_for_status()?;

    println!("✅ Order updated successfully in both sheets");
    Ok(())
}

pub async fn delete_order_in_sheets(
    access_token: &str,
    sheet_id: &str,
    sheet1_range: &str,
    sheet2_range: &str,
) -> Result<(), Box<dyn Error>> {
    let client = Client::new();

    // Sheet1 delete (clear)
    let url1 = format!(
        "https://sheets.googleapis.com/v4/spreadsheets/{}/values/{}:clear",
        sheet_id, sheet1_range
    );

    client
        .post(&url1)
        .bearer_auth(access_token)
        .send()
        .await?
        .error_for_status()?;

    // Sheet2 delete (clear)
    let url2 = format!(
        "https://sheets.googleapis.com/v4/spreadsheets/{}/values/{}:clear",
        sheet_id, sheet2_range
    );

    client
        .post(&url2)
        .bearer_auth(access_token)
        .send()
        .await?
        .error_for_status()?;

    println!("🗑️ Order deleted from both sheets");
    Ok(())
}

pub async fn fetch_sheet_data(
    access_token: &str,
    spreadsheet_id: &str,
    sheet_name: &str
) -> Result<Value, Box<dyn Error>> {
    let url = format!(
        "https://sheets.googleapis.com/v4/spreadsheets/{}/values/{}",
        spreadsheet_id,
        sheet_name
    );

    let response = Client::new().get(&url).bearer_auth(access_token).send().await?;

    if !response.status().is_success() {
        return Err(format!("API request failed: {}", response.text().await?).into());
    }

    Ok(response.json::<Value>().await?)
}
