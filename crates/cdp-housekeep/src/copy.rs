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
        use hound::{WavSpec, WavWriter};

        let temp_dir = TempDir::new().unwrap();
        let input = temp_dir.path().join("input.wav");
        let output = temp_dir.path().join("output.wav");

        // Create a proper WAV file with actual audio data
        let spec = WavSpec {
            channels: 1,
            sample_rate: 44100,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };

        let mut writer = WavWriter::create(&input, spec).unwrap();
        // Write 100 samples of silence
        for _ in 0..100 {
            writer.write_sample(0i16).unwrap();
        }
        writer.finalize().unwrap();

        // Try to copy the file
        let result = copy(&input, &output);

        // This should work with our improved WAV reader
        if let Err(e) = &result {
            eprintln!("Copy error: {:?}", e);
        }
        assert!(result.is_ok(), "Copy should succeed");

        // Verify output file was created
        assert!(output.exists(), "Output file should exist");

        // Verify it's a valid WAV file
        let reader = hound::WavReader::open(&output);
        assert!(reader.is_ok(), "Output should be a valid WAV file");
    }
}
