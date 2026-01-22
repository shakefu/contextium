//! Download and extraction utilities for bootstrap operations

use super::BootstrapError;
use flate2::read::GzDecoder;
use std::path::{Path, PathBuf};
use tar::Archive;
use tempfile::TempDir;

/// Download a file to a temporary directory and extract if it's a tar.gz
pub async fn download_and_extract(
    client: &reqwest::Client,
    url: &str,
) -> Result<TempDir, BootstrapError> {
    let temp_dir = TempDir::new().map_err(BootstrapError::IoError)?;

    // Download the file
    let response = client
        .get(url)
        .timeout(std::time::Duration::from_secs(60))
        .send()
        .await
        .map_err(|e| BootstrapError::DownloadFailed {
            url: url.to_string(),
            message: e.to_string(),
        })?;

    if !response.status().is_success() {
        return Err(BootstrapError::DownloadFailed {
            url: url.to_string(),
            message: format!("HTTP {}", response.status()),
        });
    }

    let bytes = response
        .bytes()
        .await
        .map_err(|e| BootstrapError::DownloadFailed {
            url: url.to_string(),
            message: e.to_string(),
        })?;

    // Determine if it's a tar.gz
    if url.ends_with(".tar.gz") || url.ends_with(".tgz") {
        extract_tar_gz(&bytes, temp_dir.path())?;
    } else {
        // Just write the file as-is
        let filename = url.split('/').last().unwrap_or("download");
        let file_path = temp_dir.path().join(filename);
        std::fs::write(&file_path, &bytes).map_err(BootstrapError::IoError)?;
    }

    Ok(temp_dir)
}

/// Extract a tar.gz archive to a directory
fn extract_tar_gz(data: &[u8], dest: &Path) -> Result<(), BootstrapError> {
    let decoder = GzDecoder::new(data);
    let mut archive = Archive::new(decoder);

    archive
        .unpack(dest)
        .map_err(|e| BootstrapError::ExtractionFailed(e.to_string()))?;

    Ok(())
}

/// Find a binary in a directory (recursively)
pub fn find_binary(dir: &Path, name: &str) -> Option<PathBuf> {
    if !dir.is_dir() {
        return None;
    }

    // First check direct children
    let direct_path = dir.join(name);
    if direct_path.is_file() {
        return Some(direct_path);
    }

    // Then search recursively
    if let Ok(entries) = std::fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() && path.file_name().map(|n| n == name).unwrap_or(false) {
                return Some(path);
            }
            if path.is_dir() {
                if let Some(found) = find_binary(&path, name) {
                    return Some(found);
                }
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_binary() {
        // Test with temp directory structure
        let temp = TempDir::new().unwrap();
        let bin_path = temp.path().join("subdir");
        std::fs::create_dir_all(&bin_path).unwrap();
        let binary = bin_path.join("test-binary");
        std::fs::write(&binary, b"test").unwrap();

        let found = find_binary(temp.path(), "test-binary");
        assert!(found.is_some());
        assert_eq!(found.unwrap().file_name().unwrap(), "test-binary");
    }
}
