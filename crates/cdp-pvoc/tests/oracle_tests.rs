//! Oracle tests comparing our implementation against CDP

use cdp_oracle::test_utils::cdp_command;
use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::TempDir;

/// Compare our pvoc anal output with CDP's
#[test]
fn test_pvoc_anal_matches_cdp() {
    let temp_dir = TempDir::new().unwrap();
    let input_wav = temp_dir.path().join("input.wav");
    let our_ana = temp_dir.path().join("our.ana");
    let cdp_ana = temp_dir.path().join("cdp.ana");

    // Generate test input
    generate_test_wav(&input_wav);

    // Run our pvoc
    let our_result = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "pvoc",
            "--",
            "anal",
            "1",
            input_wav.to_str().unwrap(),
            our_ana.to_str().unwrap(),
            "-c1024",
            "-o3",
        ])
        .output()
        .expect("Failed to run our pvoc");

    assert!(our_result.status.success(), "Our pvoc failed");

    // Run CDP pvoc
    let cdp_result = cdp_command("pvoc")
        .args([
            "anal",
            "1",
            input_wav.to_str().unwrap(),
            cdp_ana.to_str().unwrap(),
            "-c1024",
            "-o3",
        ])
        .output()
        .expect("Failed to run CDP pvoc");

    assert!(cdp_result.status.success(), "CDP pvoc failed");

    // Compare outputs (ignoring timestamps)
    assert!(
        compare_ana_files(&our_ana, &cdp_ana),
        "Output files don't match"
    );
}

/// Test round-trip: anal -> synth should reconstruct audio
#[test]
fn test_pvoc_roundtrip() {
    let temp_dir = TempDir::new().unwrap();
    let input_wav = temp_dir.path().join("input.wav");
    let ana_file = temp_dir.path().join("test.ana");
    let output_wav = temp_dir.path().join("output.wav");

    // Generate test input
    generate_test_wav(&input_wav);

    // Analysis
    let anal_result = Command::new("cargo")
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

    assert!(anal_result.status.success(), "pvoc anal failed");
    assert!(ana_file.exists(), "Analysis file not created");

    // Synthesis
    let synth_result = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "pvoc",
            "--",
            "synth",
            ana_file.to_str().unwrap(),
            output_wav.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to run pvoc synth");

    assert!(synth_result.status.success(), "pvoc synth failed");
    assert!(output_wav.exists(), "Output file not created");

    // Compare audio quality (allowing for some phase vocoder artifacts)
    let input_size = fs::metadata(&input_wav).unwrap().len();
    let output_size = fs::metadata(&output_wav).unwrap().len();

    // Sizes should be similar (within 10%)
    let size_ratio = output_size as f64 / input_size as f64;
    assert!(
        size_ratio > 0.9 && size_ratio < 1.1,
        "Output size significantly different from input"
    );
}

/// Test that our pvoc handles various FFT sizes correctly
#[test]
fn test_pvoc_anal_fft_sizes() {
    let temp_dir = TempDir::new().unwrap();
    let input_wav = temp_dir.path().join("input.wav");

    generate_test_wav(&input_wav);

    for fft_size in &[64, 128, 256, 512, 1024, 2048, 4096] {
        let output_ana = temp_dir.path().join(format!("test_{}.ana", fft_size));

        let result = Command::new("cargo")
            .args([
                "run",
                "--bin",
                "pvoc",
                "--",
                "anal",
                "1",
                input_wav.to_str().unwrap(),
                output_ana.to_str().unwrap(),
                &format!("-c{}", fft_size),
            ])
            .output()
            .expect("Failed to run pvoc");

        assert!(result.status.success(), "Failed with FFT size {}", fft_size);
        assert!(
            output_ana.exists(),
            "Output not created for FFT size {}",
            fft_size
        );
    }
}

/// Test different overlap factors
#[test]
fn test_pvoc_anal_overlap_factors() {
    let temp_dir = TempDir::new().unwrap();
    let input_wav = temp_dir.path().join("input.wav");

    generate_test_wav(&input_wav);

    for overlap in 1..=4 {
        let output_ana = temp_dir.path().join(format!("test_o{}.ana", overlap));

        let result = Command::new("cargo")
            .args([
                "run",
                "--bin",
                "pvoc",
                "--",
                "anal",
                "1",
                input_wav.to_str().unwrap(),
                output_ana.to_str().unwrap(),
                &format!("-o{}", overlap),
            ])
            .output()
            .expect("Failed to run pvoc");

        assert!(result.status.success(), "Failed with overlap {}", overlap);
        assert!(
            output_ana.exists(),
            "Output not created for overlap {}",
            overlap
        );
    }
}

/// Helper: Generate a simple test WAV file
fn generate_test_wav(path: &Path) {
    // Use CDP housekeep example or simple Python script
    Command::new("python3")
        .args([
            "../../scripts/generate-test-audio.py",
            path.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to generate test WAV");
}

/// Helper: Compare two .ana files ignoring timestamps
fn compare_ana_files(file1: &Path, file2: &Path) -> bool {
    let data1 = fs::read(file1).expect("Failed to read file1");
    let data2 = fs::read(file2).expect("Failed to read file2");

    // Basic size check
    if data1.len() != data2.len() {
        return false;
    }

    // Compare headers (first 12 bytes should match)
    if data1[0..12] != data2[0..12] {
        return false;
    }

    // Find and compare fmt chunks
    let fmt1_pos = find_chunk(&data1, b"fmt ").expect("fmt chunk not found in file1");
    let fmt2_pos = find_chunk(&data2, b"fmt ").expect("fmt chunk not found in file2");

    // fmt chunks should be identical
    if data1[fmt1_pos..fmt1_pos + 24] != data2[fmt2_pos..fmt2_pos + 24] {
        return false;
    }

    // Find data chunks and compare sizes
    let data1_pos = find_chunk(&data1, b"data").expect("data chunk not found in file1");
    let data2_pos = find_chunk(&data2, b"data").expect("data chunk not found in file2");

    let data1_size = u32::from_le_bytes([
        data1[data1_pos + 4],
        data1[data1_pos + 5],
        data1[data1_pos + 6],
        data1[data1_pos + 7],
    ]);

    let data2_size = u32::from_le_bytes([
        data2[data2_pos + 4],
        data2[data2_pos + 5],
        data2[data2_pos + 6],
        data2[data2_pos + 7],
    ]);

    if data1_size != data2_size {
        return false;
    }

    // For spectral data, we need to allow small floating-point differences
    let start1 = data1_pos + 8;
    let start2 = data2_pos + 8;

    for i in (0..data1_size as usize).step_by(4) {
        let val1 = f32::from_le_bytes([
            data1[start1 + i],
            data1[start1 + i + 1],
            data1[start1 + i + 2],
            data1[start1 + i + 3],
        ]);

        let val2 = f32::from_le_bytes([
            data2[start2 + i],
            data2[start2 + i + 1],
            data2[start2 + i + 2],
            data2[start2 + i + 3],
        ]);

        // Allow small differences due to floating-point computation
        if (val1 - val2).abs() > 1e-6 {
            return false;
        }
    }

    true
}

/// Helper function to find a chunk in WAV file
fn find_chunk(buffer: &[u8], chunk_id: &[u8; 4]) -> Option<usize> {
    (0..buffer.len() - 4).find(|&i| &buffer[i..i + 4] == chunk_id)
}
