//! CDP-compatible WAV file I/O
//!
//! This module implements WAV file reading and writing that exactly matches
//! CDP's format, including PEAK chunks, cue points, and metadata.

use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Seek, Write};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

/// CDP-specific WAV chunks
#[derive(Debug, Clone)]
pub struct CdpChunks {
    pub peak: PeakChunk,
    pub cue: CueChunk,
    pub list: ListChunk,
}

#[derive(Debug, Clone)]
pub struct PeakChunk {
    pub version: u32,
    pub timestamp: u32,
    pub peak_value: f32,
    pub peak_position: u32,
}

#[derive(Debug, Clone)]
pub struct CueChunk {
    pub cue_points: Vec<CuePoint>,
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
pub struct ListChunk {
    pub note_data: Vec<u8>,
}

/// Simple WAV format info
#[derive(Debug, Clone)]
pub struct WavFormat {
    pub channels: u16,
    pub sample_rate: u32,
    pub bits_per_sample: u16,
    pub data_size: u32,
}

/// Copy a WAV file with CDP metadata
pub fn copy_wav_cdp_format(input: &Path, output: &Path) -> io::Result<()> {
    // Read input file
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

    // Write output with CDP format
    let mut writer = BufWriter::new(File::create(output)?);
    write_wav_cdp(&mut writer, &format, &samples, &cdp_chunks)?;

    Ok(())
}

/// Read a basic WAV file (simplified for now)
fn read_wav<R: Read>(reader: &mut R) -> io::Result<(WavFormat, Vec<i16>)> {
    let mut header = [0u8; 44];
    reader.read_exact(&mut header)?;

    // Verify RIFF header
    if &header[0..4] != b"RIFF" || &header[8..12] != b"WAVE" {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Not a WAV file"));
    }

    // Parse fmt chunk (assuming it's at standard position)
    if &header[12..16] != b"fmt " {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "fmt chunk not found",
        ));
    }

    let format = WavFormat {
        channels: u16::from_le_bytes([header[22], header[23]]),
        sample_rate: u32::from_le_bytes([header[24], header[25], header[26], header[27]]),
        bits_per_sample: u16::from_le_bytes([header[34], header[35]]),
        data_size: u32::from_le_bytes([header[40], header[41], header[42], header[43]]),
    };

    // Read all samples (assuming 16-bit)
    let sample_count = format.data_size as usize / 2;
    let mut samples = vec![0i16; sample_count];

    for sample in &mut samples {
        let mut buf = [0u8; 2];
        reader.read_exact(&mut buf)?;
        *sample = i16::from_le_bytes(buf);
    }

    Ok((format, samples))
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

    CdpChunks {
        peak: PeakChunk {
            version: 1,
            timestamp,
            peak_value,
            peak_position,
        },
        cue: CueChunk {
            cue_points: vec![CuePoint {
                id: *b"sfif",
                position: 0,
                data_chunk_id: *b"data",
                chunk_start: 0,
                block_start: 0,
                sample_offset: 0,
            }],
        },
        list: ListChunk {
            note_data: create_note_data(),
        },
    }
}

/// Create the LIST/note chunk data
fn create_note_data() -> Vec<u8> {
    let mut data = Vec::with_capacity(2004);

    // Start with CDP marker
    data.extend_from_slice(b"sfifDATE\n");

    // Add timestamp or ID
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    data.extend_from_slice(format!("{:X}\n", timestamp).as_bytes());

    // Pad to 2004 bytes with newlines
    while data.len() < 2004 {
        data.push(b'\n');
    }

    data
}

/// Write WAV file with CDP chunks
fn write_wav_cdp<W: Write + Seek>(
    writer: &mut W,
    format: &WavFormat,
    samples: &[i16],
    cdp_chunks: &CdpChunks,
) -> io::Result<()> {
    // Calculate sizes
    let data_size = samples.len() * 2;
    let peak_chunk_size = 16;
    let cue_chunk_size = 28;
    let list_chunk_size = 4 + 8 + cdp_chunks.list.note_data.len(); // "adtl" + "note" + size + data
    let total_size = 4 + // "WAVE"
                     8 + 16 + // fmt chunk
                     8 + peak_chunk_size + // PEAK chunk
                     8 + cue_chunk_size + // cue chunk
                     8 + list_chunk_size + // LIST chunk
                     8 + data_size; // data chunk

    // Write RIFF header
    writer.write_all(b"RIFF")?;
    writer.write_all(&(total_size as u32).to_le_bytes())?;
    writer.write_all(b"WAVE")?;

    // Write fmt chunk
    writer.write_all(b"fmt ")?;
    writer.write_all(&16u32.to_le_bytes())?; // fmt chunk size
    writer.write_all(&1u16.to_le_bytes())?; // PCM format
    writer.write_all(&format.channels.to_le_bytes())?;
    writer.write_all(&format.sample_rate.to_le_bytes())?;
    writer.write_all(&(format.sample_rate * format.channels as u32 * 2).to_le_bytes())?; // byte rate
    writer.write_all(&(format.channels * 2).to_le_bytes())?; // block align
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

    // Write data chunk
    writer.write_all(b"data")?;
    writer.write_all(&(data_size as u32).to_le_bytes())?;
    for &sample in samples {
        writer.write_all(&sample.to_le_bytes())?;
    }

    Ok(())
}
