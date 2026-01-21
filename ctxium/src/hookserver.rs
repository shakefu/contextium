use axum::{extract::State, http::StatusCode, routing::post, Json, Router};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Deserialize, Serialize, Clone)]
struct HookPayload {
    hook_name: String,
    timestamp: String,
    stdin_data: serde_json::Value,
}

struct AppState {
    log_path: PathBuf,
}

/// Run the hookserver
pub async fn run(port: u16, log_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Expand ~ in path
    let log_path = expand_tilde(log_file);

    // Ensure parent directory exists
    if let Some(parent) = log_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let state = Arc::new(Mutex::new(AppState { log_path }));

    let app = Router::new()
        .route("/hook", post(handle_hook))
        .with_state(state);

    let addr = format!("127.0.0.1:{}", port);
    println!(
        "{} Hookserver listening on {}",
        "[ctxium]".cyan(),
        addr.green()
    );
    println!(
        "{} Log file: {}",
        "[ctxium]".cyan(),
        log_file.yellow()
    );
    println!("{} Waiting for hook events...\n", "[ctxium]".cyan());

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn handle_hook(
    State(state): State<Arc<Mutex<AppState>>>,
    Json(payload): Json<HookPayload>,
) -> StatusCode {
    // Pretty-print to stdout
    print_hook(&payload);

    // Append to log file
    let state = state.lock().await;
    if let Err(e) = append_to_log(&state.log_path, &payload) {
        eprintln!("{} Failed to write log: {}", "[error]".red(), e);
    }

    StatusCode::OK
}

fn print_hook(payload: &HookPayload) {
    let separator = "─".repeat(60);

    println!("{}", separator.dimmed());
    println!(
        "{} {} {}",
        "[HOOK]".cyan().bold(),
        payload.hook_name.green().bold(),
        format!("@ {}", payload.timestamp).dimmed()
    );
    println!("{}", separator.dimmed());

    // Pretty-print the stdin data
    match &payload.stdin_data {
        serde_json::Value::String(s) if s.is_empty() => {
            println!("{}", "(empty stdin)".dimmed());
        }
        serde_json::Value::String(s) => {
            println!("{}", s);
        }
        other => {
            if let Ok(pretty) = serde_json::to_string_pretty(other) {
                println!("{}", pretty);
            } else {
                println!("{:?}", other);
            }
        }
    }

    println!();
}

fn append_to_log(path: &PathBuf, payload: &HookPayload) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)?;

    let json = serde_json::to_string(payload)?;
    writeln!(file, "{}", json)?;

    Ok(())
}

fn expand_tilde(path: &str) -> PathBuf {
    if path.starts_with("~/") {
        if let Some(home) = dirs_home() {
            return PathBuf::from(home).join(&path[2..]);
        }
    }
    PathBuf::from(path)
}

fn dirs_home() -> Option<String> {
    std::env::var("HOME").ok()
}
