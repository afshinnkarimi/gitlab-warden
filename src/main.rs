mod config;

use config::Config;
use reqwest::Client;
use serde_json::Value;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let path = "config.yaml";
    let config = Config::from_file(path)?;

    let server_address = format!("https://{}/api/v4/personal_access_tokens", config.server);

    let client = Client::new();
    let res = client
        .get(&server_address)
        .header("PRIVATE-TOKEN", &config.token)
        .send()
        .await?;

    let status = res.status();
    let text = res.text().await?;

    if !status.is_success() {
        eprintln!("Request failed with status: {}", status);
        eprintln!("Response: {}", text);
        return Ok(());
    }

    let json: Value = serde_json::from_str(&text)?;
    println!("Parsed JSON: {:#}", json);

    Ok(())
}
