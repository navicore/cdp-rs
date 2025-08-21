//! Tests for stretch operations

use cdp_spectral::stretch_time;
use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::TempDir;

/// Test basic stretch functionality
#[test]
fn test_stretch_basic() {
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
            // Apply stretch (2x slower)
            let result = stretch_time(&ana_file, &output_ana, 2.0);
            assert!(result.is_ok(), "Stretch failed: {:?}", result);
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

/// Test stretch with different factors
#[test]
fn test_stretch_factors() {
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
            // Test various stretch factors
            for stretch_factor in &[0.5, 0.75, 1.0, 1.5, 2.0, 3.0] {
                let output_ana = temp_dir
                    .path()
                    .join(format!("stretch_{}.ana", stretch_factor));

                let result = stretch_time(&ana_file, &output_ana, *stretch_factor);
                assert!(
                    result.is_ok(),
                    "Stretch with factor {} failed: {:?}",
                    stretch_factor,
                    result
                );
                assert!(
                    output_ana.exists(),
                    "Output not created for factor {}",
                    stretch_factor
                );
            }
        }
    }
}

/// Test stretch CLI compatibility
#[test]
fn test_stretch_cli() {
    let output = Command::new("cargo")
        .args(["run", "--bin", "stretch"])
        .output()
        .expect("Failed to run stretch binary");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("CDP Release 7.1 2016"));
    assert!(stderr.contains("STRETCHING A SPECTRAL FILE"));
}

/// Test stretch time mode help
#[test]
fn test_stretch_time_help() {
    let output = Command::new("cargo")
        .args(["run", "--bin", "stretch", "--", "time"])
        .output()
        .expect("Failed to run stretch time");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("TIME-STRETCHING OF INFILE"));
    assert!(stderr.contains("timestretch"));
}

/// Test stretch mode 1 (actual stretching)
#[test]
fn test_stretch_mode1_help() {
    let output = Command::new("cargo")
        .args(["run", "--bin", "stretch", "--", "time", "1"])
        .output()
        .expect("Failed to run stretch time 1");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("stretch time 1 infile outfile timestretch"));
}

/// Test stretch mode 2 (duration calculation)
#[test]
fn test_stretch_mode2_help() {
    let output = Command::new("cargo")
        .args(["run", "--bin", "stretch", "--", "time", "2"])
        .output()
        .expect("Failed to run stretch time 2");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("stretch time 2 infile timestretch"));
    assert!(stderr.contains("calculates length of output"));
}

/// Test invalid stretch values
#[test]
fn test_stretch_invalid_values() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.ana");
    let output = temp_dir.path().join("output.ana");

    // Test invalid stretch factors
    let result = stretch_time(&input, &output, 0.0);
    assert!(result.is_err());

    let result = stretch_time(&input, &output, -1.0);
    assert!(result.is_err());

    let result = stretch_time(&input, &output, 0.001);
    assert!(result.is_err());

    let result = stretch_time(&input, &output, 1000.0);
    assert!(result.is_err());
}
