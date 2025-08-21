//! Tests to ensure .ana file format compatibility with CDP

use std::fs::File;
use std::io::Read;

/// CDP .ana files are WAV files with IEEE float format
#[test]
fn test_ana_is_valid_wav() {
    // When we create an .ana file, it should have valid WAV headers
    // This test will be implemented once pvoc_anal works
}

/// Verify the expected structure of a CDP .ana file
#[test]
#[ignore] // Will enable once we have test files
fn test_ana_file_structure() {
    // Expected structure:
    // 1. RIFF header
    // 2. fmt chunk with type 3 (IEEE float)
    // 3. LIST chunk with CDP metadata
    // 4. data chunk with float spectral data

    let mut file = File::open("test.ana").expect("Test file not found");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read file");

    // Check RIFF header
    assert_eq!(&buffer[0..4], b"RIFF");
    assert_eq!(&buffer[8..12], b"WAVE");

    // Check fmt chunk
    assert_eq!(&buffer[12..16], b"fmt ");

    // Format type should be 3 (IEEE float)
    assert_eq!(buffer[20], 3);
    assert_eq!(buffer[21], 0);

    // Should have LIST chunk with metadata
    let list_pos = find_chunk(&buffer, b"LIST");
    assert!(list_pos.is_some(), "LIST chunk not found");

    // Should have data chunk
    let data_pos = find_chunk(&buffer, b"data");
    assert!(data_pos.is_some(), "data chunk not found");
}

/// Test that metadata in LIST chunk matches CDP format
#[test]
#[ignore] // Will enable once we generate .ana files
fn test_ana_list_chunk_format() {
    let mut file = File::open("test.ana").expect("Test file not found");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read file");

    let list_pos = find_chunk(&buffer, b"LIST").expect("LIST chunk not found");
    let list_data = &buffer[list_pos + 8..]; // Skip "LIST" and size

    // Should start with "adtl"
    assert_eq!(&list_data[0..4], b"adtl");

    // Should have "note" subchunk
    assert_eq!(&list_data[4..8], b"note");

    // Note data should contain CDP metadata
    let note_start = 12; // After "adtl" and "note" header
    let note_str = std::str::from_utf8(&list_data[note_start..note_start + 100])
        .expect("Invalid UTF-8 in note");

    // Check for expected metadata fields
    assert!(note_str.contains("original sampsize"));
    assert!(note_str.contains("original sample rate"));
    assert!(note_str.contains("arate"));
    assert!(note_str.contains("analwinlen"));
    assert!(note_str.contains("decfactor"));
    assert!(note_str.contains("DATE"));
}

/// Test that spectral data is stored as IEEE float
#[test]
#[ignore] // Will enable once we generate .ana files
fn test_ana_data_is_float() {
    let mut file = File::open("test.ana").expect("Test file not found");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read file");

    let data_pos = find_chunk(&buffer, b"data").expect("data chunk not found");
    let data_size = u32::from_le_bytes([
        buffer[data_pos + 4],
        buffer[data_pos + 5],
        buffer[data_pos + 6],
        buffer[data_pos + 7],
    ]) as usize;

    // Data should be pairs of floats (magnitude, frequency)
    assert_eq!(
        data_size % 8,
        0,
        "Data size should be multiple of 8 bytes (float pairs)"
    );

    // Read first few floats to verify they're valid
    let data_start = data_pos + 8;
    for i in 0..10 {
        let float_bytes = [
            buffer[data_start + i * 4],
            buffer[data_start + i * 4 + 1],
            buffer[data_start + i * 4 + 2],
            buffer[data_start + i * 4 + 3],
        ];
        let value = f32::from_le_bytes(float_bytes);

        // Spectral data should be finite
        assert!(value.is_finite(), "Invalid float at position {}", i);
    }
}

/// Helper function to find a chunk in WAV file
fn find_chunk(buffer: &[u8], chunk_id: &[u8; 4]) -> Option<usize> {
    (0..buffer.len() - 4).find(|&i| &buffer[i..i + 4] == chunk_id)
}

/// Test the number of channels in .ana file
#[test]
#[ignore] // Will enable once we generate .ana files
fn test_ana_channel_count() {
    let mut file = File::open("test.ana").expect("Test file not found");
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).expect("Failed to read file");

    // Channel count is at offset 22-23 in fmt chunk
    let channels = u16::from_le_bytes([buffer[22], buffer[23]]);

    // CDP uses channel count = FFT_size/2 + 2
    // For default 1024 FFT: should be 514 channels
    assert!(channels > 0, "Channel count should be positive");
    assert!(channels <= 16384, "Channel count seems too large");

    // The actual channel count in fmt is different from spectral channels
    // CDP encodes as: channels = spectral_channels * 2 + 2
    // where spectral_channels = FFT_size / 2
}
