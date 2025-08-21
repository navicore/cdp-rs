//! Tests to ensure CLI compatibility with CDP pvoc

use serial_test::serial;
use std::process::Command;

/// Test that our pvoc binary exists and runs
#[test]
#[serial]
fn test_pvoc_binary_exists() {
    let output = Command::new("cargo")
        .args(["run", "--bin", "pvoc", "--", "--help"])
        .output()
        .expect("Failed to run pvoc binary");

    // Should exit with error (1) when given --help (unknown command)
    assert_ne!(output.status.code(), Some(0));

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("CDP Release 7.1 2016"));
    assert!(stderr.contains("USAGE: pvoc NAME"));
}

/// Test that pvoc without arguments shows correct usage
#[test]
#[serial]
fn test_pvoc_no_args_shows_usage() {
    let output = Command::new("cargo")
        .args(["run", "--bin", "pvoc"])
        .output()
        .expect("Failed to run pvoc");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("CDP Release 7.1 2016"));
    assert!(stderr.contains("anal   synth"));
    assert!(stderr.contains("extract"));
}

/// Test that pvoc anal without arguments shows correct help
#[test]
#[serial]
fn test_pvoc_anal_help() {
    let output = Command::new("cargo")
        .args(["run", "--bin", "pvoc", "--", "anal"])
        .output()
        .expect("Failed to run pvoc anal");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("CONVERT SOUNDFILE TO SPECTRAL FILE"));
    assert!(stderr.contains("USAGE: pvoc anal  mode infile outfile"));
    assert!(stderr.contains("1) STANDARD ANALYSIS"));
    assert!(stderr.contains("2) OUTPUT SPECTRAL ENVELOPE VALS ONLY"));
    assert!(stderr.contains("3) OUTPUT SPECTRAL MAGNITUDE VALS ONLY"));
    assert!(stderr.contains("POINTS"));
    assert!(stderr.contains("OVERLAP"));
}

/// Test that pvoc synth without arguments shows correct help
#[test]
#[serial]
fn test_pvoc_synth_help() {
    let output = Command::new("cargo")
        .args(["run", "--bin", "pvoc", "--", "synth"])
        .output()
        .expect("Failed to run pvoc synth");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("CONVERT SPECTRAL FILE TO SOUNDFILE"));
    assert!(stderr.contains("USAGE: pvoc synth infile outfile"));
}

/// Test that pvoc extract without arguments shows correct help
#[test]
#[serial]
fn test_pvoc_extract_help() {
    let output = Command::new("cargo")
        .args(["run", "--bin", "pvoc", "--", "extract"])
        .output()
        .expect("Failed to run pvoc extract");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("EXTRACT FREQUENCY BAND"));
    assert!(stderr.contains("USAGE: pvoc extract infile outfile lo_freq hi_freq"));
}

/// Test that invalid mode is rejected
#[test]
#[serial]
fn test_pvoc_anal_invalid_mode() {
    let output = Command::new("cargo")
        .args([
            "run", "--bin", "pvoc", "--", "anal", "5", "in.wav", "out.ana",
        ])
        .output()
        .expect("Failed to run pvoc");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("ERROR"));
    assert!(stderr.contains("Invalid mode"));
}

/// Test that invalid channel count is rejected
#[test]
#[serial]
fn test_pvoc_anal_invalid_channels() {
    let output = Command::new("cargo")
        .args([
            "run", "--bin", "pvoc", "--", "anal", "1", "in.wav", "out.ana", "-c1023",
        ])
        .output()
        .expect("Failed to run pvoc");

    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("ERROR"));
    assert!(stderr.contains("power of 2"));
}

/// Test that valid channel counts are accepted (even if analysis fails)
#[test]
#[serial]
fn test_pvoc_anal_valid_channels() {
    // These should parse correctly even if the analysis fails
    for channels in &[
        "2", "4", "8", "16", "32", "64", "128", "256", "512", "1024", "2048",
    ] {
        let output = Command::new("cargo")
            .args([
                "run",
                "--bin",
                "pvoc",
                "--",
                "anal",
                "1",
                "in.wav",
                "out.ana",
                &format!("-c{}", channels),
            ])
            .output()
            .expect("Failed to run pvoc");

        let stderr = String::from_utf8_lossy(&output.stderr);
        // Should get to "analysis/synthesis beginning" before failing on missing input
        assert!(
            stderr.contains("analysis/synthesis beginning")
                || stderr.contains("not yet implemented")
        );
    }
}
