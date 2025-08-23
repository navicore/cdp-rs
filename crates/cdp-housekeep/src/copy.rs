//! Housekeep copy operation - byte-perfect file copying
//!
//! This is our first CDP operation implementation, chosen because:
//! 1. It's the simplest possible operation (just file I/O)
//! 2. Easy to validate (binary comparison)
//! 3. Establishes our WAV file handling

use super::wav_cdp;
use super::Result;
use std::path::Path;

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
            wav_cdp::copy_wav_cdp(input, output)?;
            Ok(())
        }
        _ => Err(super::HousekeepError::UnsupportedFormat(format!(
            "Mode {} not yet implemented",
            mode
        ))),
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
        // This test needs a real WAV file, not a dummy one
        // For now, just test that the function exists and compiles
        // Real testing is done via oracle tests against CDP
        let temp_dir = TempDir::new().unwrap();
        let input = temp_dir.path().join("input.wav");
        let output = temp_dir.path().join("output.wav");

        // Create a minimal valid WAV file (44 byte header + empty data)
        let wav_data = vec![
            b'R', b'I', b'F', b'F', // "RIFF"
            36, 0, 0, 0, // File size - 8
            b'W', b'A', b'V', b'E', // "WAVE"
            b'f', b'm', b't', b' ', // "fmt "
            16, 0, 0, 0, // fmt chunk size
            1, 0, // PCM format
            1, 0, // Mono
            0x44, 0xAC, 0, 0, // 44100 Hz
            0x88, 0x58, 0x01, 0, // Byte rate
            2, 0, // Block align
            16, 0, // 16 bits per sample
            b'd', b'a', b't', b'a', // "data"
            0, 0, 0, 0, // Data size = 0
        ];
        fs::write(&input, &wav_data).unwrap();

        // Try to copy the file
        let result = copy(&input, &output);

        // This should work with our improved WAV reader
        if let Err(e) = &result {
            eprintln!("Copy error: {:?}", e);
        }
        assert!(result.is_ok());

        // Verify output file was created
        assert!(output.exists());
    }
}
