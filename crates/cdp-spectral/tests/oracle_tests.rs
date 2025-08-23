//! Oracle tests comparing our spectral implementations against CDP

use std::fs;
use std::path::Path;
use cdp_oracle::test_utils::cdp_command;
use std::process::Command;
use tempfile::TempDir;

/// Compare our blur output with CDP's blur
#[test]
#[ignore] // TODO: Enable when module is implemented
fn test_blur_matches_cdp() {
    let temp_dir = TempDir::new().unwrap();
    let input_wav = temp_dir.path().join("input.wav");
    let ana_file = temp_dir.path().join("input.ana");
    let our_blur = temp_dir.path().join("our_blur.ana");
    let cdp_blur = temp_dir.path().join("cdp_blur.ana");

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
    if !sample_path.exists() {
        eprintln!("Sample file not found, skipping oracle test");
        return;
    }

    fs::copy(sample_path, &input_wav).expect("Failed to copy sample");

    // Convert to .ana using CDP pvoc
    let cdp_pvoc = cdp_command("pvoc")
        .args([
            "anal",
            "1",
            input_wav.to_str().unwrap(),
            ana_file.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to run CDP pvoc");

    if !cdp_pvoc.status.success() {
        eprintln!("CDP pvoc failed, skipping oracle test");
        return;
    }

    // Run our blur
    let our_result = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "blur",
            "--",
            "blur",
            ana_file.to_str().unwrap(),
            our_blur.to_str().unwrap(),
            "5",
        ])
        .output()
        .expect("Failed to run our blur");

    assert!(our_result.status.success(), "Our blur failed");

    // Run CDP blur
    let cdp_result = cdp_command("blur")
        .args([
            "blur",
            ana_file.to_str().unwrap(),
            cdp_blur.to_str().unwrap(),
            "5",
        ])
        .output()
        .expect("Failed to run CDP blur");

    assert!(cdp_result.status.success(), "CDP blur failed");

    // Compare outputs
    assert!(
        compare_ana_files(&our_blur, &cdp_blur),
        "Blur outputs don't match CDP"
    );
}

/// Test blur with various window counts against CDP
#[test]
#[ignore] // TODO: Enable when module is implemented
fn test_blur_window_counts_oracle() {
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
    if !sample_path.exists() {
        eprintln!("Sample file not found, skipping oracle test");
        return;
    }

    fs::copy(sample_path, &input_wav).expect("Failed to copy sample");

    // Convert to .ana
    let cdp_pvoc = cdp_command("pvoc")
        .args([
            "anal",
            "1",
            input_wav.to_str().unwrap(),
            ana_file.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to run CDP pvoc");

    if !cdp_pvoc.status.success() {
        eprintln!("CDP pvoc failed, skipping oracle test");
        return;
    }

    // Test various blur window counts
    for blur_windows in &[1, 3, 5, 7, 11] {
        let our_blur = temp_dir
            .path()
            .join(format!("our_blur_{}.ana", blur_windows));
        let cdp_blur = temp_dir
            .path()
            .join(format!("cdp_blur_{}.ana", blur_windows));

        // Run our blur
        let our_result = Command::new("cargo")
            .args([
                "run",
                "--bin",
                "blur",
                "--",
                "blur",
                ana_file.to_str().unwrap(),
                our_blur.to_str().unwrap(),
                &blur_windows.to_string(),
            ])
            .output()
            .expect("Failed to run our blur");

        assert!(
            our_result.status.success(),
            "Our blur failed with {} windows",
            blur_windows
        );

        // Run CDP blur
        let cdp_result = cdp_command("blur")
            .args([
                "blur",
                ana_file.to_str().unwrap(),
                cdp_blur.to_str().unwrap(),
                &blur_windows.to_string(),
            ])
            .output()
            .expect("Failed to run CDP blur");

        assert!(
            cdp_result.status.success(),
            "CDP blur failed with {} windows",
            blur_windows
        );

        // Compare outputs
        assert!(
            compare_ana_files(&our_blur, &cdp_blur),
            "Blur with {} windows doesn't match CDP",
            blur_windows
        );
    }
}

/// Helper: Compare two .ana files ignoring timestamps
fn compare_ana_files(file1: &Path, file2: &Path) -> bool {
    let data1 = fs::read(file1).expect("Failed to read file1");
    let data2 = fs::read(file2).expect("Failed to read file2");

    // Basic size check
    if data1.len() != data2.len() {
        eprintln!("File sizes differ: {} vs {}", data1.len(), data2.len());
        return false;
    }

    // Compare headers (first 12 bytes should match)
    if data1[0..12] != data2[0..12] {
        eprintln!("RIFF headers don't match");
        return false;
    }

    // Find and compare fmt chunks
    let fmt1_pos = find_chunk(&data1, b"fmt ").expect("fmt chunk not found in file1");
    let fmt2_pos = find_chunk(&data2, b"fmt ").expect("fmt chunk not found in file2");

    // fmt chunks should be identical
    if data1[fmt1_pos..fmt1_pos + 24] != data2[fmt2_pos..fmt2_pos + 24] {
        eprintln!("fmt chunks don't match");
        return false;
    }

    // Find data chunks and compare
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
        eprintln!("Data chunk sizes differ");
        return false;
    }

    // Compare spectral data with tolerance for floating-point differences
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
        let tolerance = 1e-5;
        if (val1 - val2).abs() > tolerance {
            eprintln!(
                "Values differ at offset {}: {} vs {} (diff: {})",
                i,
                val1,
                val2,
                (val1 - val2).abs()
            );
            return false;
        }
    }

    true
}

/// Test stretch against CDP
#[test]
#[ignore] // TODO: Enable when module is implemented
fn test_stretch_matches_cdp() {
    let temp_dir = TempDir::new().unwrap();
    let input_wav = temp_dir.path().join("input.wav");
    let ana_file = temp_dir.path().join("input.ana");
    let our_stretch = temp_dir.path().join("our_stretch.ana");
    let cdp_stretch = temp_dir.path().join("cdp_stretch.ana");

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
    if !sample_path.exists() {
        eprintln!("Sample file not found, skipping oracle test");
        return;
    }

    fs::copy(sample_path, &input_wav).expect("Failed to copy sample");

    // Convert to .ana using CDP pvoc
    let cdp_pvoc = cdp_command("pvoc")
        .args([
            "anal",
            "1",
            input_wav.to_str().unwrap(),
            ana_file.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to run CDP pvoc");

    if !cdp_pvoc.status.success() {
        eprintln!("CDP pvoc failed, skipping oracle test");
        return;
    }

    // Run our stretch
    let our_result = Command::new("cargo")
        .args([
            "run",
            "--bin",
            "stretch",
            "--",
            "time",
            "1",
            ana_file.to_str().unwrap(),
            our_stretch.to_str().unwrap(),
            "2.0",
        ])
        .output()
        .expect("Failed to run our stretch");

    assert!(our_result.status.success(), "Our stretch failed");

    // Run CDP stretch
    let cdp_result = cdp_command("stretch")
        .args([
            "time",
            "1",
            ana_file.to_str().unwrap(),
            cdp_stretch.to_str().unwrap(),
            "2.0",
        ])
        .output()
        .expect("Failed to run CDP stretch");

    assert!(cdp_result.status.success(), "CDP stretch failed");

    // Compare outputs - stretch will have different phase accumulation,
    // so we can only check that sizes are similar
    let our_size = fs::metadata(&our_stretch).unwrap().len();
    let cdp_size = fs::metadata(&cdp_stretch).unwrap().len();

    // Sizes should be within 10% for same stretch factor
    let size_ratio = our_size as f64 / cdp_size as f64;
    assert!(
        size_ratio > 0.9 && size_ratio < 1.1,
        "Output sizes differ significantly: {} vs {}",
        our_size,
        cdp_size
    );
}

/// Test stretch with various factors against CDP
#[test]
#[ignore] // TODO: Enable when module is implemented
fn test_stretch_factors_oracle() {
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
    if !sample_path.exists() {
        eprintln!("Sample file not found, skipping oracle test");
        return;
    }

    fs::copy(sample_path, &input_wav).expect("Failed to copy sample");

    // Convert to .ana
    let cdp_pvoc = cdp_command("pvoc")
        .args([
            "anal",
            "1",
            input_wav.to_str().unwrap(),
            ana_file.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to run CDP pvoc");

    if !cdp_pvoc.status.success() {
        eprintln!("CDP pvoc failed, skipping oracle test");
        return;
    }

    // Test various stretch factors
    for stretch_factor in &[0.5, 1.0, 1.5, 2.0, 3.0] {
        let our_stretch = temp_dir
            .path()
            .join(format!("our_stretch_{}.ana", stretch_factor));
        let cdp_stretch = temp_dir
            .path()
            .join(format!("cdp_stretch_{}.ana", stretch_factor));

        // Run our stretch
        let our_result = Command::new("cargo")
            .args([
                "run",
                "--bin",
                "stretch",
                "--",
                "time",
                "1",
                ana_file.to_str().unwrap(),
                our_stretch.to_str().unwrap(),
                &stretch_factor.to_string(),
            ])
            .output()
            .expect("Failed to run our stretch");

        assert!(
            our_result.status.success(),
            "Our stretch failed with factor {}",
            stretch_factor
        );

        // Run CDP stretch
        let cdp_result = cdp_command("stretch")
            .args([
                "time",
                "1",
                ana_file.to_str().unwrap(),
                cdp_stretch.to_str().unwrap(),
                &stretch_factor.to_string(),
            ])
            .output()
            .expect("Failed to run CDP stretch");

        assert!(
            cdp_result.status.success(),
            "CDP stretch failed with factor {}",
            stretch_factor
        );

        // Check sizes are similar
        let our_size = fs::metadata(&our_stretch).unwrap().len();
        let cdp_size = fs::metadata(&cdp_stretch).unwrap().len();
        let size_ratio = our_size as f64 / cdp_size as f64;

        assert!(
            size_ratio > 0.9 && size_ratio < 1.1,
            "Stretch factor {}: sizes differ significantly: {} vs {}",
            stretch_factor,
            our_size,
            cdp_size
        );
    }
}

/// Helper function to find a chunk in WAV file
fn find_chunk(buffer: &[u8], chunk_id: &[u8; 4]) -> Option<usize> {
    (0..buffer.len() - 4).find(|&i| &buffer[i..i + 4] == chunk_id)
}
