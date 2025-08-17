//! Housekeep copy operation - byte-perfect file copying
//!
//! This is our first CDP operation implementation, chosen because:
//! 1. It's the simplest possible operation (just file I/O)
//! 2. Easy to validate (binary comparison)
//! 3. Establishes our WAV file handling

use std::path::Path;
use super::Result;
use super::wav_cdp;

/// Copy a WAV file, preserving exact format and data
/// 
/// Mode parameter (CDP compatibility):
/// - 1: Normal copy with CDP metadata
/// - 2: Future: copy with normalization
/// - 3: Future: copy with conversion
pub fn copy_file(input: &Path, output: &Path, mode: i32) -> Result<()> {
    match mode {
        1 => {
            // Use CDP-compatible WAV format
            wav_cdp::copy_wav_cdp_format(input, output)?;
            Ok(())
        }
        _ => {
            Err(super::HousekeepError::UnsupportedFormat(
                format!("Mode {} not yet implemented", mode)
            ))
        }
    }
}

/// Library-friendly version without mode parameter
pub fn copy(input: &Path, output: &Path) -> Result<()> {
    copy_file(input, output, 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_basic_copy() {
        let temp_dir = TempDir::new().unwrap();
        let input = temp_dir.path().join("input.wav");
        let output = temp_dir.path().join("output.wav");
        
        // Create a dummy file
        fs::write(&input, b"RIFF....WAVEfmt ").unwrap();
        
        // Copy it
        copy(&input, &output).unwrap();
        
        // Verify
        let input_data = fs::read(&input).unwrap();
        let output_data = fs::read(&output).unwrap();
        assert_eq!(input_data, output_data);
    }
}