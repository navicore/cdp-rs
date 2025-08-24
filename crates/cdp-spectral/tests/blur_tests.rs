//! Tests for blur operations

use cdp_spectral::blur;
use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::TempDir;

/// Test basic blur functionality
#[test]
#[ignore] // TODO: Enable when module is implemented
fn test_blur_basic() {
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
            // Apply blur
            let result = blur(&ana_file, &output_ana, 5);
            assert!(result.is_ok(), "Blur failed: {:?}", result);
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

/// Test blur with different window counts
#[test]
#[ignore] // TODO: Enable when module is implemented
fn test_blur_window_counts() {
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
            // Test various blur window counts
            for blur_windows in &[1, 3, 5, 7, 11, 21] {
                let output_ana = temp_dir.path().join(format!("blur_{}.ana", blur_windows));

                let result = blur(&ana_file, &output_ana, *blur_windows);
                assert!(
                    result.is_ok(),
                    "Blur with {} windows failed: {:?}",
                    blur_windows,
                    result
                );
                assert!(
                    output_ana.exists(),
                    "Output not created for {} windows",
                    blur_windows
                );
            }
        }
    }
}

/// Test blur CLI compatibility
#[test]
#[ignore] // TODO: Enable when module is implemented
fn test_blur_cli() {
    let output = Command::new("cargo")
        .args(["run", "--bin", "blur"])
        .output()
        .expect("Failed to run blur binary");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("CDP Release 7.1 2016"));
    assert!(stderr.contains("USAGE: blur NAME"));
}

/// Test blur blur mode help
#[test]
#[ignore] // TODO: Enable when module is implemented
fn test_blur_blur_help() {
    let output = Command::new("cargo")
        .args(["run", "--bin", "blur", "--", "blur"])
        .output()
        .expect("Failed to run blur blur");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("TIME-AVERAGE THE SPECTRUM"));
    assert!(stderr.contains("blurring"));
}

/// Test invalid blur values
#[test]
#[ignore] // TODO: Enable when module is implemented
fn test_blur_invalid_values() {
    let temp_dir = TempDir::new().unwrap();
    let input = temp_dir.path().join("input.ana");
    let output = temp_dir.path().join("output.ana");

    // Test zero blur windows
    let result = blur(&input, &output, 0);
    assert!(result.is_err());

    // Test with non-existent input
    let result = blur(&input, &output, 5);
    assert!(result.is_err());
}
