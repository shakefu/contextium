//! macOS code signing utilities for bootstrap operations

#[cfg(target_os = "macos")]
use super::shell::run_command;
use super::BootstrapError;
use std::path::Path;

/// Ad-hoc sign a binary on macOS
///
/// This is required for binaries downloaded from the internet on macOS
/// to avoid "cannot be opened because the developer cannot be verified" errors.
#[cfg_attr(not(target_os = "macos"), allow(dead_code))]
pub fn codesign_adhoc(binary_path: &Path) -> Result<(), BootstrapError> {
    #[cfg(target_os = "macos")]
    {
        let path_str = binary_path.to_string_lossy();
        let output = run_command("codesign", &["--force", "--sign", "-", &path_str])?;

        if !output.status.success() {
            // Codesign failure is not fatal - binary might still work
            // Just log the warning
            let stderr = String::from_utf8_lossy(&output.stderr);
            eprintln!("Warning: codesign failed for {}: {}", path_str, stderr);
        }
        Ok(())
    }

    #[cfg(not(target_os = "macos"))]
    {
        let _ = binary_path;
        Ok(())
    }
}

/// Remove quarantine attribute on macOS
///
/// Downloaded binaries may have a quarantine attribute that prevents execution.
#[cfg_attr(not(target_os = "macos"), allow(dead_code))]
pub fn remove_quarantine(path: &Path) -> Result<(), BootstrapError> {
    #[cfg(target_os = "macos")]
    {
        let path_str = path.to_string_lossy();
        // Use xattr to remove quarantine - failure is not fatal
        let _ = run_command("xattr", &["-d", "com.apple.quarantine", &path_str]);
        Ok(())
    }

    #[cfg(not(target_os = "macos"))]
    {
        let _ = path;
        Ok(())
    }
}

/// Prepare a downloaded binary for execution on macOS
///
/// This removes quarantine attributes and performs ad-hoc signing.
pub fn prepare_binary(binary_path: &Path) -> Result<(), BootstrapError> {
    // Make executable
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = std::fs::metadata(binary_path)
            .map_err(BootstrapError::IoError)?
            .permissions();
        perms.set_mode(0o755);
        std::fs::set_permissions(binary_path, perms).map_err(BootstrapError::IoError)?;
    }

    // On macOS, also handle signing
    #[cfg(target_os = "macos")]
    {
        remove_quarantine(binary_path)?;
        codesign_adhoc(binary_path)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    #[test]
    fn test_prepare_binary() {
        let temp = NamedTempFile::new().unwrap();
        std::fs::write(temp.path(), b"#!/bin/sh\necho test").unwrap();

        // Should not fail even on non-macOS
        prepare_binary(temp.path()).expect("prepare_binary should succeed");

        // Check that file is executable
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            let perms = std::fs::metadata(temp.path()).unwrap().permissions();
            assert!(perms.mode() & 0o111 != 0, "File should be executable");
        }
    }
}
