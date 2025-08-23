//! Oracle tests comparing cdp-distort with CDP original
//!
//! These tests are ignored by default since they require CDP binaries.
//! Run with: cargo test --package cdp-distort oracle -- --ignored

use cdp_distort::{divide, multiply, overload, ClipType};
use hound::{SampleFormat, WavReader, WavSpec, WavWriter};
use std::path::Path;
use std::process::Command;
use tempfile::tempdir;

/// Create a test WAV file
fn create_test_wav(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let spec = WavSpec {
        channels: 1,
        sample_rate: 44100,
        bits_per_sample: 16,
        sample_format: SampleFormat::Int,
    };

    let mut writer = WavWriter::create(path, spec)?;

    // Generate a complex test signal
    for i in 0..44100 {
        let t = i as f32 / 44100.0;
        // Mix of frequencies with envelope
        let fundamental = (2.0 * std::f32::consts::PI * 440.0 * t).sin();
        let harmonic = (2.0 * std::f32::consts::PI * 880.0 * t).sin() * 0.3;
        let envelope = (2.0 * std::f32::consts::PI * 2.0 * t).sin() * 0.3 + 0.7;
        let sample = ((fundamental + harmonic) * envelope * 16000.0) as i16;
        writer.write_sample(sample)?;
    }

    writer.finalize()?;
    Ok(())
}

/// Compare two WAV files allowing for small differences
fn compare_wav_files(
    file1: &Path,
    file2: &Path,
    tolerance: f32,
) -> Result<bool, Box<dyn std::error::Error>> {
    let reader1 = WavReader::open(file1)?;
    let reader2 = WavReader::open(file2)?;

    let spec1 = reader1.spec();
    let spec2 = reader2.spec();

    // Check basic format matches
    if spec1.channels != spec2.channels || spec1.sample_rate != spec2.sample_rate {
        return Ok(false);
    }

    // Compare samples
    let samples1: Vec<f32> = reader1
        .into_samples::<i16>()
        .map(|s| s.map(|v| v as f32 / 32768.0))
        .collect::<Result<Vec<_>, _>>()?;

    let samples2: Vec<f32> = reader2
        .into_samples::<i16>()
        .map(|s| s.map(|v| v as f32 / 32768.0))
        .collect::<Result<Vec<_>, _>>()?;

    if samples1.len() != samples2.len() {
        return Ok(false);
    }

    // Calculate RMS difference
    let mut sum_diff = 0.0;
    for (s1, s2) in samples1.iter().zip(samples2.iter()) {
        let diff = s1 - s2;
        sum_diff += diff * diff;
    }
    let rms_diff = (sum_diff / samples1.len() as f32).sqrt();

    Ok(rms_diff < tolerance)
}

#[test]
fn test_multiply_matches_cdp() {
    let dir = tempdir().unwrap();
    let input_path = dir.path().join("input.wav");
    let cdp_output = dir.path().join("cdp_output.wav");
    let rust_output = dir.path().join("rust_output.wav");

    // Create test input
    create_test_wav(&input_path).unwrap();

    // Run CDP distort multiply
    let status = Command::new("distort")
        .args([
            "multiply",
            input_path.to_str().unwrap(),
            cdp_output.to_str().unwrap(),
            "2", // multiply factor
        ])
        .status();

    if status.is_err() {
        eprintln!("CDP distort not found, skipping oracle test");
        return;
    }

    // Run Rust implementation
    multiply(&input_path, &rust_output, 2.0, 1.0).unwrap();

    // Compare outputs
    assert!(
        compare_wav_files(&cdp_output, &rust_output, 0.05).unwrap(),
        "Multiply output differs from CDP"
    );
}

#[test]
fn test_divide_matches_cdp() {
    let dir = tempdir().unwrap();
    let input_path = dir.path().join("input.wav");
    let cdp_output = dir.path().join("cdp_output.wav");
    let rust_output = dir.path().join("rust_output.wav");

    // Create test input
    create_test_wav(&input_path).unwrap();

    // Run CDP distort divide
    let status = Command::new("distort")
        .args([
            "divide",
            input_path.to_str().unwrap(),
            cdp_output.to_str().unwrap(),
            "2", // divide factor
        ])
        .status();

    if status.is_err() {
        eprintln!("CDP distort not found, skipping oracle test");
        return;
    }

    // Run Rust implementation
    divide(&input_path, &rust_output, 2, 1.0).unwrap();

    // Compare outputs
    assert!(
        compare_wav_files(&cdp_output, &rust_output, 0.05).unwrap(),
        "Divide output differs from CDP"
    );
}

#[test]
fn test_overload_matches_cdp() {
    let dir = tempdir().unwrap();
    let input_path = dir.path().join("input.wav");
    let cdp_output = dir.path().join("cdp_output.wav");
    let rust_output = dir.path().join("rust_output.wav");

    // Create test input
    create_test_wav(&input_path).unwrap();

    // Run CDP distort overload
    let status = Command::new("distort")
        .args([
            "overload",
            "1", // mode 1 = clipping
            input_path.to_str().unwrap(),
            cdp_output.to_str().unwrap(),
            "0.5", // clip level
        ])
        .status();

    if status.is_err() {
        eprintln!("CDP distort not found, skipping oracle test");
        return;
    }

    // Run Rust implementation (CDP mode 1 is similar to our hard clip)
    overload(&input_path, &rust_output, 0.5, 1.0, ClipType::Hard).unwrap();

    // Compare outputs
    assert!(
        compare_wav_files(&cdp_output, &rust_output, 0.05).unwrap(),
        "Overload output differs from CDP"
    );
}

#[test]
fn test_multiply_with_mix() {
    let dir = tempdir().unwrap();
    let input_path = dir.path().join("input.wav");
    let cdp_output = dir.path().join("cdp_output.wav");
    let rust_output = dir.path().join("rust_output.wav");

    // Create test input
    create_test_wav(&input_path).unwrap();

    // Run CDP distort multiply with prescale (similar to mix)
    let status = Command::new("distort")
        .args([
            "multiply",
            input_path.to_str().unwrap(),
            cdp_output.to_str().unwrap(),
            "4",
            "-p0.5", // prescale = 0.5
        ])
        .status();

    if status.is_err() {
        eprintln!("CDP distort not found, skipping oracle test");
        return;
    }

    // Run Rust implementation
    multiply(&input_path, &rust_output, 4.0, 0.5).unwrap();

    // Compare outputs (higher tolerance for complex operations)
    assert!(
        compare_wav_files(&cdp_output, &rust_output, 0.1).unwrap(),
        "Multiply with mix output differs from CDP"
    );
}

#[test]
fn test_distort_chain() {
    let dir = tempdir().unwrap();
    let input_path = dir.path().join("input.wav");
    let temp_path = dir.path().join("temp.wav");
    let cdp_output = dir.path().join("cdp_output.wav");
    let rust_output = dir.path().join("rust_output.wav");

    // Create test input
    create_test_wav(&input_path).unwrap();

    // CDP chain: multiply then overload
    let status = Command::new("distort")
        .args([
            "multiply",
            input_path.to_str().unwrap(),
            temp_path.to_str().unwrap(),
            "2",
        ])
        .status();

    if status.is_err() {
        eprintln!("CDP distort not found, skipping oracle test");
        return;
    }

    Command::new("distort")
        .args([
            "overload",
            "2", // mode 2 = soft clip
            temp_path.to_str().unwrap(),
            cdp_output.to_str().unwrap(),
            "0.7",
        ])
        .status()
        .unwrap();

    // Rust chain
    multiply(&input_path, &temp_path, 2.0, 1.0).unwrap();
    overload(&temp_path, &rust_output, 0.7, 1.0, ClipType::Soft).unwrap();

    // Compare outputs
    assert!(
        compare_wav_files(&cdp_output, &rust_output, 0.1).unwrap(),
        "Distortion chain output differs from CDP"
    );
}
