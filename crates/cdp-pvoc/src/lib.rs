//! CDP Phase Vocoder implementation
//!
//! This crate provides phase vocoder functionality matching CDP's implementation.
//! The analysis files (.ana) are stored as WAV files with IEEE float format.

use num_complex::Complex32;
use rustfft::{num_complex::ComplexFloat, FftPlanner};
use std::f32::consts::PI;
use std::fs::File;
use std::io::{self, BufReader, BufWriter, Read, Seek, SeekFrom, Write};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PvocError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("Invalid file format")]
    InvalidFormat,

    #[error("Invalid parameters: {0}")]
    InvalidParams(String),

    #[error("Housekeep error: {0}")]
    Housekeep(#[from] cdp_housekeep::HousekeepError),
}

pub type Result<T> = std::result::Result<T, PvocError>;

/// CDP .ana file header information
#[derive(Debug, Clone)]
pub struct AnaHeader {
    /// Sample rate of original file
    pub sample_rate: u32,
    /// Number of frequency channels (half FFT size)
    pub channels: u32,
    /// Analysis window length
    pub window_len: u32,
    /// Decimation factor (hop size divisor)
    pub dec_factor: u32,
    /// Original file size in samples
    pub orig_size: u32,
}

/// Perform phase vocoder analysis
pub fn pvoc_anal(
    input_path: &Path,
    output_path: &Path,
    mode: u32,
    channels: Option<u32>,
    overlap: Option<u32>,
) -> Result<()> {
    // Default parameters
    let fft_size = channels.unwrap_or(1024);
    let overlap_factor = overlap.unwrap_or(3);

    // Validate FFT size is power of 2
    if !(2..=32768).contains(&fft_size) || (fft_size & (fft_size - 1)) != 0 {
        return Err(PvocError::InvalidParams(
            "FFT size must be power of 2 between 2 and 32768".into(),
        ));
    }

    // Read input WAV file
    let (format, samples) = cdp_housekeep::read_wav_basic(input_path)?;

    // Convert samples to float
    let float_samples: Vec<f32> = samples.iter().map(|&s| s as f32 / 32768.0).collect();

    // Calculate hop size
    let hop_size = fft_size / overlap_factor;

    // Create window function (Hanning)
    let window = create_hanning_window(fft_size as usize);

    // Prepare FFT
    let mut planner = FftPlanner::<f32>::new();
    let fft = planner.plan_fft_forward(fft_size as usize);

    // Process frames
    let mut spectral_frames = Vec::new();
    let mut position = 0;

    while position + fft_size as usize <= float_samples.len() {
        // Extract and window frame
        let mut frame: Vec<Complex32> = float_samples[position..position + fft_size as usize]
            .iter()
            .zip(window.iter())
            .map(|(&sample, &w)| Complex32::new(sample * w, 0.0))
            .collect();

        // Perform FFT
        fft.process(&mut frame);

        // Store spectral frame based on mode
        let spectral_data = match mode {
            1 => convert_to_polar(&frame), // Standard analysis (magnitude + phase)
            2 => extract_envelope(&frame), // Envelope only
            3 => extract_magnitude(&frame), // Magnitude only
            _ => return Err(PvocError::InvalidParams("Invalid mode".into())),
        };

        spectral_frames.push(spectral_data);
        position += hop_size as usize;
    }

    // Write output as IEEE float WAV with CDP metadata
    write_ana_file(
        output_path,
        &spectral_frames,
        format.sample_rate,
        fft_size,
        overlap_factor,
        float_samples.len() as u32,
    )?;

    Ok(())
}

/// Create Hanning window
fn create_hanning_window(size: usize) -> Vec<f32> {
    (0..size)
        .map(|i| 0.5 * (1.0 - (2.0 * PI * i as f32 / (size - 1) as f32).cos()))
        .collect()
}

/// Convert complex FFT output to polar form (magnitude, phase)
fn convert_to_polar(frame: &[Complex32]) -> Vec<f32> {
    let mut result = Vec::with_capacity((frame.len() / 2 + 1) * 2);

    // CDP format: for each bin from 0 to N/2, store real and imaginary parts
    // This maintains phase information in rectangular form
    for complex in frame.iter().take(frame.len() / 2 + 1) {
        result.push(complex.re);
        result.push(complex.im);
    }

    result
}

/// Extract spectral envelope
fn extract_envelope(frame: &[Complex32]) -> Vec<f32> {
    // For mode 2, we extract envelope values
    // Store magnitude in real part, zero in imaginary
    let mut result = Vec::with_capacity((frame.len() / 2 + 1) * 2);

    for complex in frame.iter().take(frame.len() / 2 + 1) {
        let mag = complex.abs();
        result.push(mag);
        result.push(0.0); // No phase for envelope mode
    }

    result
}

/// Extract magnitude only
fn extract_magnitude(frame: &[Complex32]) -> Vec<f32> {
    // Store magnitude values, zero phase
    let mut result = Vec::with_capacity((frame.len() / 2 + 1) * 2);

    for complex in frame.iter().take(frame.len() / 2 + 1) {
        result.push(complex.abs());
        result.push(0.0); // No phase for magnitude mode
    }

    result
}

/// Write .ana file (IEEE float WAV with CDP metadata)
fn write_ana_file(
    path: &Path,
    frames: &[Vec<f32>],
    sample_rate: u32,
    fft_size: u32,
    overlap_factor: u32,
    _orig_samples: u32,
) -> Result<()> {
    let mut writer = BufWriter::new(File::create(path)?);

    // Calculate sizes
    // CDP stores spectral data as (FFT_size/2 + 1) complex pairs = (FFT_size/2 + 1) * 2 channels
    let channels = ((fft_size / 2 + 1) * 2) as u16; // CDP convention
    let frame_count = frames.len() as u32;
    let data_size = frame_count * channels as u32 * 4; // 4 bytes per float

    // Create LIST chunk metadata
    let _timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as u32;

    let metadata = format!(
        "original sampsize: 16\n\
         original sample rate: {}\n\
         arate: {}\n\
         analwinlen: {}\n\
         decfactor: {}\n\
         origrate: {}\n\
         DATE: CDP Phase Vocoder Analysis\n",
        sample_rate,
        sample_rate as f32 / (fft_size / overlap_factor) as f32,
        fft_size,
        overlap_factor,
        sample_rate
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
    writer.write_all(&channels.to_le_bytes())?;
    writer.write_all(&sample_rate.to_le_bytes())?;
    let byte_rate = sample_rate * channels as u32 * 4; // 4 bytes per float
    writer.write_all(&byte_rate.to_le_bytes())?;
    let block_align = channels * 4;
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

    // Write spectral frames
    for frame in frames {
        for &value in frame {
            writer.write_all(&value.to_le_bytes())?;
        }
    }

    writer.flush()?;
    Ok(())
}

/// Perform phase vocoder synthesis
pub fn pvoc_synth(input_path: &Path, output_path: &Path) -> Result<()> {
    // Read .ana file
    let (header, spectral_frames) = read_ana_file(input_path)?;

    // Calculate parameters from header
    // CDP uses channels = (fft_size/2 + 1) * 2
    let fft_size = (header.channels / 2 - 1) * 2;
    let hop_size = fft_size / header.dec_factor;

    // Create window function (Hanning)
    let window = create_hanning_window(fft_size as usize);

    // Prepare IFFT
    let mut planner = FftPlanner::<f32>::new();
    let ifft = planner.plan_fft_inverse(fft_size as usize);

    // Synthesize audio
    let output_length = ((spectral_frames.len() - 1) * hop_size as usize) + fft_size as usize;
    let mut output = vec![0.0f32; output_length];
    let mut position = 0;

    for frame_data in &spectral_frames {
        // Convert polar to complex
        let mut frame = polar_to_complex(frame_data, fft_size as usize);

        // Perform IFFT
        ifft.process(&mut frame);

        // Apply window and overlap-add
        for (i, sample) in frame.iter().enumerate() {
            if position + i < output.len() {
                output[position + i] += sample.re * window[i] / fft_size as f32;
            }
        }

        position += hop_size as usize;
    }

    // Normalize to prevent clipping
    let max_val = output.iter().map(|&x| x.abs()).fold(0.0f32, f32::max);
    if max_val > 1.0 {
        for sample in &mut output {
            *sample /= max_val * 1.1; // Scale with headroom
        }
    }

    // Convert to i16 samples
    let i16_samples: Vec<i16> = output
        .iter()
        .map(|&s| (s * 32767.0).clamp(-32768.0, 32767.0) as i16)
        .collect();

    // Write output WAV
    let format = cdp_housekeep::wav_cdp::WavFormat {
        channels: 1,
        sample_rate: header.sample_rate,
        bits_per_sample: 16,
        data_size: (i16_samples.len() * 2) as u32,
    };

    cdp_housekeep::write_wav_cdp(output_path, &format, &i16_samples)?;

    Ok(())
}

/// Convert polar representation back to complex
fn polar_to_complex(polar_data: &[f32], fft_size: usize) -> Vec<Complex32> {
    let mut result = vec![Complex32::new(0.0, 0.0); fft_size];

    // CDP format: real and imaginary parts for bins 0 to N/2
    let mut idx = 0;
    for val in result.iter_mut().take(fft_size / 2 + 1) {
        if idx + 1 < polar_data.len() {
            *val = Complex32::new(polar_data[idx], polar_data[idx + 1]);
            idx += 2;
        }
    }

    // Mirror for negative frequencies (except DC and Nyquist)
    for i in 1..fft_size / 2 {
        result[fft_size - i] = result[i].conj();
    }

    result
}

/// Read .ana file (IEEE float WAV with CDP metadata)
fn read_ana_file(path: &Path) -> Result<(AnaHeader, Vec<Vec<f32>>)> {
    let mut reader = BufReader::new(File::open(path)?);

    // Read RIFF header
    let mut header = [0u8; 12];
    reader.read_exact(&mut header)?;

    if &header[0..4] != b"RIFF" || &header[8..12] != b"WAVE" {
        return Err(PvocError::InvalidFormat);
    }

    let mut ana_header = AnaHeader {
        sample_rate: 0,
        channels: 0,
        window_len: 0,
        dec_factor: 3, // default
        orig_size: 0,
    };

    let mut spectral_data = Vec::new();

    // Read chunks
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

                let format_type = u16::from_le_bytes([fmt_data[0], fmt_data[1]]);
                if format_type != 3 {
                    return Err(PvocError::InvalidFormat);
                }

                ana_header.channels = u16::from_le_bytes([fmt_data[2], fmt_data[3]]) as u32;
                ana_header.sample_rate =
                    u32::from_le_bytes([fmt_data[4], fmt_data[5], fmt_data[6], fmt_data[7]]);
                ana_header.window_len = (ana_header.channels / 2 - 1) * 2;
            }
            b"LIST" => {
                // Parse metadata for overlap factor
                let mut list_data = vec![0u8; chunk_size as usize];
                reader.read_exact(&mut list_data)?;

                if let Ok(metadata) = std::str::from_utf8(&list_data[8..]) {
                    for line in metadata.lines() {
                        if line.starts_with("decfactor:") {
                            if let Some(val) = line.split(':').nth(1) {
                                ana_header.dec_factor = val.trim().parse().unwrap_or(3);
                            }
                        }
                    }
                }
            }
            b"data" => {
                let frame_size = ana_header.channels as usize;
                let num_frames = (chunk_size as usize) / (frame_size * 4);

                for _ in 0..num_frames {
                    let mut frame = Vec::with_capacity(frame_size);
                    for _ in 0..frame_size {
                        let mut float_bytes = [0u8; 4];
                        reader.read_exact(&mut float_bytes)?;
                        frame.push(f32::from_le_bytes(float_bytes));
                    }
                    spectral_data.push(frame);
                }
            }
            _ => {
                // Skip unknown chunks
                reader.seek(SeekFrom::Current(chunk_size as i64))?;
            }
        }
    }

    Ok((ana_header, spectral_data))
}

/// Extract a frequency band from analysis file
pub fn pvoc_extract(
    input_path: &Path,
    output_path: &Path,
    lo_freq: f32,
    hi_freq: f32,
) -> Result<()> {
    // Read input .ana file
    let (header, spectral_frames) = read_ana_file(input_path)?;

    // Calculate bin frequencies
    // CDP uses channels = (fft_size/2 + 1) * 2
    let fft_size = (header.channels / 2 - 1) * 2;
    let bin_width = header.sample_rate as f32 / fft_size as f32;

    // Calculate bin range for extraction
    let lo_bin = (lo_freq / bin_width).floor() as usize;
    let hi_bin = ((hi_freq / bin_width).ceil() as usize).min(fft_size as usize / 2);

    // Extract frequency band from each frame
    let mut filtered_frames = Vec::new();

    for frame in &spectral_frames {
        let mut filtered_frame = vec![0.0f32; frame.len()];

        // Process each bin (real/imag pairs)
        let mut idx = 0;
        for bin in 0..=fft_size as usize / 2 {
            if bin == 0 || bin == fft_size as usize / 2 {
                // Always keep DC and Nyquist
                filtered_frame[idx] = frame[idx];
                filtered_frame[idx + 1] = frame[idx + 1];
            } else if bin >= lo_bin && bin <= hi_bin {
                // Keep bins in frequency range
                filtered_frame[idx] = frame[idx]; // real
                filtered_frame[idx + 1] = frame[idx + 1]; // imag
            } else {
                // Zero out bins outside range
                filtered_frame[idx] = 0.0;
                filtered_frame[idx + 1] = 0.0;
            }
            idx += 2;
        }

        filtered_frames.push(filtered_frame);
    }

    // Write output .ana file
    write_ana_file(
        output_path,
        &filtered_frames,
        header.sample_rate,
        fft_size,
        header.dec_factor,
        header.orig_size,
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_placeholder() {
        // Placeholder test until we implement functionality
        assert_eq!(1 + 1, 2);
    }
}
