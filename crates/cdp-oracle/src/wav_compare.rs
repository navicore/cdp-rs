//! Smart WAV file comparison for CDP oracle tests
//!
//! Compares WAV files while accounting for expected differences like timestamps

use std::fs::File;
use std::io::{self, Read, Seek, SeekFrom};
use std::path::Path;

#[derive(Debug)]
pub struct WavChunk {
    pub id: [u8; 4],
    pub size: u32,
    pub offset: u64, // Position in file where data starts
}

#[derive(Debug)]
pub struct WavComparison {
    pub format_matches: bool,
    pub data_matches: bool,
    pub peak_matches: bool, // Ignoring timestamp
    pub chunks_match: bool,
    pub details: String,
}

/// Compare two WAV files intelligently
pub fn compare_wav_files(file1: &Path, file2: &Path) -> io::Result<WavComparison> {
    let mut f1 = File::open(file1)?;
    let mut f2 = File::open(file2)?;

    let chunks1 = read_chunks(&mut f1)?;
    let chunks2 = read_chunks(&mut f2)?;

    let mut comparison = WavComparison {
        format_matches: false,
        data_matches: false,
        peak_matches: false,
        chunks_match: false,
        details: String::new(),
    };

    // Check if both have the same chunks (by type, not necessarily same order)
    comparison.chunks_match = have_same_chunk_types(&chunks1, &chunks2);
    if !comparison.chunks_match {
        comparison
            .details
            .push_str("Different chunk types present\n");
    }

    // Compare fmt chunks
    if let (Some(fmt1), Some(fmt2)) = (find_chunk(&chunks1, b"fmt "), find_chunk(&chunks2, b"fmt "))
    {
        comparison.format_matches = compare_fmt_chunk(&mut f1, fmt1, &mut f2, fmt2)?;
        if !comparison.format_matches {
            comparison.details.push_str("Format chunks differ\n");
        }
    }

    // Compare data chunks
    if let (Some(data1), Some(data2)) =
        (find_chunk(&chunks1, b"data"), find_chunk(&chunks2, b"data"))
    {
        comparison.data_matches = compare_data_chunk(&mut f1, data1, &mut f2, data2)?;
        if !comparison.data_matches {
            comparison.details.push_str("Audio data differs\n");
        }
    }

    // Compare PEAK chunks (ignoring timestamp)
    if let (Some(peak1), Some(peak2)) =
        (find_chunk(&chunks1, b"PEAK"), find_chunk(&chunks2, b"PEAK"))
    {
        comparison.peak_matches = compare_peak_chunk(&mut f1, peak1, &mut f2, peak2)?;
        if !comparison.peak_matches {
            comparison
                .details
                .push_str("PEAK values differ (not timestamp)\n");
        }
    } else if find_chunk(&chunks1, b"PEAK").is_some() || find_chunk(&chunks2, b"PEAK").is_some() {
        comparison.details.push_str("One file missing PEAK chunk\n");
    }

    if comparison.details.is_empty() {
        comparison.details = "Files match (ignoring timestamps)".to_string();
    }

    Ok(comparison)
}

fn read_chunks(file: &mut File) -> io::Result<Vec<WavChunk>> {
    let mut chunks = Vec::new();
    let mut header = [0u8; 12];

    file.read_exact(&mut header)?;

    // Skip RIFF header validation - assume it's valid

    loop {
        let mut chunk_header = [0u8; 8];
        let pos = file.stream_position()?;

        if file.read_exact(&mut chunk_header).is_err() {
            break;
        }

        let size = u32::from_le_bytes([
            chunk_header[4],
            chunk_header[5],
            chunk_header[6],
            chunk_header[7],
        ]);

        chunks.push(WavChunk {
            id: [
                chunk_header[0],
                chunk_header[1],
                chunk_header[2],
                chunk_header[3],
            ],
            size,
            offset: pos + 8, // After the chunk header
        });

        // Skip to next chunk
        let skip_amount = if size % 2 == 0 { size } else { size + 1 };
        file.seek(SeekFrom::Current(skip_amount as i64))?;
    }

    Ok(chunks)
}

fn have_same_chunk_types(chunks1: &[WavChunk], chunks2: &[WavChunk]) -> bool {
    let mut types1: Vec<[u8; 4]> = chunks1.iter().map(|c| c.id).collect();
    let mut types2: Vec<[u8; 4]> = chunks2.iter().map(|c| c.id).collect();

    types1.sort();
    types2.sort();

    types1 == types2
}

fn find_chunk<'a>(chunks: &'a [WavChunk], id: &[u8; 4]) -> Option<&'a WavChunk> {
    chunks.iter().find(|c| &c.id == id)
}

fn compare_fmt_chunk(
    f1: &mut File,
    chunk1: &WavChunk,
    f2: &mut File,
    chunk2: &WavChunk,
) -> io::Result<bool> {
    if chunk1.size != chunk2.size {
        return Ok(false);
    }

    let mut data1 = vec![0u8; chunk1.size as usize];
    let mut data2 = vec![0u8; chunk2.size as usize];

    f1.seek(SeekFrom::Start(chunk1.offset))?;
    f1.read_exact(&mut data1)?;

    f2.seek(SeekFrom::Start(chunk2.offset))?;
    f2.read_exact(&mut data2)?;

    Ok(data1 == data2)
}

fn compare_data_chunk(
    f1: &mut File,
    chunk1: &WavChunk,
    f2: &mut File,
    chunk2: &WavChunk,
) -> io::Result<bool> {
    if chunk1.size != chunk2.size {
        return Ok(false);
    }

    // Compare in blocks to avoid loading huge files into memory
    const BLOCK_SIZE: usize = 8192;
    let mut buf1 = vec![0u8; BLOCK_SIZE];
    let mut buf2 = vec![0u8; BLOCK_SIZE];

    f1.seek(SeekFrom::Start(chunk1.offset))?;
    f2.seek(SeekFrom::Start(chunk2.offset))?;

    let mut remaining = chunk1.size as usize;

    while remaining > 0 {
        let to_read = remaining.min(BLOCK_SIZE);
        f1.read_exact(&mut buf1[..to_read])?;
        f2.read_exact(&mut buf2[..to_read])?;

        if buf1[..to_read] != buf2[..to_read] {
            return Ok(false);
        }

        remaining -= to_read;
    }

    Ok(true)
}

fn compare_peak_chunk(
    f1: &mut File,
    chunk1: &WavChunk,
    f2: &mut File,
    chunk2: &WavChunk,
) -> io::Result<bool> {
    if chunk1.size != chunk2.size {
        return Ok(false);
    }

    // PEAK chunk structure:
    // 4 bytes: version
    // 4 bytes: timestamp (IGNORE THIS)
    // For each channel:
    //   4 bytes: float peak value
    //   4 bytes: position

    let mut data1 = vec![0u8; chunk1.size as usize];
    let mut data2 = vec![0u8; chunk2.size as usize];

    f1.seek(SeekFrom::Start(chunk1.offset))?;
    f1.read_exact(&mut data1)?;

    f2.seek(SeekFrom::Start(chunk2.offset))?;
    f2.read_exact(&mut data2)?;

    // Compare version (first 4 bytes)
    if data1[0..4] != data2[0..4] {
        return Ok(false);
    }

    // Skip timestamp comparison (bytes 4-7)

    // Compare peak data (bytes 8 onwards)
    if data1[8..] != data2[8..] {
        return Ok(false);
    }

    Ok(true)
}

/// Check if a file has CDP-compatible format
pub fn has_cdp_format(file_path: &Path) -> io::Result<bool> {
    let mut file = File::open(file_path)?;
    let chunks = read_chunks(&mut file)?;

    // CDP files should have: fmt, PEAK, cue, LIST, data
    let has_peak = find_chunk(&chunks, b"PEAK").is_some();
    let has_cue = find_chunk(&chunks, b"cue ").is_some();
    let has_list = find_chunk(&chunks, b"LIST").is_some();

    Ok(has_peak && has_cue && has_list)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cdp_format_check() {
        // Test with the files we know CDP created
        if Path::new("cdp_copy_output.wav").exists() {
            let result = has_cdp_format(Path::new("cdp_copy_output.wav"));
            assert!(result.unwrap(), "CDP output should have CDP format");
        }
    }
}
