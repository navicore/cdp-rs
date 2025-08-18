//! Generate sample audio files for the examples
//!
//! Run this first to create the sample WAV files needed by other examples:
//! cargo run -p cdp-sandbox --example generate_samples

use cdp_sandbox::housekeep::wav_cdp::{self, WavFormat};
use std::f32::consts::PI;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Generating sample audio files for examples...\n");

    // Generate files in current directory
    println!("Generating files in current directory...");

    // Generate stereo test tone (440Hz left, 880Hz right)
    generate_stereo_tone()?;

    // Generate mono sine wave
    generate_mono_sine()?;

    // Generate white noise
    generate_white_noise()?;

    // Generate chirp signal
    generate_chirp()?;

    // Generate quiet signal (for normalization demo)
    generate_quiet_signal()?;

    println!("\n✓ All sample files generated in current directory");
    println!("\nYou can now run the other examples:");
    println!("  cargo run -p cdp-sandbox --example audio_processing");
    println!("  cargo run -p cdp-sandbox --example batch_normalize");
    println!("  cargo run -p cdp-sandbox --example channel_extract");

    Ok(())
}

fn generate_stereo_tone() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("stereo_tone.wav");
    let sample_rate = 44100;
    let duration = 2.0;
    let num_samples = (sample_rate as f32 * duration) as usize;

    let mut samples = Vec::new();
    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let left = (2.0 * PI * 440.0 * t).sin() * 0.7; // A4
        let right = (2.0 * PI * 880.0 * t).sin() * 0.7; // A5

        samples.push((left * 32767.0) as i16);
        samples.push((right * 32767.0) as i16);
    }

    let format = WavFormat {
        channels: 2,
        sample_rate,
        bits_per_sample: 16,
        data_size: (samples.len() * 2) as u32,
    };

    wav_cdp::write_wav_cdp(path, &format, &samples)?;
    println!("  Created: stereo_tone.wav (440Hz left, 880Hz right, 2 seconds)");
    Ok(())
}

fn generate_mono_sine() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("mono_sine.wav");
    let sample_rate = 44100;
    let duration = 1.0;
    let frequency = 440.0;
    let num_samples = (sample_rate as f32 * duration) as usize;

    let mut samples = Vec::new();
    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let sample = (2.0 * PI * frequency * t).sin() * 0.8;
        samples.push((sample * 32767.0) as i16);
    }

    let format = WavFormat {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        data_size: (samples.len() * 2) as u32,
    };

    wav_cdp::write_wav_cdp(path, &format, &samples)?;
    println!("  Created: mono_sine.wav (440Hz, 1 second)");
    Ok(())
}

fn generate_white_noise() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("white_noise.wav");
    let sample_rate = 44100;
    let duration = 1.0;
    let num_samples = (sample_rate as f32 * duration) as usize;

    // Simple pseudo-random noise generator
    let mut samples = Vec::new();
    let mut seed = 12345u32;
    for _ in 0..num_samples {
        // Linear congruential generator
        seed = (seed.wrapping_mul(1103515245).wrapping_add(12345)) & 0x7fffffff;
        let normalized = (seed as f32 / 0x7fffffff as f32) * 2.0 - 1.0;
        samples.push((normalized * 0.3 * 32767.0) as i16); // Keep it quieter
    }

    let format = WavFormat {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        data_size: (samples.len() * 2) as u32,
    };

    wav_cdp::write_wav_cdp(path, &format, &samples)?;
    println!("  Created: white_noise.wav (1 second)");
    Ok(())
}

fn generate_chirp() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("chirp.wav");
    let sample_rate = 44100;
    let duration = 2.0;
    let start_freq = 100.0;
    let end_freq = 2000.0;
    let num_samples = (sample_rate as f32 * duration) as usize;

    let mut samples = Vec::new();
    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let progress = t / duration;
        let freq = start_freq + (end_freq - start_freq) * progress;
        let sample = (2.0 * PI * freq * t).sin() * 0.7;
        samples.push((sample * 32767.0) as i16);
    }

    let format = WavFormat {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        data_size: (samples.len() * 2) as u32,
    };

    wav_cdp::write_wav_cdp(path, &format, &samples)?;
    println!("  Created: chirp.wav (100Hz to 2000Hz sweep, 2 seconds)");
    Ok(())
}

fn generate_quiet_signal() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("quiet_sine.wav");
    let sample_rate = 44100;
    let duration = 1.0;
    let frequency = 440.0;
    let num_samples = (sample_rate as f32 * duration) as usize;

    let mut samples = Vec::new();
    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;
        let sample = (2.0 * PI * frequency * t).sin() * 0.1; // Very quiet
        samples.push((sample * 32767.0) as i16);
    }

    let format = WavFormat {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        data_size: (samples.len() * 2) as u32,
    };

    wav_cdp::write_wav_cdp(path, &format, &samples)?;
    println!("  Created: quiet_sine.wav (440Hz at -20dB, 1 second)");
    Ok(())
}
