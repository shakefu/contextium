//! Platform detection for bootstrap operations

use super::BootstrapError;

/// Operating system
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)] // Variants constructed via #[cfg] conditional compilation
pub enum Os {
    Darwin,
    Linux,
    Windows,
}

impl std::fmt::Display for Os {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Darwin => write!(f, "darwin"),
            Self::Linux => write!(f, "linux"),
            Self::Windows => write!(f, "windows"),
        }
    }
}

/// CPU architecture
#[derive(Debug, Clone, PartialEq)]
#[allow(dead_code)] // Variants constructed via #[cfg] conditional compilation
pub enum Arch {
    Amd64,
    Arm64,
}

impl std::fmt::Display for Arch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Amd64 => write!(f, "amd64"),
            Self::Arm64 => write!(f, "arm64"),
        }
    }
}

/// Platform information (OS + architecture)
#[derive(Debug, Clone)]
pub struct Platform {
    pub os: Os,
    pub arch: Arch,
}

impl Default for Platform {
    fn default() -> Self {
        Self {
            os: Os::Darwin,
            arch: Arch::Arm64,
        }
    }
}

impl std::fmt::Display for Platform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}-{}", self.os, self.arch)
    }
}

impl Platform {
    /// Detect the current platform
    pub fn detect() -> Result<Self, BootstrapError> {
        let os = Self::detect_os()?;
        let arch = Self::detect_arch()?;
        Ok(Self { os, arch })
    }

    /// Detect operating system
    fn detect_os() -> Result<Os, BootstrapError> {
        #[cfg(target_os = "macos")]
        return Ok(Os::Darwin);

        #[cfg(target_os = "linux")]
        return Ok(Os::Linux);

        #[cfg(target_os = "windows")]
        return Ok(Os::Windows);

        #[cfg(not(any(target_os = "macos", target_os = "linux", target_os = "windows")))]
        return Err(BootstrapError::UnsupportedPlatform(format!(
            "Unsupported OS: {}",
            std::env::consts::OS
        )));
    }

    /// Detect CPU architecture
    fn detect_arch() -> Result<Arch, BootstrapError> {
        #[cfg(target_arch = "x86_64")]
        return Ok(Arch::Amd64);

        #[cfg(target_arch = "aarch64")]
        return Ok(Arch::Arm64);

        #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
        return Err(BootstrapError::UnsupportedPlatform(format!(
            "Unsupported architecture: {}",
            std::env::consts::ARCH
        )));
    }

    /// Get platform suffix for download URLs (e.g., "darwin-arm64")
    pub fn as_download_suffix(&self) -> String {
        format!("{}-{}", self.os, self.arch)
    }

    /// Check if running on macOS
    pub fn is_macos(&self) -> bool {
        matches!(self.os, Os::Darwin)
    }

    /// Check if running on Linux
    pub fn is_linux(&self) -> bool {
        matches!(self.os, Os::Linux)
    }

    /// Check if running on Windows
    pub fn is_windows(&self) -> bool {
        matches!(self.os, Os::Windows)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platform_detect() {
        let platform = Platform::detect().expect("Platform detection should succeed");
        println!("Detected platform: {}", platform);
        assert!(!platform.as_download_suffix().is_empty());
    }

    #[test]
    fn test_platform_display() {
        let platform = Platform {
            os: Os::Darwin,
            arch: Arch::Arm64,
        };
        assert_eq!(platform.to_string(), "darwin-arm64");
        assert_eq!(platform.as_download_suffix(), "darwin-arm64");
    }
}
