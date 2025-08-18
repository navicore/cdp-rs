//! Loudness modification operations
//!
//! Provides gain adjustment, normalization, and other amplitude-related operations

use super::{ModifyError, Result};
use crate::housekeep::wav_cdp;
use std::path::Path;

/// Apply gain to audio samples
pub fn apply_gain(input: &Path, output: &Path, gain: f32) -> Result<()> {
    // Read input file
    let (format, samples) = wav_cdp::read_wav_basic(input)?;

    // Apply gain to all samples
    let mut processed = Vec::with_capacity(samples.len());
    for sample in samples {
        let scaled = (sample as f32 * gain) as i32;
        // Clamp to 16-bit range
        let clamped = scaled.clamp(-32768, 32767) as i16;
        processed.push(clamped);
    }

    // Write output with CDP format
    wav_cdp::write_wav_cdp(output, &format, &processed)?;
    Ok(())
}

/// Normalize audio to maximum level (or specified level)
pub fn normalize(input: &Path, output: &Path, target_level: Option<f32>) -> Result<()> {
    // Read input file
    let (format, samples) = wav_cdp::read_wav_basic(input)?;

    // Find peak value
    let peak = samples.iter().map(|&s| s.abs()).max().unwrap_or(0) as f32 / 32767.0;

    if peak == 0.0 {
        // Silent file, just copy
        wav_cdp::write_wav_cdp(output, &format, &samples)?;
        return Ok(());
    }

    // Calculate gain needed
    let target = target_level.unwrap_or(1.0);
    if target > 1.0 {
        return Err(ModifyError::InvalidParameter(
            "Target level cannot exceed 1.0".into(),
        ));
    }

    let gain = target / peak;

    // Apply normalization
    let mut processed = Vec::with_capacity(samples.len());
    for sample in samples {
        let scaled = (sample as f32 * gain) as i32;
        // Should not need clamping for normalize, but be safe
        let clamped = scaled.clamp(-32768, 32767) as i16;
        processed.push(clamped);
    }

    // Write output
    wav_cdp::write_wav_cdp(output, &format, &processed)?;
    Ok(())
}

/// Apply dB gain adjustment
pub fn apply_db_gain(input: &Path, output: &Path, db_gain: f32) -> Result<()> {
    // Convert dB to linear gain
    let gain = 10.0_f32.powf(db_gain / 20.0);
    apply_gain(input, output, gain)
}

/// CLI compatibility layer for loudness operations
pub fn loudness(mode: i32, args: &[&str]) -> Result<()> {
    match mode {
        1 => {
            // Gain adjustment
            if args.len() < 3 {
                return Err(ModifyError::InvalidParameter(
                    "Usage: loudness 1 infile outfile gain".into(),
                ));
            }
            let input = Path::new(args[0]);
            let output = Path::new(args[1]);
            let gain = args[2]
                .parse::<f32>()
                .map_err(|_| ModifyError::InvalidParameter("Invalid gain value".into()))?;
            apply_gain(input, output, gain)
        }
        2 => {
            // dB gain adjustment
            if args.len() < 3 {
                return Err(ModifyError::InvalidParameter(
                    "Usage: loudness 2 infile outfile gain_db".into(),
                ));
            }
            let input = Path::new(args[0]);
            let output = Path::new(args[1]);
            let db_gain = args[2]
                .parse::<f32>()
                .map_err(|_| ModifyError::InvalidParameter("Invalid dB gain value".into()))?;

            if !(-96.0..=96.0).contains(&db_gain) {
                return Err(ModifyError::InvalidParameter(
                    "dB gain must be between -96 and +96".into(),
                ));
            }

            apply_db_gain(input, output, db_gain)
        }
        3 => {
            // Normalize
            if args.len() < 2 {
                return Err(ModifyError::InvalidParameter(
                    "Usage: loudness 3 infile outfile [-llevel]".into(),
                ));
            }
            let input = Path::new(args[0]);
            let output = Path::new(args[1]);

            // Check for optional level parameter
            let level = if args.len() > 2 && args[2].starts_with("-l") {
                let level_str = &args[2][2..];
                Some(
                    level_str
                        .parse::<f32>()
                        .map_err(|_| ModifyError::InvalidParameter("Invalid level value".into()))?,
                )
            } else {
                None
            };

            normalize(input, output, level)
        }
        6 => {
            // Invert phase
            if args.len() < 2 {
                return Err(ModifyError::InvalidParameter(
                    "Usage: loudness 6 infile outfile".into(),
                ));
            }
            let input = Path::new(args[0]);
            let output = Path::new(args[1]);

            // Invert phase is just gain of -1
            apply_gain(input, output, -1.0)
        }
        _ => Err(ModifyError::UnsupportedOperation(format!(
            "Loudness mode {} not yet implemented",
            mode
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_gain_validation() {
        let temp_dir = TempDir::new().unwrap();
        let input = temp_dir.path().join("input.wav");
        let output = temp_dir.path().join("output.wav");

        // Test with non-existent file
        let result = apply_gain(&input, &output, 2.0);
        assert!(result.is_err());
    }

    #[test]
    fn test_normalize_validation() {
        let temp_dir = TempDir::new().unwrap();
        let input = temp_dir.path().join("input.wav");
        let output = temp_dir.path().join("output.wav");

        // Test invalid target level
        let result = normalize(&input, &output, Some(1.5));
        assert!(result.is_err());
    }
}
