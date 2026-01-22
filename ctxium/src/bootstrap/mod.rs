//! Bootstrap module for installing Contextium tools
//!
//! Handles automatic installation of required tools when ctxium hook SessionStart is called.

pub mod codesign;
pub mod download;
pub mod platform;
pub mod shell;
pub mod tools;

use colored::Colorize;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

pub use platform::Platform;
pub use shell::BootstrapContext;
use tools::ToolInstaller;

/// Error types for bootstrap operations
#[derive(Debug)]
pub enum BootstrapError {
    /// Platform not supported
    UnsupportedPlatform(String),
    /// Command execution failed
    CommandFailed { command: String, message: String },
    /// Download failed
    DownloadFailed { url: String, message: String },
    /// Archive extraction failed
    ExtractionFailed(String),
    /// File operation failed
    IoError(std::io::Error),
    /// HTTP error
    HttpError(String),
    /// Tool not found after installation
    ToolNotFound(String),
}

impl std::fmt::Display for BootstrapError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedPlatform(msg) => write!(f, "Unsupported platform: {}", msg),
            Self::CommandFailed { command, message } => {
                write!(f, "Command '{}' failed: {}", command, message)
            }
            Self::DownloadFailed { url, message } => {
                write!(f, "Download from '{}' failed: {}", url, message)
            }
            Self::ExtractionFailed(msg) => write!(f, "Archive extraction failed: {}", msg),
            Self::IoError(e) => write!(f, "IO error: {}", e),
            Self::HttpError(msg) => write!(f, "HTTP error: {}", msg),
            Self::ToolNotFound(tool) => write!(f, "Tool '{}' not found after installation", tool),
        }
    }
}

impl std::error::Error for BootstrapError {}

impl From<std::io::Error> for BootstrapError {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}

/// Result of a tool installation
struct ToolResult {
    success: bool,
    already_installed: bool,
}

/// Summary of the bootstrap process
pub struct BootstrapResult {
    results: Vec<ToolResult>,
}

impl BootstrapResult {
    pub fn success_count(&self) -> usize {
        self.results.iter().filter(|r| r.success).count()
    }

    pub fn failure_count(&self) -> usize {
        self.results.iter().filter(|r| !r.success).count()
    }

    pub fn already_installed_count(&self) -> usize {
        self.results.iter().filter(|r| r.already_installed).count()
    }
}

/// Log bootstrap output to file
fn log_to_file(log_path: &Path, message: &str) {
    if let Ok(mut file) = OpenOptions::new().create(true).append(true).open(log_path) {
        let timestamp = chrono::Utc::now().format("%Y-%m-%d %H:%M:%S UTC");
        let _ = writeln!(file, "[{}] {}", timestamp, message);
    }
}

/// Run the bootstrap process
///
/// Installs all required tools for Contextium. This function:
/// 1. Detects the platform (OS/arch)
/// 2. Installs uv first (required for Python packages)
/// 3. Installs remaining tools in parallel
/// 4. Logs results to /tmp/contextium-bootstrap.log
///
/// Errors are logged but never cause the function to fail - bootstrap
/// failures should not block the hook from completing.
pub async fn run_bootstrap(repo_root: &Path) -> BootstrapResult {
    let log_path = Path::new("/tmp/contextium-bootstrap.log");

    // Ensure log directory exists
    if let Some(parent) = log_path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }

    log_to_file(log_path, "=== Bootstrap started ===");

    // Detect platform
    let platform = match Platform::detect() {
        Ok(p) => {
            log_to_file(log_path, &format!("Platform: {}", p));
            p
        }
        Err(e) => {
            let msg = format!("Platform detection failed: {}", e);
            log_to_file(log_path, &msg);
            eprintln!("{} {}", "==>".green(), msg);
            return BootstrapResult { results: vec![] };
        }
    };

    // Create bootstrap context
    let ctx = BootstrapContext::new(repo_root.to_path_buf(), platform.clone());

    let mut results = Vec::new();

    // Phase 1: Install uv first (required for Python packages)
    log_to_file(log_path, "Installing uv (required for Python packages)...");
    let uv_result = tools::uv::UvInstaller.install(&ctx).await;
    let uv_success = uv_result.is_ok();
    results.push(match uv_result {
        Ok(already) => {
            log_to_file(
                log_path,
                if already {
                    "uv already installed"
                } else {
                    "uv installed successfully"
                },
            );
            ToolResult {
                success: true,
                already_installed: already,
            }
        }
        Err(e) => {
            log_to_file(log_path, &format!("uv installation failed: {}", e));
            ToolResult {
                success: false,
                already_installed: false,
            }
        }
    });

    // Phase 2: Install remaining tools in parallel
    // Only proceed with Python-dependent tools if uv installed successfully
    log_to_file(log_path, "Installing remaining tools...");

    let tool_futures = vec![
        ("gibram", tools::gibram::GibramInstaller.install(&ctx)),
        (
            "worktrunk",
            tools::worktrunk::WorktrunkInstaller.install(&ctx),
        ),
    ];

    // Add Python-dependent tools only if uv is available
    let mut all_futures: Vec<(&str, _)> = tool_futures;
    if uv_success {
        all_futures.push(("memvid", tools::memvid::MemvidInstaller.install(&ctx)));
        all_futures.push((
            "adversarial-spec",
            tools::adversarial_spec::AdversarialSpecInstaller.install(&ctx),
        ));
    } else {
        log_to_file(
            log_path,
            "Skipping Python-dependent tools (uv not available)",
        );
        results.push(ToolResult {
            success: false,
            already_installed: false,
        });
        results.push(ToolResult {
            success: false,
            already_installed: false,
        });
    }

    // Execute all tool installations concurrently
    let tool_results = futures::future::join_all(
        all_futures
            .into_iter()
            .map(|(name, fut)| async move { (name, fut.await) }),
    )
    .await;

    for (name, result) in tool_results {
        results.push(match result {
            Ok(already) => {
                log_to_file(
                    log_path,
                    &format!(
                        "{} {}",
                        name,
                        if already {
                            "already installed"
                        } else {
                            "installed successfully"
                        }
                    ),
                );
                ToolResult {
                    success: true,
                    already_installed: already,
                }
            }
            Err(e) => {
                log_to_file(log_path, &format!("{} installation failed: {}", name, e));
                ToolResult {
                    success: false,
                    already_installed: false,
                }
            }
        });
    }

    // Log summary
    let summary = BootstrapResult { results };

    log_to_file(
        log_path,
        &format!(
            "=== Bootstrap complete: {} succeeded, {} failed, {} already installed ===",
            summary.success_count(),
            summary.failure_count(),
            summary.already_installed_count()
        ),
    );

    // Print summary to stderr (visible in hook output)
    eprintln!(
        "{} Bootstrap: {} tools ready ({} installed, {} skipped)",
        "==>".green(),
        summary.success_count(),
        summary.success_count() - summary.already_installed_count(),
        summary.already_installed_count()
    );

    summary
}
