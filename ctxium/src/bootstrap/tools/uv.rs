//! uv installer - Fast Python package manager from Astral
//! https://github.com/astral-sh/uv

use super::{BootstrapContext, BootstrapError, ToolInstaller};
use crate::bootstrap::shell::{
    command_exists_with_path, run_shell_command, run_shell_command_with_path,
};
use async_trait::async_trait;
use colored::Colorize;

pub struct UvInstaller;

#[async_trait]
impl ToolInstaller for UvInstaller {
    fn check_command(&self) -> &'static str {
        "uv"
    }

    async fn install(&self, ctx: &BootstrapContext) -> Result<bool, BootstrapError> {
        // Check if already installed
        if self.is_installed(ctx) {
            eprintln!("{} uv already installed", "==>".green());
            return Ok(true);
        }

        eprintln!("{} Installing uv...", "==>".green());

        // Try the official installer first
        let install_result = run_shell_command("curl -LsSf https://astral.sh/uv/install.sh | sh");

        if let Ok(output) = install_result {
            if output.status.success() {
                // Verify installation with updated PATH
                if command_exists_with_path("uv", &ctx.path_with_local) {
                    eprintln!("{} uv installed successfully", "==>".green());
                    return Ok(false);
                }
            }
        }

        // Fallback: try Homebrew on macOS/Linux
        if command_exists_with_path("brew", &ctx.path_with_local) {
            eprintln!("{} Trying Homebrew...", "==>".green());
            let brew_result = run_shell_command_with_path("brew install uv", &ctx.path_with_local);
            if let Ok(output) = brew_result {
                if output.status.success() && command_exists_with_path("uv", &ctx.path_with_local) {
                    eprintln!("{} uv installed via Homebrew", "==>".green());
                    return Ok(false);
                }
            }
        }

        // Fallback: try pipx (if available)
        if command_exists_with_path("pipx", &ctx.path_with_local) {
            eprintln!("{} Trying pipx...", "==>".green());
            let pipx_result = run_shell_command_with_path("pipx install uv", &ctx.path_with_local);
            if let Ok(output) = pipx_result {
                if output.status.success() && command_exists_with_path("uv", &ctx.path_with_local) {
                    eprintln!("{} uv installed via pipx", "==>".green());
                    return Ok(false);
                }
            }
        }

        Err(BootstrapError::ToolNotFound(
            "uv - try: curl -LsSf https://astral.sh/uv/install.sh | sh".to_string(),
        ))
    }
}
