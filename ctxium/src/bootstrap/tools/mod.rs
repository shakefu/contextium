//! Tool installers for bootstrap operations

pub mod adversarial_spec;
pub mod gibram;
pub mod memvid;
pub mod uv;
pub mod worktrunk;

use super::{shell::BootstrapContext, BootstrapError};
use async_trait::async_trait;

/// Trait for tool installers
///
/// Each tool installer implements this trait to provide:
/// - A way to check if the tool is already installed
/// - A way to install the tool
#[async_trait]
pub trait ToolInstaller: Send + Sync {
    /// Command to check if installed (e.g., "uv", "gibram-server")
    fn check_command(&self) -> &'static str;

    /// Check if the tool is already installed
    fn is_installed(&self, ctx: &BootstrapContext) -> bool {
        super::shell::command_exists_with_path(self.check_command(), &ctx.path_with_local)
    }

    /// Install the tool
    ///
    /// Returns Ok(true) if already installed, Ok(false) if newly installed
    async fn install(&self, ctx: &BootstrapContext) -> Result<bool, BootstrapError>;
}
