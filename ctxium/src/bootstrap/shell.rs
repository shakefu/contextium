//! Shell command execution helpers for bootstrap operations

use super::{BootstrapError, Platform};
use std::path::PathBuf;
use std::process::{Command, Output};

/// Context for bootstrap operations
#[derive(Debug, Clone)]
pub struct BootstrapContext {
    /// Repository root path
    pub repo_root: PathBuf,
    /// Detected platform
    pub platform: Platform,
    /// Local bin directory (~/.local/bin)
    pub local_bin: PathBuf,
    /// PATH with local bin prepended
    pub path_with_local: String,
}

impl BootstrapContext {
    pub fn new(repo_root: PathBuf, platform: Platform) -> Self {
        let home_dir = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/tmp"));
        let local_bin = home_dir.join(".local/bin");
        let cargo_bin = home_dir.join(".cargo/bin");

        // Build PATH with local directories prepended
        let current_path = std::env::var("PATH").unwrap_or_default();
        let path_with_local = format!(
            "{}:{}:{}",
            local_bin.display(),
            cargo_bin.display(),
            current_path
        );

        Self {
            repo_root,
            platform,
            local_bin,
            path_with_local,
        }
    }

    /// Get the tools directory (repo_root/tools)
    pub fn tools_dir(&self) -> PathBuf {
        self.repo_root.join("tools")
    }

    /// Get the skills directory (repo_root/.claude/skills)
    pub fn skills_dir(&self) -> PathBuf {
        self.repo_root.join(".claude/skills")
    }

    /// Get the venv directory (repo_root/.venv)
    pub fn venv_dir(&self) -> PathBuf {
        self.repo_root.join(".venv")
    }
}

/// Check if a command exists in a specific PATH
pub fn command_exists_with_path(cmd: &str, path: &str) -> bool {
    // First check if it's an absolute path
    if std::path::Path::new(cmd).is_absolute() {
        return std::path::Path::new(cmd).exists();
    }

    // Check each directory in PATH
    for dir in path.split(':') {
        let full_path = std::path::Path::new(dir).join(cmd);
        if full_path.exists() && full_path.is_file() {
            return true;
        }
    }

    // Fall back to using `which`
    Command::new("which")
        .arg(cmd)
        .env("PATH", path)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Run a command and return its output
pub fn run_command(program: &str, args: &[&str]) -> Result<Output, BootstrapError> {
    Command::new(program)
        .args(args)
        .output()
        .map_err(|e| BootstrapError::CommandFailed {
            command: format!("{} {}", program, args.join(" ")),
            message: e.to_string(),
        })
}

/// Run a command in a specific directory
pub fn run_command_in_dir(
    program: &str,
    args: &[&str],
    dir: &std::path::Path,
) -> Result<Output, BootstrapError> {
    Command::new(program)
        .args(args)
        .current_dir(dir)
        .output()
        .map_err(|e| BootstrapError::CommandFailed {
            command: format!("{} {}", program, args.join(" ")),
            message: e.to_string(),
        })
}

/// Run a shell command (via sh -c)
pub fn run_shell_command(command: &str) -> Result<Output, BootstrapError> {
    Command::new("sh")
        .arg("-c")
        .arg(command)
        .output()
        .map_err(|e| BootstrapError::CommandFailed {
            command: command.to_string(),
            message: e.to_string(),
        })
}

/// Run a shell command with custom PATH
pub fn run_shell_command_with_path(command: &str, path: &str) -> Result<Output, BootstrapError> {
    Command::new("sh")
        .arg("-c")
        .arg(command)
        .env("PATH", path)
        .output()
        .map_err(|e| BootstrapError::CommandFailed {
            command: command.to_string(),
            message: e.to_string(),
        })
}

/// Ensure a directory exists
pub fn ensure_dir(path: &std::path::Path) -> Result<(), BootstrapError> {
    std::fs::create_dir_all(path).map_err(BootstrapError::IoError)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_exists_with_path() {
        let path = std::env::var("PATH").unwrap_or_default();
        assert!(command_exists_with_path("ls", &path));
        assert!(command_exists_with_path("sh", &path));
        assert!(!command_exists_with_path("nonexistent_command_xyz", &path));
    }

    #[test]
    fn test_run_command() {
        let output = run_command("echo", &["hello"]).expect("echo should succeed");
        assert!(output.status.success());
        assert!(String::from_utf8_lossy(&output.stdout).contains("hello"));
    }
}
