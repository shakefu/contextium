//! memvid installer - Portable single-file memory system for AI agents
//! https://github.com/memvid/memvid

use super::{BootstrapContext, BootstrapError, ToolInstaller};
use crate::bootstrap::shell::{
    command_exists_with_path, run_command_in_dir, run_shell_command_with_path,
};
use async_trait::async_trait;
use colored::Colorize;

pub struct MemvidInstaller;

#[async_trait]
impl ToolInstaller for MemvidInstaller {
    fn check_command(&self) -> &'static str {
        "memvid"
    }

    async fn install(&self, ctx: &BootstrapContext) -> Result<bool, BootstrapError> {
        // Check if already installed
        if self.is_installed(ctx) {
            eprintln!("{} memvid already installed", "==>".green());
            return Ok(true);
        }

        eprintln!("{} Installing memvid...", "==>".green());

        let mut installed = false;

        // Install CLI via npm
        if command_exists_with_path("npm", &ctx.path_with_local) {
            eprintln!("{} Installing memvid-cli via npm...", "==>".green());
            let result =
                run_shell_command_with_path("npm install -g memvid-cli", &ctx.path_with_local);
            if let Ok(output) = result {
                if output.status.success() {
                    installed = true;
                } else {
                    eprintln!("{} memvid-cli npm install failed", "==>".yellow());
                }
            }
        } else {
            eprintln!("{} npm not found, skipping CLI install", "==>".yellow());
        }

        // Install Python SDK using uv
        if command_exists_with_path("uv", &ctx.path_with_local) {
            // Ensure venv exists
            let venv_dir = ctx.venv_dir();
            if !venv_dir.exists() {
                eprintln!("{} Creating .venv...", "==>".green());
                let result = run_command_in_dir("uv", &["venv", ".venv"], &ctx.repo_root);
                if let Err(e) = result {
                    eprintln!("{} Failed to create venv: {}", "==>".yellow(), e);
                }
            }

            eprintln!("{} Installing memvid-sdk via uv...", "==>".green());
            let result = run_command_in_dir(
                "uv",
                &["pip", "install", "--quiet", "--upgrade", "memvid-sdk"],
                &ctx.repo_root,
            );
            match result {
                Ok(output) if output.status.success() => {
                    installed = true;
                }
                Ok(output) => {
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    eprintln!("{} memvid-sdk install failed: {}", "==>".yellow(), stderr);
                }
                Err(e) => {
                    eprintln!("{} memvid-sdk install failed: {}", "==>".yellow(), e);
                }
            }
        } else {
            eprintln!(
                "{} uv not available, skipping Python SDK install",
                "==>".yellow()
            );
        }

        // Note about Rust crate
        if command_exists_with_path("cargo", &ctx.path_with_local) {
            eprintln!(
                "{} Note: For Rust integration, add to Cargo.toml: memvid-core = \"2.0\"",
                "==>".green()
            );
        }

        if installed {
            eprintln!("{} memvid installed successfully", "==>".green());
            Ok(false)
        } else {
            Err(BootstrapError::ToolNotFound(
                "memvid - npm or uv required".to_string(),
            ))
        }
    }
}
