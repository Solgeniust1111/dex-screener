use serde_json::Value;

pub async fn fetch_dex_data(mint: &str) -> Result<Value, String> {
    let url = format!("https://api.dexscreener.com/latest/dex/tokens/{}", mint);

    // Making the GET request
    let response = reqwest::get(&url).await.map_err(|e| e.to_string())?;

    // Check if the response status is successful
    if response.status().is_success() {
        // Try to parse JSON
        response.json::<Value>().await.map_err(|e| e.to_string())
    } else {
        Err(format!(
            "Failed to retrieve data: HTTP {}",
            response.status()
        ))
    }
}
