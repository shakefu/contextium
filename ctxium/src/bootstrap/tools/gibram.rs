//! gibram installer - In-memory knowledge graph for RAG
//! https://github.com/gibram-io/gibram

use super::{BootstrapContext, BootstrapError, ToolInstaller};
use crate::bootstrap::codesign::prepare_binary;
use crate::bootstrap::download::{download_and_extract, find_binary};
use crate::bootstrap::shell::ensure_dir;
use async_trait::async_trait;
use colored::Colorize;

const GIBRAM_VERSION: &str = "v0.1.0";

pub struct GibramInstaller;

impl GibramInstaller {
    fn download_url(&self, ctx: &BootstrapContext) -> String {
        let platform = ctx.platform.as_download_suffix();
        format!(
            "https://github.com/gibram-io/gibram/releases/download/{}/gibram-{}.tar.gz",
            GIBRAM_VERSION, platform
        )
    }
}

#[async_trait]
impl ToolInstaller for GibramInstaller {
    fn check_command(&self) -> &'static str {
        "gibram-server"
    }

    async fn install(&self, ctx: &BootstrapContext) -> Result<bool, BootstrapError> {
        // Check if already installed
        if self.is_installed(ctx) {
            eprintln!("{} gibram-server already installed", "==>".green());
            return Ok(true);
        }

        eprintln!("{} Installing gibram...", "==>".green());

        // Ensure ~/.local/bin exists
        ensure_dir(&ctx.local_bin)?;

        // Download and extract
        let url = self.download_url(ctx);
        eprintln!("{} Downloading gibram {}...", "==>".green(), GIBRAM_VERSION);

        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .build()
            .map_err(|e| BootstrapError::HttpError(e.to_string()))?;

        let temp_dir = download_and_extract(&client, &url).await?;

        // Find the binary in the extracted archive
        let binary = find_binary(temp_dir.path(), "gibram-server")
            .or_else(|| find_binary(temp_dir.path(), "gibram"))
            .ok_or_else(|| {
                BootstrapError::ToolNotFound(
                    "gibram-server binary not found in archive".to_string(),
                )
            })?;

        // Copy to ~/.local/bin
        let dest = ctx.local_bin.join("gibram-server");
        std::fs::copy(&binary, &dest).map_err(BootstrapError::IoError)?;

        // Make executable and sign on macOS
        prepare_binary(&dest)?;

        eprintln!(
            "{} Installed gibram-server to {}",
            "==>".green(),
            dest.display()
        );

        // Verify installation
        if self.is_installed(ctx) {
            Ok(false)
        } else {
            eprintln!(
                "{} gibram installed but not in PATH for this session",
                "==>".yellow()
            );
            eprintln!(
                "{} Add to your shell profile: export PATH=\"{}:$PATH\"",
                "==>".yellow(),
                ctx.local_bin.display()
            );
            Ok(false)
        }
    }
}
