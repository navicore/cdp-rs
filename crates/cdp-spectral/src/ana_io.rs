//! Utilities for reading and writing CDP .ana files
//!
//! CDP .ana files are WAV files with IEEE float format and LIST chunk metadata.

use crate::error::{Result, SpectralError};
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::path::Path;

/// CDP .ana file header information
#[derive(Debug, Clone)]
pub struct AnaHeader {
    /// Sample rate of original file
    pub sample_rate: u32,
    /// Number of frequency channels
    pub channels: u16,
    /// Analysis window length
    pub window_len: u32,
    /// Decimation factor (hop size divisor)
    pub dec_factor: u32,
}

/// Read a CDP .ana file
pub fn read_ana_file(path: &Path) -> Result<(AnaHeader, Vec<f32>)> {
    let mut reader = BufReader::new(File::open(path)?);

    // Read RIFF header
    let mut header = [0u8; 12];
    reader.read_exact(&mut header)?;

    if &header[0..4] != b"RIFF" || &header[8..12] != b"WAVE" {
        return Err(SpectralError::InvalidInput(
            "Not a valid WAV file".to_string(),
        ));
    }

    let mut ana_header = AnaHeader {
        sample_rate: 44100,
        channels: 0,
        window_len: 0,
        dec_factor: 4,
    };

    let mut data_offset = 0u64;
    let mut data_size = 0u32;

    // Parse chunks
    loop {
        let mut chunk_header = [0u8; 8];
        if reader.read_exact(&mut chunk_header).is_err() {
            break;
        }

        let chunk_id = &chunk_header[0..4];
        let chunk_size = u32::from_le_bytes([
            chunk_header[4],
            chunk_header[5],
            chunk_header[6],
            chunk_header[7],
        ]);

        match chunk_id {
            b"fmt " => {
                let mut fmt_data = vec![0u8; chunk_size as usize];
                reader.read_exact(&mut fmt_data)?;

                // Parse format chunk
                let format_type = u16::from_le_bytes([fmt_data[0], fmt_data[1]]);
                if format_type != 3 {
                    // 3 = IEEE float
                    return Err(SpectralError::InvalidInput(
                        "Not IEEE float format".to_string(),
                    ));
                }

                ana_header.channels = u16::from_le_bytes([fmt_data[2], fmt_data[3]]);
                ana_header.sample_rate =
                    u32::from_le_bytes([fmt_data[4], fmt_data[5], fmt_data[6], fmt_data[7]]);
            }
            b"LIST" => {
                let mut list_type = [0u8; 4];
                reader.read_exact(&mut list_type)?;

                if &list_type == b"adtl" {
                    // Parse metadata
                    let mut metadata = vec![0u8; (chunk_size - 4) as usize];
                    reader.read_exact(&mut metadata)?;

                    // Extract window length and dec factor from metadata
                    let metadata_str = String::from_utf8_lossy(&metadata);
                    for line in metadata_str.lines() {
                        if let Some(rest) = line.strip_prefix("analwinlen: ") {
                            if let Ok(val) = rest.parse::<u32>() {
                                ana_header.window_len = val;
                            }
                        } else if let Some(rest) = line.strip_prefix("decfactor: ") {
                            if let Ok(val) = rest.parse::<u32>() {
                                ana_header.dec_factor = val;
                            }
                        }
                    }
                } else {
                    // Skip other LIST types
                    reader.seek(SeekFrom::Current((chunk_size - 4) as i64))?;
                }
            }
            b"data" => {
                data_offset = reader.stream_position()?;
                data_size = chunk_size;
                break;
            }
            _ => {
                // Skip unknown chunks
                reader.seek(SeekFrom::Current(chunk_size as i64))?;
            }
        }

        // Align to word boundary
        if chunk_size % 2 != 0 {
            reader.seek(SeekFrom::Current(1))?;
        }
    }

    // Validate that we have the required metadata
    if ana_header.window_len == 0 || ana_header.channels == 0 {
        return Err(SpectralError::InvalidInput(
            "Missing or invalid analysis metadata".to_string(),
        ));
    }

    // Read spectral data
    reader.seek(SeekFrom::Start(data_offset))?;
    let num_samples = data_size / 4; // 4 bytes per float
    let mut samples = Vec::with_capacity(num_samples as usize);

    for _ in 0..num_samples {
        let mut bytes = [0u8; 4];
        reader.read_exact(&mut bytes)?;
        samples.push(f32::from_le_bytes(bytes));
    }

    // Validate spectral data format (should be interleaved real/imaginary pairs)
    if samples.len() % 2 != 0 {
        return Err(SpectralError::InvalidInput(
            "Spectral data must contain real/imaginary pairs".to_string(),
        ));
    }

    // Validate that channels matches expected spectral format
    let expected_window_size = ana_header.channels as usize;
    if samples.len() % expected_window_size != 0 {
        return Err(SpectralError::InvalidInput(
            "Data size doesn't match channel count".to_string(),
        ));
    }

    Ok((ana_header, samples))
}

/// Write a CDP .ana file
pub fn write_ana_file(path: &Path, header: &AnaHeader, samples: &[f32]) -> Result<()> {
    let mut writer = BufWriter::new(File::create(path)?);

    // Calculate data size
    let data_size = (samples.len() * 4) as u32;

    // Create metadata
    let hop_size = header.window_len / header.dec_factor;
    let arate = header.sample_rate as f32 / hop_size as f32;
    let metadata = format!(
        "original sampsize: 16\n\
         original sample rate: {}\n\
         arate: {:.5}\n\
         analwinlen: {}\n\
         decfactor: {}\n\
         origrate: {}\n\
         DATE: CDP Phase Vocoder Analysis\n",
        header.sample_rate, arate, header.window_len, header.dec_factor, header.sample_rate
    );

    let list_data = metadata.as_bytes();
    let list_size = 4 + 4 + 4 + list_data.len(); // "adtl" + "note" + size + data
    let list_size_padded = if list_size % 2 == 0 {
        list_size
    } else {
        list_size + 1
    };

    // Calculate RIFF size
    let riff_size = 4 + // "WAVE"
        8 + 16 + // fmt chunk
        8 + list_size_padded as u32 + // LIST chunk
        8 + data_size; // data chunk

    // Write RIFF header
    writer.write_all(b"RIFF")?;
    writer.write_all(&riff_size.to_le_bytes())?;
    writer.write_all(b"WAVE")?;

    // Write fmt chunk (IEEE float format)
    writer.write_all(b"fmt ")?;
    writer.write_all(&16u32.to_le_bytes())?; // chunk size
    writer.write_all(&3u16.to_le_bytes())?; // format type 3 = IEEE float
    writer.write_all(&header.channels.to_le_bytes())?;
    writer.write_all(&header.sample_rate.to_le_bytes())?;
    let byte_rate = header.sample_rate * header.channels as u32 * 4; // 4 bytes per float
    writer.write_all(&byte_rate.to_le_bytes())?;
    let block_align = header.channels * 4;
    writer.write_all(&block_align.to_le_bytes())?;
    writer.write_all(&32u16.to_le_bytes())?; // bits per sample (32 for float)

    // Write LIST chunk
    writer.write_all(b"LIST")?;
    writer.write_all(&(list_size_padded as u32).to_le_bytes())?;
    writer.write_all(b"adtl")?;
    writer.write_all(b"note")?;
    writer.write_all(&(list_data.len() as u32).to_le_bytes())?;
    writer.write_all(list_data)?;
    if list_data.len() % 2 != 0 {
        writer.write_all(&[0u8])?; // padding
    }

    // Write data chunk
    writer.write_all(b"data")?;
    writer.write_all(&data_size.to_le_bytes())?;

    // Write spectral samples
    for &sample in samples {
        writer.write_all(&sample.to_le_bytes())?;
    }

    writer.flush()?;
    Ok(())
}
