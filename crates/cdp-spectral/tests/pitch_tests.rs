//! Tests for pitch shifting operations

use cdp_spectral::{factor_to_semitones, pitch_shift, semitones_to_factor};
use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::TempDir;

/// Test basic pitch functionality
#[test]
#[ignore] // TODO: Enable when module is implemented
fn test_pitch_basic() {
    let temp_dir = TempDir::new().unwrap();
    let input_wav = temp_dir.path().join("input.wav");
    let ana_file = temp_dir.path().join("input.ana");
    let output_ana = temp_dir.path().join("output.ana");
    let output_wav = temp_dir.path().join("output.wav");

    // Generate test input
    Command::new("cargo")
        .args([
            "run",
            "-p",
            "cdp-housekeep",
            "--example",
            "generate_samples",
        ])
        .output()
        .expect("Failed to generate samples");

    // Get a sample file
    let sample_path = Path::new("crates/cdp-housekeep/examples/sine_tone.wav");
    if sample_path.exists() {
        fs::copy(sample_path, &input_wav).expect("Failed to copy sample");

        // Convert to .ana file using pvoc
        let pvoc_result = Command::new("cargo")
            .args([
                "run",
                "--bin",
                "pvoc",
                "--",
                "anal",
                "1",
                input_wav.to_str().unwrap(),
                ana_file.to_str().unwrap(),
            ])
            .output()
            .expect("Failed to run pvoc anal");

        if pvoc_result.status.success() && ana_file.exists() {
            // Apply pitch shift (octave up)
            let result = pitch_shift(&ana_file, &output_ana, 2.0);
            assert!(result.is_ok(), "Pitch shift failed: {:?}", result);
            assert!(output_ana.exists(), "Output file not created");

            // Convert back to audio to verify
            let synth_result = Command::new("cargo")
                .args([
                    "run",
                    "--bin",
                    "pvoc",
                    "--",
                    "synth",
                    output_ana.to_str().unwrap(),
                    output_wav.to_str().unwrap(),
                ])
                .output()
                .expect("Failed to run pvoc synth");

            if synth_result.status.success() {
                assert!(output_wav.exists(), "Synthesized output not created");
            }
        }
    }
}

/// Test pitch with different shift factors
#[test]
#[ignore] // TODO: Enable when module is implemented
fn test_pitch_factors() {
    let temp_dir = TempDir::new().unwrap();
    let input_wav = temp_dir.path().join("input.wav");
    let ana_file = temp_dir.path().join("input.ana");

    // Generate test input
    Command::new("cargo")
        .args([
            "run",
            "-p",
            "cdp-housekeep",
            "--example",
            "generate_samples",
        ])
        .output()
        .expect("Failed to generate samples");

    let sample_path = Path::new("crates/cdp-housekeep/examples/sine_tone.wav");
    if sample_path.exists() {
        fs::copy(sample_path, &input_wav).expect("Failed to copy sample");

        // Convert to .ana file
        let pvoc_result = Command::new("cargo")
            .args([
                "run",
                "--bin",
                "pvoc",
                "--",
                "anal",
                "1",
                input_wav.to_str().unwrap(),
                ana_file.to_str().unwrap(),
            ])
            .output()
            .expect("Failed to run pvoc anal");

        if pvoc_result.status.success() && ana_file.exists() {
            // Test various pitch shift factors
            for shift_factor in &[0.5, 0.75, 1.0, 1.5, 2.0, 3.0] {
                let output_ana = temp_dir.path().join(format!("pitch_{}.ana", shift_factor));

                let result = pitch_shift(&ana_file, &output_ana, *shift_factor);
                assert!(
                    result.is_ok(),
                    "Pitch shift with factor {} failed: {:?}",
                    shift_factor,
                    result
                );
                assert!(
                    output_ana.exists(),
                    "Output not created for factor {}",
                    shift_factor
                );
            }
        }
    }
}

/// Test pitch CLI compatibility
#[test]
#[ignore] // TODO: Enable when module is implemented
fn test_pitch_cli() {
    let output = Command::new("cargo")
        .args(["run", "--bin", "pitch"])
        .output()
        .expect("Failed to run pitch binary");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("CDP-RS Pitch Shift"));
    assert!(stderr.contains("USAGE: pitch"));
}

/// Test semitone conversion
#[test]
#[ignore] // TODO: Enable when module is implemented
fn test_semitone_conversion() {
    // Octave up: 12 semitones = factor of 2
    let factor = semitones_to_factor(12.0);
    assert!((factor - 2.0).abs() < 1e-6);

    // Octave down: -12 semitones = factor of 0.5
    let factor = semitones_to_factor(-12.0);
    assert!((factor - 0.5).abs() < 1e-6);

    // Perfect fifth: 7 semitones
    let factor = semitones_to_factor(7.0);
    assert!((factor - 1.498307).abs() < 1e-5);

    // Test reverse conversion
    let semitones = factor_to_semitones(2.0);
    assert!((semitones - 12.0).abs() < 1e-6);

    let semitones = factor_to_semitones(0.5);
    assert!((semitones + 12.0).abs() < 1e-6);
}

/// Test invalid pitch values
#[test]
#[ignore] // TODO: Enable when module is implemented
fn test_pitch_invalid_values() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.ana");
    let output = temp_dir.path().join("output.ana");

    // Test invalid shift factors
    let result = pitch_shift(&input, &output, 0.0);
    assert!(result.is_err());

    let result = pitch_shift(&input, &output, -1.0);
    assert!(result.is_err());

    let result = pitch_shift(&input, &output, 0.05);
    assert!(result.is_err());

    let result = pitch_shift(&input, &output, 20.0);
    assert!(result.is_err());
}
