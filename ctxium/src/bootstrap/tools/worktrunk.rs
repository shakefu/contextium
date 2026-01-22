//! worktrunk installer - Git worktree management for parallel AI agents
//! https://github.com/max-sixty/worktrunk

use super::{BootstrapContext, BootstrapError, ToolInstaller};
use crate::bootstrap::shell::{command_exists_with_path, run_shell_command_with_path};
use async_trait::async_trait;
use colored::Colorize;

pub struct WorktrunkInstaller;

#[async_trait]
impl ToolInstaller for WorktrunkInstaller {
    fn check_command(&self) -> &'static str {
        "wt"
    }

    async fn install(&self, ctx: &BootstrapContext) -> Result<bool, BootstrapError> {
        // Check if already installed
        if self.is_installed(ctx) {
            eprintln!("{} worktrunk already installed", "==>".green());
            return Ok(true);
        }

        eprintln!("{} Installing worktrunk...", "==>".green());

        // Determine installation method based on platform
        if ctx.platform.is_macos() || ctx.platform.is_linux() {
            // Try Homebrew first
            if command_exists_with_path("brew", &ctx.path_with_local) {
                eprintln!("{} Installing via Homebrew...", "==>".green());
                let result =
                    run_shell_command_with_path("brew install worktrunk", &ctx.path_with_local);
                if let Ok(output) = result {
                    if output.status.success() {
                        // Configure shell integration
                        let _ = run_shell_command_with_path(
                            "wt config shell install",
                            &ctx.path_with_local,
                        );
                        eprintln!("{} worktrunk installed via Homebrew", "==>".green());
                        return Ok(false);
                    }
                }
            }

            // Fallback: try cargo on Linux
            if ctx.platform.is_linux() && command_exists_with_path("cargo", &ctx.path_with_local) {
                eprintln!("{} Installing via cargo...", "==>".green());
                let result =
                    run_shell_command_with_path("cargo install worktrunk", &ctx.path_with_local);
                if let Ok(output) = result {
                    if output.status.success() {
                        // Configure shell integration
                        let _ = run_shell_command_with_path(
                            "wt config shell install",
                            &ctx.path_with_local,
                        );
                        eprintln!("{} worktrunk installed via cargo", "==>".green());
                        return Ok(false);
                    }
                }
            }

            // Provide guidance if installation failed
            if ctx.platform.is_macos() {
                return Err(BootstrapError::ToolNotFound(
                    "worktrunk - Homebrew required. Install from https://brew.sh".to_string(),
                ));
            } else {
                return Err(BootstrapError::ToolNotFound(
                    "worktrunk - Homebrew or Cargo required".to_string(),
                ));
            }
        } else if ctx.platform.is_windows() {
            // Try winget on Windows
            if command_exists_with_path("winget", &ctx.path_with_local) {
                eprintln!("{} Installing via winget...", "==>".green());
                let result = run_shell_command_with_path(
                    "winget install max-sixty.worktrunk",
                    &ctx.path_with_local,
                );
                if let Ok(output) = result {
                    if output.status.success() {
                        // Configure shell integration
                        let _ = run_shell_command_with_path(
                            "git-wt config shell install",
                            &ctx.path_with_local,
                        );
                        eprintln!("{} worktrunk installed via winget", "==>".green());
                        return Ok(false);
                    }
                }
            }

            return Err(BootstrapError::ToolNotFound(
                "worktrunk - winget required on Windows".to_string(),
            ));
        }

        Err(BootstrapError::UnsupportedPlatform(
            "worktrunk installation not supported on this platform".to_string(),
        ))
    }
}
