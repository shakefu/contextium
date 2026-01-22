//! adversarial-spec installer - Multi-model specification refinement
//! https://github.com/zscole/adversarial-spec

use super::{BootstrapContext, BootstrapError, ToolInstaller};
use crate::bootstrap::shell::{
    command_exists_with_path, ensure_dir, run_command, run_command_in_dir,
};
use async_trait::async_trait;
use colored::Colorize;
use std::path::Path;

pub struct AdversarialSpecInstaller;

impl AdversarialSpecInstaller {
    fn spec_dir(ctx: &BootstrapContext) -> std::path::PathBuf {
        ctx.tools_dir().join("adversarial-spec")
    }

    fn is_cloned(ctx: &BootstrapContext) -> bool {
        Self::spec_dir(ctx).exists()
    }
}

#[async_trait]
impl ToolInstaller for AdversarialSpecInstaller {
    fn check_command(&self) -> &'static str {
        // adversarial-spec doesn't have a binary - check if directory exists
        "adversarial-spec"
    }

    fn is_installed(&self, ctx: &BootstrapContext) -> bool {
        Self::is_cloned(ctx)
    }

    async fn install(&self, ctx: &BootstrapContext) -> Result<bool, BootstrapError> {
        eprintln!("{} Installing adversarial-spec...", "==>".green());

        // Require uv for Python dependencies
        if !command_exists_with_path("uv", &ctx.path_with_local) {
            return Err(BootstrapError::ToolNotFound(
                "adversarial-spec requires uv for Python dependencies".to_string(),
            ));
        }

        // Ensure venv exists
        let venv_dir = ctx.venv_dir();
        if !venv_dir.exists() {
            eprintln!("{} Creating .venv...", "==>".green());
            let result = run_command_in_dir("uv", &["venv", ".venv"], &ctx.repo_root);
            if let Err(e) = result {
                eprintln!("{} Failed to create venv: {}", "==>".yellow(), e);
            }
        }

        // Install litellm
        eprintln!("{} Installing litellm via uv...", "==>".green());
        let result = run_command_in_dir(
            "uv",
            &["pip", "install", "--quiet", "--upgrade", "litellm"],
            &ctx.repo_root,
        );
        match result {
            Ok(output) if !output.status.success() => {
                let stderr = String::from_utf8_lossy(&output.stderr);
                eprintln!("{} litellm install failed: {}", "==>".yellow(), stderr);
            }
            Err(e) => {
                eprintln!("{} litellm install failed: {}", "==>".yellow(), e);
            }
            _ => {}
        }

        // Clone or update the repo
        let tools_dir = ctx.tools_dir();
        let spec_dir = Self::spec_dir(ctx);

        ensure_dir(&tools_dir)?;

        if spec_dir.exists() {
            eprintln!(
                "{} adversarial-spec already cloned, updating...",
                "==>".green()
            );
            let _ = run_command_in_dir("git", &["pull", "--quiet"], &spec_dir);
            return Ok(true);
        }

        eprintln!("{} Cloning adversarial-spec...", "==>".green());
        let result = run_command(
            "git",
            &[
                "clone",
                "--quiet",
                "https://github.com/zscole/adversarial-spec",
                &spec_dir.to_string_lossy(),
            ],
        );

        match result {
            Ok(output) if !output.status.success() => {
                let stderr = String::from_utf8_lossy(&output.stderr);
                eprintln!(
                    "{} Failed to clone adversarial-spec: {}",
                    "==>".yellow(),
                    stderr
                );
                return Err(BootstrapError::CommandFailed {
                    command: "git clone".to_string(),
                    message: "Failed to clone adversarial-spec".to_string(),
                });
            }
            Err(e) => {
                eprintln!("{} Failed to clone: {}", "==>".yellow(), e);
                return Err(e);
            }
            _ => {}
        }

        // Create symlink in .claude/skills
        let skills_dir = ctx.skills_dir();
        let skill_link = skills_dir.join("adversarial-spec");
        let skill_source = spec_dir.join("skills/adversarial-spec");

        // Remove existing symlink
        if skill_link.exists() || skill_link.is_symlink() {
            let _ = std::fs::remove_file(&skill_link);
        }

        if skill_source.exists() {
            // Use relative path from .claude/skills/ to tools/
            let relative_path = Path::new("../../tools/adversarial-spec/skills/adversarial-spec");

            #[cfg(unix)]
            {
                use std::os::unix::fs::symlink;
                if let Err(e) = symlink(relative_path, &skill_link) {
                    eprintln!("{} Failed to create skill symlink: {}", "==>".yellow(), e);
                } else {
                    eprintln!(
                        "{} Skill linked: .claude/skills/adversarial-spec",
                        "==>".green()
                    );
                }
            }

            #[cfg(not(unix))]
            {
                eprintln!(
                    "{} Symlinks not supported on this platform - manual setup required",
                    "==>".yellow()
                );
            }
        } else {
            eprintln!(
                "{} Skill source not found at {}",
                "==>".yellow(),
                skill_source.display()
            );
        }

        eprintln!("{} adversarial-spec installed", "==>".green());
        eprintln!("{}", "");
        eprintln!(
            "{} Usage: /adversarial-spec \"your feature description\"",
            "==>".green()
        );
        eprintln!("{}", "");
        eprintln!("{} API keys (set at least one):", "==>".green());
        eprintln!(
            "{}   OPENAI_API_KEY, ANTHROPIC_API_KEY, GEMINI_API_KEY",
            "==>".green()
        );
        eprintln!("{}   XAI_API_KEY, OPENROUTER_API_KEY, etc.", "==>".green());

        Ok(false)
    }
}
