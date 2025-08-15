//! Oracle testing framework using CDP binaries as ground truth

use std::path::{Path, PathBuf};
use std::process::Command;
use tempfile::TempDir;
use thiserror::Error;

pub mod audio;
pub mod validator;
pub mod generator;

pub use validator::{Validator, ValidationResult};
pub use generator::TestGenerator;

#[derive(Error, Debug)]
pub enum OracleError {
    #[error("CDP binary not found: {0}")]
    CdpBinaryNotFound(String),
    
    #[error("CDP execution failed: {0}")]
    CdpExecutionFailed(String),
    
    #[error("Audio comparison failed: {0}")]
    ComparisonFailed(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Audio format error: {0}")]
    AudioFormat(#[from] hound::Error),
}

pub type Result<T> = std::result::Result<T, OracleError>;

/// Configuration for the CDP Oracle
#[derive(Debug, Clone)]
pub struct OracleConfig {
    /// Path to CDP binaries directory
    pub cdp_path: Option<PathBuf>,
    
    /// Tolerance for floating-point comparison
    pub tolerance: f32,
    
    /// Whether to keep temporary files for debugging
    pub keep_temp_files: bool,
    
    /// Maximum difference in spectral correlation to consider a match
    pub spectral_threshold: f32,
}

impl Default for OracleConfig {
    fn default() -> Self {
        Self {
            cdp_path: None,
            tolerance: 1e-6,
            keep_temp_files: false,
            spectral_threshold: 0.9999,
        }
    }
}

/// Main Oracle struct for running CDP binaries
pub struct CdpOracle {
    config: OracleConfig,
    temp_dir: Option<TempDir>,
}

impl CdpOracle {
    pub fn new(config: OracleConfig) -> Result<Self> {
        let temp_dir = if !config.keep_temp_files {
            Some(TempDir::new()?)
        } else {
            None
        };
        
        Ok(Self { config, temp_dir })
    }
    
    /// Find a CDP binary by name
    pub fn find_cdp_binary(&self, name: &str) -> Result<PathBuf> {
        if let Some(ref cdp_path) = self.config.cdp_path {
            let binary = cdp_path.join(name);
            if binary.exists() {
                return Ok(binary);
            }
        }
        
        // Try to find in PATH
        which::which(name)
            .map_err(|_| OracleError::CdpBinaryNotFound(name.to_string()))
    }
    
    /// Run a CDP binary with arguments
    pub fn run_cdp(
        &self,
        program: &str,
        args: &[&str],
    ) -> Result<Vec<u8>> {
        let binary = self.find_cdp_binary(program)?;
        
        let output = Command::new(binary)
            .args(args)
            .output()
            .map_err(|e| OracleError::CdpExecutionFailed(e.to_string()))?;
        
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(OracleError::CdpExecutionFailed(
                format!("{} failed: {}", program, stderr)
            ));
        }
        
        Ok(output.stdout)
    }
    
    /// Get temporary directory for test files
    pub fn temp_dir(&self) -> Result<&Path> {
        self.temp_dir
            .as_ref()
            .map(|d| d.path())
            .ok_or_else(|| OracleError::Io(
                std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "No temp directory available"
                )
            ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_oracle_creation() {
        let config = OracleConfig::default();
        let oracle = CdpOracle::new(config);
        assert!(oracle.is_ok());
    }
}