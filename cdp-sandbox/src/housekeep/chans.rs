//! Housekeep channel operations - extract, convert, manipulate channels
//!
//! Implements CDP's channel manipulation operations

use super::wav_cdp;
use super::{HousekeepError, Result};
use std::path::Path;

/// Extract a single channel from a multi-channel file
///
/// Channel numbers are 1-based (1 = first channel, 2 = second, etc.)
pub fn extract_channel(input: &Path, channel: usize) -> Result<()> {
    if channel == 0 {
        return Err(HousekeepError::InvalidFile(
            "Channel number must be 1 or greater".into(),
        ));
    }

    // Read the input file
    let (format, samples) = wav_cdp::read_wav_basic(input)?;

    if format.channels == 1 {
        return Err(HousekeepError::InvalidFile(
            "Cannot extract channel from mono file".into(),
        ));
    }

    if channel > format.channels as usize {
        return Err(HousekeepError::InvalidFile(format!(
            "Channel {} does not exist (file has {} channels)",
            channel, format.channels
        )));
    }

    // Extract the requested channel (convert to 0-based indexing)
    let chan_idx = channel - 1;
    let mut extracted = Vec::new();

    // Samples are interleaved: L R L R L R for stereo
    for i in (chan_idx..samples.len()).step_by(format.channels as usize) {
        extracted.push(samples[i]);
    }

    // Create output filename: input_c1.wav, input_c2.wav, etc.
    let stem = input.file_stem().unwrap().to_str().unwrap();
    let output_name = format!("{}_c{}.wav", stem, channel);
    let output = input.with_file_name(output_name);

    // Create mono format
    let mut mono_format = format.clone();
    mono_format.channels = 1;
    mono_format.data_size = (extracted.len() * 2) as u32;

    // Write the extracted channel with CDP format
    wav_cdp::write_wav_cdp(&output, &mono_format, &extracted)?;

    Ok(())
}

/// CLI compatibility layer for channel operations
pub fn chans(mode: i32, args: &[&str]) -> Result<()> {
    match mode {
        1 => {
            // Extract a channel
            if args.len() < 2 {
                return Err(HousekeepError::InvalidFile(
                    "Usage: chans 1 infile channo".into(),
                ));
            }
            let input = Path::new(args[0]);
            let channel = args[1]
                .parse::<usize>()
                .map_err(|_| HousekeepError::InvalidFile("Invalid channel number".into()))?;
            extract_channel(input, channel)
        }
        2 => {
            // Extract all channels - TODO
            Err(HousekeepError::UnsupportedFormat(
                "Mode 2 (extract all) not yet implemented".into(),
            ))
        }
        3 => {
            // Zero one channel - TODO
            Err(HousekeepError::UnsupportedFormat(
                "Mode 3 (zero channel) not yet implemented".into(),
            ))
        }
        4 => {
            // Mix down to mono - TODO
            Err(HousekeepError::UnsupportedFormat(
                "Mode 4 (mix to mono) not yet implemented".into(),
            ))
        }
        5 => {
            // Mono to stereo - TODO
            Err(HousekeepError::UnsupportedFormat(
                "Mode 5 (mono to stereo) not yet implemented".into(),
            ))
        }
        _ => Err(HousekeepError::UnsupportedFormat(format!(
            "Unknown chans mode: {}",
            mode
        ))),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_extract_channel_validation() {
        let temp_dir = TempDir::new().unwrap();
        let input = temp_dir.path().join("test.wav");

        // Test invalid channel number
        let result = extract_channel(&input, 0);
        assert!(result.is_err());
    }
}
