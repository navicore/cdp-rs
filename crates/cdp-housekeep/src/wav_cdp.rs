//! WAV file I/O with CDP-specific metadata
//!
//! Handles reading and writing WAV files with CDP's PEAK chunks,
//! cue points, and LIST metadata.

use super::Result;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

/// WAV format information
#[derive(Debug, Clone)]
pub struct WavFormat {
    pub channels: u16,
    pub sample_rate: u32,
    pub bits_per_sample: u16,
    pub data_size: u32,
}

/// CDP-specific PEAK chunk
#[derive(Debug, Clone)]
pub struct PeakChunk {
    pub version: u32,
    pub timestamp: u32,
    pub peak_value: f32,
    pub peak_position: u32,
}

#[derive(Debug, Clone)]
pub struct CuePoint {
    pub id: [u8; 4],
    pub position: u32,
    pub data_chunk_id: [u8; 4],
    pub chunk_start: u32,
    pub block_start: u32,
    pub sample_offset: u32,
}

#[derive(Debug, Clone)]
pub struct CueChunk {
    pub cue_points: Vec<CuePoint>,
}

#[derive(Debug, Clone)]
pub struct ListChunk {
    pub note_data: Vec<u8>,
}

/// CDP metadata chunks
#[derive(Debug, Clone)]
pub struct CdpChunks {
    pub peak: PeakChunk,
    pub cue: CueChunk,
    pub list: ListChunk,
}

/// Read a WAV file (basic version without CDP metadata)
pub fn read_wav_basic(input: &Path) -> io::Result<(WavFormat, Vec<i16>)> {
    let mut reader = BufReader::new(File::open(input)?);
    read_wav(&mut reader)
}

/// Write a WAV file with CDP metadata (for internal use)
pub fn write_wav_cdp(output: &Path, format: &WavFormat, samples: &[i16]) -> io::Result<()> {
    // Calculate peak
    let (peak_value, peak_position) = calculate_peak(samples);

    // Create CDP chunks
    let cdp_chunks = create_cdp_chunks(
        peak_value,
        peak_position,
        samples.len() as u32 / (format.channels as u32),
    );

    // Write output
    let mut writer = BufWriter::new(File::create(output)?);
    write_wav_cdp_internal(&mut writer, format, samples, &cdp_chunks)?;
    writer.flush()?;
    Ok(())
}

/// Copy a WAV file with CDP metadata
pub fn copy_wav_cdp(input: &Path, output: &Path) -> Result<()> {
    let mut reader = BufReader::new(File::open(input)?);
    let (format, samples) = read_wav(&mut reader)?;

    // Calculate peak while reading
    let (peak_value, peak_position) = calculate_peak(&samples);

    // Create CDP chunks
    let cdp_chunks = create_cdp_chunks(
        peak_value,
        peak_position,
        samples.len() as u32 / (format.channels as u32),
    );

    // Write output
    let mut writer = BufWriter::new(File::create(output)?);
    write_wav_cdp_internal(&mut writer, &format, &samples, &cdp_chunks)?;
    writer.flush()?;
    Ok(())
}

/// Read WAV file (handles both simple and CDP-format WAVs)
fn read_wav<R: Read>(reader: &mut R) -> io::Result<(WavFormat, Vec<i16>)> {
    let mut header = [0u8; 12];
    reader.read_exact(&mut header)?;

    // Verify RIFF header
    if &header[0..4] != b"RIFF" {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Not a WAV file"));
    }

    // Skip file size (bytes 4-7)

    if &header[8..12] != b"WAVE" {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Not a WAV file"));
    }

    // Now read chunks until we find fmt and data
    let mut format: Option<WavFormat> = None;
    let mut samples = Vec::new();

    loop {
        let mut chunk_header = [0u8; 8];
        if reader.read_exact(&mut chunk_header).is_err() {
            break; // End of file
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
                // Read format chunk
                let mut fmt_data = vec![0u8; chunk_size as usize];
                reader.read_exact(&mut fmt_data)?;

                format = Some(WavFormat {
                    channels: u16::from_le_bytes([fmt_data[2], fmt_data[3]]),
                    sample_rate: u32::from_le_bytes([
                        fmt_data[4],
                        fmt_data[5],
                        fmt_data[6],
                        fmt_data[7],
                    ]),
                    bits_per_sample: u16::from_le_bytes([fmt_data[14], fmt_data[15]]),
                    data_size: 0, // Will be set when we find data chunk
                });
            }
            b"data" => {
                // Read data chunk
                if let Some(ref mut fmt) = format {
                    fmt.data_size = chunk_size;

                    // Read all samples (assuming 16-bit)
                    let sample_count = chunk_size as usize / 2;
                    samples = vec![0i16; sample_count];

                    for sample in &mut samples {
                        let mut buf = [0u8; 2];
                        reader.read_exact(&mut buf)?;
                        *sample = i16::from_le_bytes(buf);
                    }
                    break; // We have everything we need
                }
            }
            _ => {
                // Skip unknown chunks
                let mut skip_buf = vec![0u8; chunk_size as usize];
                reader.read_exact(&mut skip_buf)?;
            }
        }

        // Ensure chunk size is even (WAV spec requires word alignment)
        if chunk_size % 2 != 0 {
            let mut padding = [0u8; 1];
            let _ = reader.read_exact(&mut padding);
        }
    }

    if let Some(format) = format {
        if !samples.is_empty() {
            return Ok((format, samples));
        }
    }

    Err(io::Error::new(
        io::ErrorKind::InvalidData,
        "Missing fmt or data chunk",
    ))
}

/// Calculate peak value from samples  
fn calculate_peak(samples: &[i16]) -> (f32, u32) {
    let mut max_sample = 0i16;
    let mut peak_position = 0u32;

    for (i, &sample) in samples.iter().enumerate() {
        let abs_sample = sample.abs();
        if abs_sample > max_sample {
            max_sample = abs_sample;
            peak_position = i as u32;
        }
    }

    (max_sample as f32 / 32767.0, peak_position)
}

/// Create CDP-specific chunks
fn create_cdp_chunks(peak_value: f32, peak_position: u32, _frame_count: u32) -> CdpChunks {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as u32;

    // Create CDP's fixed-size note chunk (2004 bytes)
    let mut note_data = Vec::with_capacity(2004);

    // Write "sfif" identifier
    note_data.extend_from_slice(b"sfif");

    // Write "DATE\n" followed by timestamp in uppercase hex
    note_data.extend_from_slice(b"DATE\n");
    note_data.extend_from_slice(format!("{:X}\n", timestamp).as_bytes());

    // Pad with newlines to exactly 2004 bytes
    while note_data.len() < 2004 {
        note_data.push(b'\n');
    }

    CdpChunks {
        peak: PeakChunk {
            version: 1,
            timestamp,
            peak_value,
            peak_position,
        },
        cue: CueChunk {
            cue_points: vec![CuePoint {
                id: [b's', b'f', b'i', b'f'], // CDP uses "sfif" as cue point ID
                position: 0,
                data_chunk_id: *b"data",
                chunk_start: 0,
                block_start: 0,
                sample_offset: 0,
            }],
        },
        list: ListChunk { note_data },
    }
}

/// Write WAV file with CDP chunks
fn write_wav_cdp_internal<W: Write>(
    writer: &mut W,
    format: &WavFormat,
    samples: &[i16],
    cdp_chunks: &CdpChunks,
) -> io::Result<()> {
    // Calculate sizes
    let data_size = samples.len() * 2;
    let fmt_chunk_size = 16;
    let peak_chunk_size = 16; // 4 * 4 bytes
    let cue_chunk_size = 28; // 4 + 24 for one cue point

    // LIST chunk needs padding if note_data length is odd
    let note_data_padded_len = if cdp_chunks.list.note_data.len() % 2 != 0 {
        cdp_chunks.list.note_data.len() + 1
    } else {
        cdp_chunks.list.note_data.len()
    };
    let list_chunk_size = 4 + 4 + 4 + cdp_chunks.list.note_data.len(); // "adtl" + "note" + note_size + data (not padded)

    let riff_size = 4 + // "WAVE"
        8 + fmt_chunk_size +
        8 + peak_chunk_size +
        8 + cue_chunk_size +
        8 + list_chunk_size + (note_data_padded_len - cdp_chunks.list.note_data.len()) +
        8 + data_size;

    // Write RIFF header
    writer.write_all(b"RIFF")?;
    writer.write_all(&(riff_size as u32).to_le_bytes())?;
    writer.write_all(b"WAVE")?;

    // Write fmt chunk
    writer.write_all(b"fmt ")?;
    writer.write_all(&16u32.to_le_bytes())?; // chunk size
    writer.write_all(&1u16.to_le_bytes())?; // audio format (PCM)
    writer.write_all(&format.channels.to_le_bytes())?;
    writer.write_all(&format.sample_rate.to_le_bytes())?;
    let byte_rate = format.sample_rate * format.channels as u32 * 2;
    writer.write_all(&byte_rate.to_le_bytes())?;
    let block_align = format.channels * 2;
    writer.write_all(&block_align.to_le_bytes())?;
    writer.write_all(&format.bits_per_sample.to_le_bytes())?;

    // Write PEAK chunk
    writer.write_all(b"PEAK")?;
    writer.write_all(&16u32.to_le_bytes())?;
    writer.write_all(&cdp_chunks.peak.version.to_le_bytes())?;
    writer.write_all(&cdp_chunks.peak.timestamp.to_le_bytes())?;
    writer.write_all(&cdp_chunks.peak.peak_value.to_le_bytes())?;
    writer.write_all(&cdp_chunks.peak.peak_position.to_le_bytes())?;

    // Write cue chunk
    writer.write_all(b"cue ")?;
    writer.write_all(&28u32.to_le_bytes())?;
    writer.write_all(&1u32.to_le_bytes())?; // cue point count
    writer.write_all(&cdp_chunks.cue.cue_points[0].id)?;
    writer.write_all(&cdp_chunks.cue.cue_points[0].position.to_le_bytes())?;
    writer.write_all(&cdp_chunks.cue.cue_points[0].data_chunk_id)?;
    writer.write_all(&cdp_chunks.cue.cue_points[0].chunk_start.to_le_bytes())?;
    writer.write_all(&cdp_chunks.cue.cue_points[0].block_start.to_le_bytes())?;
    writer.write_all(&cdp_chunks.cue.cue_points[0].sample_offset.to_le_bytes())?;

    // Write LIST chunk
    writer.write_all(b"LIST")?;
    writer.write_all(&(list_chunk_size as u32).to_le_bytes())?;
    writer.write_all(b"adtl")?;
    writer.write_all(b"note")?;
    writer.write_all(&(cdp_chunks.list.note_data.len() as u32).to_le_bytes())?;
    writer.write_all(&cdp_chunks.list.note_data)?;

    // Add padding if note_data length is odd
    if cdp_chunks.list.note_data.len() % 2 != 0 {
        writer.write_all(&[0u8])?;
    }

    // Write data chunk
    writer.write_all(b"data")?;
    writer.write_all(&(data_size as u32).to_le_bytes())?;
    for &sample in samples {
        writer.write_all(&sample.to_le_bytes())?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peak_calculation() {
        let samples = vec![0, 1000, -2000, 3000, -32767];
        let (peak, pos) = calculate_peak(&samples);
        assert_eq!(peak, 32767.0 / 32767.0);
        assert_eq!(pos, 4);
    }

    #[test]
    fn test_wav_format() {
        let format = WavFormat {
            channels: 2,
            sample_rate: 44100,
            bits_per_sample: 16,
            data_size: 176400,
        };
        assert_eq!(format.channels, 2);
        assert_eq!(format.sample_rate, 44100);
    }
}
