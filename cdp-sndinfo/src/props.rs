//! Properties display for sound files
//!
//! Shows format information, duration, peak levels, etc.

use super::{Result, SndinfoError};
use cdp_housekeep::wav_cdp;
use std::fs::File;
use std::io::{BufReader, Read, Seek, SeekFrom};
use std::path::Path;

/// Display properties of a sound file
pub fn show_props(input: &Path) -> Result<()> {
    // Read the WAV file header and metadata
    let mut reader = BufReader::new(File::open(input)?);

    // Read basic format info
    let (format, peak_info) = read_wav_with_metadata(&mut reader)?;

    // Calculate duration
    let total_samples = format.data_size as usize / 2 / format.channels as usize;
    let duration_secs = total_samples as f64 / format.sample_rate as f64;

    // Display CDP-style output
    println!("CDP Release 7.1 2016"); // Match CDP's output format
    println!("A SOUND file.");
    println!("samples: ............ {}", total_samples);
    println!("file type: ........... SOUND");
    println!("sample rate: ........ {}", format.sample_rate);
    println!("channels: ........... {}", format.channels);
    println!("sample type:  {}bit", format.bits_per_sample);

    // Show peak info if available
    if let Some((peak_value, peak_pos)) = peak_info {
        // Calculate dB value
        let db = if peak_value > 0.0 {
            20.0 * peak_value.log10()
        } else {
            -96.0 // Silence
        };

        // For mono files, show channel-specific peak info
        if format.channels == 1 {
            println!("PEAK data (simplified)");
            println!();
            println!(
                "CH 1:\tamp = {:.4} ({:.2} dB)\tFrame {}",
                peak_value, db, peak_pos
            );
        } else {
            println!("maximum level: ...... {:.6}", peak_value);
        }
    } else {
        println!("No PEAK chunk in this file");
    }

    // Show duration
    let mins = (duration_secs / 60.0) as i32;
    let secs = duration_secs - (mins as f64 * 60.0);
    if mins > 0 {
        println!("duration: ........... {} min {:.2} sec", mins, secs);
    } else {
        println!("duration: ........... {:.2} sec", secs);
    }

    Ok(())
}

/// Read WAV file with metadata (including PEAK chunk if present)
fn read_wav_with_metadata<R: Read + Seek>(
    reader: &mut R,
) -> Result<(wav_cdp::WavFormat, Option<(f32, u32)>)> {
    let mut header = [0u8; 12];
    reader.read_exact(&mut header)?;

    // Verify RIFF header
    if &header[0..4] != b"RIFF" || &header[8..12] != b"WAVE" {
        return Err(SndinfoError::InvalidFile("Not a WAV file".into()));
    }

    let mut format_info = None;
    let mut peak_info = None;
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

                if fmt_data.len() >= 16 {
                    let channels = u16::from_le_bytes([fmt_data[2], fmt_data[3]]);
                    let sample_rate =
                        u32::from_le_bytes([fmt_data[4], fmt_data[5], fmt_data[6], fmt_data[7]]);
                    let bits_per_sample = u16::from_le_bytes([fmt_data[14], fmt_data[15]]);

                    format_info = Some((channels, sample_rate, bits_per_sample));
                }
            }
            b"PEAK" => {
                if chunk_size >= 16 {
                    let mut peak_data = [0u8; 16];
                    reader.read_exact(&mut peak_data)?;

                    // Skip version and timestamp
                    let peak_bytes = [peak_data[8], peak_data[9], peak_data[10], peak_data[11]];
                    let peak_value = f32::from_le_bytes(peak_bytes);

                    let peak_pos = u32::from_le_bytes([
                        peak_data[12],
                        peak_data[13],
                        peak_data[14],
                        peak_data[15],
                    ]);

                    peak_info = Some((peak_value, peak_pos));

                    // Skip any remaining data
                    if chunk_size > 16 {
                        reader.seek(SeekFrom::Current((chunk_size - 16) as i64))?;
                    }
                }
            }
            b"data" => {
                data_size = chunk_size;
                // Skip the actual audio data
                reader.seek(SeekFrom::Current(chunk_size as i64))?;
            }
            _ => {
                // Skip unknown chunks
                reader.seek(SeekFrom::Current(chunk_size as i64))?;
            }
        }

        // Align to word boundary if necessary
        if chunk_size % 2 != 0 {
            reader.seek(SeekFrom::Current(1))?;
        }
    }

    if let Some((channels, sample_rate, bits_per_sample)) = format_info {
        let format = wav_cdp::WavFormat {
            channels,
            sample_rate,
            bits_per_sample,
            data_size,
        };
        Ok((format, peak_info))
    } else {
        Err(SndinfoError::InvalidFile("No format chunk found".into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_props_validation() {
        // Test with non-existent file
        let result = show_props(Path::new("nonexistent.wav"));
        assert!(result.is_err());
    }
}
