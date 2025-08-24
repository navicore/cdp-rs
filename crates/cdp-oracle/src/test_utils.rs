//! Test utilities for finding and running CDP binaries in tests

use std::env;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Get the path to a CDP binary for testing
///
/// This function looks for CDP binaries in the following order:
/// 1. In PATH (if already set by Makefile)
/// 2. In build/cdp-install/bin relative to workspace root
/// 3. In ../../../build/cdp-install/bin relative to test directory
///
/// # Panics
/// Panics if the CDP binary cannot be found. Tests should never skip - they should fail
/// if the required CDP binaries are not available.
pub fn get_cdp_binary_path(binary_name: &str) -> PathBuf {
    // First check if it's already in PATH (e.g., when run via Makefile)
    if let Ok(output) = Command::new("which").arg(binary_name).output() {
        if output.status.success() {
            let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !path.is_empty() && Path::new(&path).exists() {
                return PathBuf::from(path);
            }
        }
    }

    // Try to find workspace root by looking for Cargo.toml with [workspace]
    let mut current_dir = env::current_dir().expect("Failed to get current directory");
    let mut workspace_root = None;

    while current_dir.parent().is_some() {
        let cargo_toml = current_dir.join("Cargo.toml");
        if cargo_toml.exists() {
            // Check if this is the workspace root
            if let Ok(contents) = std::fs::read_to_string(&cargo_toml) {
                if contents.contains("[workspace]") {
                    workspace_root = Some(current_dir.clone());
                    break;
                }
            }
        }
        current_dir = current_dir.parent().unwrap().to_path_buf();
    }

    // Try workspace root relative path
    if let Some(root) = workspace_root {
        let cdp_bin_path = root
            .join("build")
            .join("cdp-install")
            .join("bin")
            .join(binary_name);
        if cdp_bin_path.exists() {
            return cdp_bin_path;
        }
    }

    // Try relative paths from test location (for backwards compatibility)
    let relative_paths = [
        format!("build/cdp-install/bin/{}", binary_name),
        format!("../build/cdp-install/bin/{}", binary_name),
        format!("../../build/cdp-install/bin/{}", binary_name),
        format!("../../../build/cdp-install/bin/{}", binary_name),
    ];

    for relative_path in &relative_paths {
        let path = Path::new(relative_path);
        if path.exists() {
            return path.canonicalize().expect("Failed to canonicalize path");
        }
    }

    panic!(
        "CDP binary '{}' not found. CDP is REQUIRED for all tests.\n\
        Please run 'make install-cdp' to install CDP binaries.\n\
        Searched in:\n\
        - PATH\n\
        - workspace_root/build/cdp-install/bin/\n\
        - Various relative paths from current directory",
        binary_name
    )
}

/// Create a Command for a CDP binary
///
/// This is a convenience function that creates a Command with the correct path
/// to the CDP binary.
pub fn cdp_command(binary_name: &str) -> Command {
    Command::new(get_cdp_binary_path(binary_name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_cdp_binary() {
        // This test verifies that we can find at least one CDP binary
        // CDP is REQUIRED - this test should fail if CDP is not installed
        let pvoc_path = get_cdp_binary_path("pvoc");
        assert!(
            pvoc_path.exists(),
            "CDP pvoc binary should exist at {:?}",
            pvoc_path
        );
    }
}
