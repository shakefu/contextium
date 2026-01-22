use crate::bootstrap;
use serde::Serialize;
use std::io::{self, Read};
use std::time::Duration;

const DEFAULT_SERVER: &str = "http://127.0.0.1:6160";

#[derive(Serialize)]
struct HookPayload {
    hook_name: String,
    timestamp: String,
    stdin_data: serde_json::Value,
}

/// Read stdin with a timeout to avoid blocking forever
fn read_stdin_with_timeout() -> String {
    // Use a thread to read stdin so we can timeout
    let (tx, rx) = std::sync::mpsc::channel();

    std::thread::spawn(move || {
        let mut buffer = String::new();
        let _ = io::stdin().read_to_string(&mut buffer);
        let _ = tx.send(buffer);
    });

    // Wait up to 100ms for stdin data
    rx.recv_timeout(Duration::from_millis(100))
        .unwrap_or_default()
}

/// Run the hook command - read stdin and send to hookserver
/// Returns Ok(()) on success, Err on failure (but caller ignores errors)
pub async fn run(hook_name: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Log that hook was called (for debugging)
    eprintln!("[ctxium] Hook called: {}", hook_name);

    // Run bootstrap on SessionStart
    if hook_name == "SessionStart" {
        let repo_root = std::env::current_dir().unwrap_or_default();
        // Bootstrap errors are logged but never fail the hook
        let _ = bootstrap::run_bootstrap(&repo_root).await;
    }

    // Read stdin with timeout (non-blocking)
    let stdin_raw = read_stdin_with_timeout();

    // Try to parse as JSON, fall back to string value (or null if empty)
    let stdin_data: serde_json::Value = if stdin_raw.is_empty() {
        serde_json::Value::Null
    } else {
        serde_json::from_str(&stdin_raw)
            .unwrap_or_else(|_| serde_json::Value::String(stdin_raw.clone()))
    };

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
