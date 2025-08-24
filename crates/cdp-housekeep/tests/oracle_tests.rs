//! Oracle tests for cdp-housekeep

use cdp_housekeep::copy;
use cdp_oracle::test_utils::cdp_command;
use cdp_oracle::wav_compare::{compare_wav_files, has_cdp_format};
use hound::{WavSpec, WavWriter};
use std::path::Path;
use tempfile::tempdir;

/// Create a proper test WAV file
fn create_test_wav(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let spec = WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = WavWriter::create(path, spec)?;

    // Generate a simple sine wave
    for i in 0..44100 {
        let t = i as f32 / 44100.0;
        let sample = (2.0 * std::f32::consts::PI * 440.0 * t).sin();
        writer.write_sample((sample * 16384.0) as i16)?;
    }

    writer.finalize()?;
    Ok(())
}

#[test]
fn test_copy_produces_cdp_format() {
    let dir = tempdir().unwrap();
    let input = dir.path().join("input.wav");
    let output = dir.path().join("output.wav");

    // Create test input
    create_test_wav(&input).unwrap();

    // Run our copy
    copy(&input, &output).unwrap();

    // Check that output has CDP format
    assert!(
        has_cdp_format(&output).unwrap(),
        "Output should have CDP format with PEAK, cue, and LIST chunks"
    );
}

#[test]
fn test_copy_matches_cdp() {
    let dir = tempdir().unwrap();
    let input = dir.path().join("input.wav");
    let cdp_output = dir.path().join("cdp_output.wav");
    let rust_output = dir.path().join("rust_output.wav");

    // Create test input
    create_test_wav(&input).unwrap();

    // Run CDP housekeep copy
    let cdp_result = cdp_command("housekeep")
        .args([
            "copy",
            "1",
            input.to_str().unwrap(),
            cdp_output.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to run CDP housekeep copy");

    assert!(cdp_result.status.success(), "CDP housekeep copy failed");

    // Run our copy
    copy(&input, &rust_output).unwrap();

    // Compare outputs intelligently
    let comparison = compare_wav_files(&cdp_output, &rust_output).unwrap();

    assert!(
        comparison.format_matches,
        "Format should match: {}",
        comparison.details
    );
    assert!(
        comparison.data_matches,
        "Audio data should match: {}",
        comparison.details
    );
    assert!(
        comparison.peak_matches,
        "Peak values should match (ignoring timestamp): {}",
        comparison.details
    );

    // Don't require exact chunk match - CDP might add extra chunks we don't
    // The important thing is we have the essential CDP chunks
    assert!(
        has_cdp_format(&rust_output).unwrap(),
        "Our output should have CDP format"
    );
}

#[test]
fn test_copy_preserves_audio() {
    let dir = tempdir().unwrap();
    let input = dir.path().join("input.wav");
    let output = dir.path().join("output.wav");

    // Create test input with known pattern
    let spec = WavSpec {
        channels: 2,
        sample_rate: 48000,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut writer = WavWriter::create(&input, spec).unwrap();
    let test_samples: Vec<i16> = (0..1000).map(|i| ((i * 100) % 32768) as i16).collect();
    for &sample in &test_samples {
        writer.write_sample(sample).unwrap();
    }
    writer.finalize().unwrap();

    // Copy the file
    copy(&input, &output).unwrap();

    // Read back and verify audio is preserved
    let reader = hound::WavReader::open(&output).unwrap();
    let output_samples: Vec<i16> = reader.into_samples::<i16>().map(|s| s.unwrap()).collect();

    assert_eq!(
        test_samples.len(),
        output_samples.len(),
        "Sample count should be preserved"
    );

    for (i, (&original, &copied)) in test_samples.iter().zip(output_samples.iter()).enumerate() {
        assert_eq!(
            original, copied,
            "Sample {} should match: {} != {}",
            i, original, copied
        );
    }
}
