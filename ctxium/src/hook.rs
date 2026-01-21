use serde::Serialize;
use std::io::{self, Read};

const DEFAULT_SERVER: &str = "http://127.0.0.1:6160";

#[derive(Serialize)]
struct HookPayload {
    hook_name: String,
    timestamp: String,
    stdin_data: serde_json::Value,
}

/// Run the hook command - read stdin and send to hookserver
/// Returns Ok(()) on success, Err on failure (but caller ignores errors)
pub async fn run(hook_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Read all of stdin
    let mut stdin_raw = String::new();
    io::stdin().read_to_string(&mut stdin_raw)?;

    // Try to parse as JSON, fall back to string value
    let stdin_data: serde_json::Value = serde_json::from_str(&stdin_raw)
        .unwrap_or_else(|_| serde_json::Value::String(stdin_raw.clone()));

    // Build payload
    let payload = HookPayload {
        hook_name: hook_name.to_string(),
        timestamp: chrono::Utc::now().to_rfc3339(),
        stdin_data,
    };

    // Send to server with short timeout
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(2))
        .build()?;

    client
        .post(format!("{}/hook", DEFAULT_SERVER))
        .json(&payload)
        .send()
        .await?;

    Ok(())
}
