//! Vocal processing with distortion effects

use cdp_distort::{multiply, overload, ClipType};
use hound::{SampleFormat, WavSpec, WavWriter};
use std::f32::consts::PI;
use std::fs;
use std::path::Path;

fn generate_vocal_sample(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let sample_rate = 44100;
    let duration = 2.0;
    let num_samples = (sample_rate as f32 * duration) as usize;

    let mut samples = Vec::with_capacity(num_samples);

    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;

        // Simulate vocal formants
        let f1 = (2.0 * PI * 700.0 * t).sin() * 0.3; // First formant
        let f2 = (2.0 * PI * 1220.0 * t).sin() * 0.2; // Second formant
        let f3 = (2.0 * PI * 2600.0 * t).sin() * 0.1; // Third formant

        // Add some vibrato
        let vibrato_freq = 5.0;
        let vibrato_depth = 0.02;
        let vibrato = (2.0 * PI * vibrato_freq * t).sin() * vibrato_depth;

        // Fundamental with vibrato
        let fundamental = (2.0 * PI * 220.0 * (1.0 + vibrato) * t).sin() * 0.4;

        // Dynamic envelope (simulate words/phrases)
        let phrase_envelope = ((t * 2.0).sin() * 0.3 + 0.7).max(0.0);
        let attack = (t * 10.0).min(1.0);
        let envelope = attack * phrase_envelope;

        let sample = (fundamental + f1 + f2 + f3) * envelope;
        samples.push(sample);
    }

    let spec = WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 32,
        sample_format: SampleFormat::Float,
    };

    let mut writer = WavWriter::create(path, spec)?;
    for sample in samples {
        writer.write_sample(sample)?;
    }
    writer.finalize()?;

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Vocal Processing Examples");
    println!("=========================\n");

    // Create examples directory
    fs::create_dir_all("crates/cdp-distort/examples")?;

    // Generate vocal-like sound
    let input_path = Path::new("crates/cdp-distort/examples/vocal_sample.wav");
    println!("Generating vocal sample...");
    generate_vocal_sample(input_path)?;

    // 1. Telephone effect
    println!("\n1. Telephone/Radio Effect:");
    let output_path = Path::new("crates/cdp-distort/examples/telephone_vocal.wav");
    overload(input_path, output_path, 0.4, 2.0, ClipType::Hard)?;
    println!("   Created: telephone_vocal.wav");
    println!("   Effect: Lo-fi telephone/radio voice");
    println!("   Use: Dialog processing, vintage effect");

    // 2. Warm saturation
    println!("\n2. Warm Vocal Saturation:");
    let output_path = Path::new("crates/cdp-distort/examples/warm_vocal.wav");
    overload(input_path, output_path, 0.85, 1.3, ClipType::Tube)?;
    println!("   Created: warm_vocal.wav");
    println!("   Effect: Subtle warmth and presence");
    println!("   Use: Enhance vocal presence in mix");

    // 3. Robot/vocoder effect
    println!("\n3. Robot/Vocoder Style:");
    let output_path = Path::new("crates/cdp-distort/examples/robot_vocal.wav");
    multiply(input_path, output_path, 4.0, 0.7)?;
    println!("   Created: robot_vocal.wav");
    println!("   Effect: Metallic, robotic voice");
    println!("   Use: Electronic music, special effects");

    // 4. Aggressive vocal
    println!("\n4. Aggressive/Screaming Vocal:");
    let output_path = Path::new("crates/cdp-distort/examples/aggressive_vocal.wav");
    overload(input_path, output_path, 0.3, 5.0, ClipType::Asymmetric)?;
    println!("   Created: aggressive_vocal.wav");
    println!("   Effect: Intense, distorted vocal");
    println!("   Use: Heavy metal, industrial music");

    // 5. Megaphone effect
    println!("\n5. Megaphone/Bullhorn Effect:");
    let temp_path = Path::new("crates/cdp-distort/examples/temp.wav");
    let output_path = Path::new("crates/cdp-distort/examples/megaphone_vocal.wav");

    // First add harmonics
    multiply(input_path, temp_path, 2.0, 0.5)?;
    // Then hard clip
    overload(temp_path, output_path, 0.5, 3.0, ClipType::Hard)?;
    fs::remove_file(temp_path)?;

    println!("   Created: megaphone_vocal.wav");
    println!("   Effect: Megaphone/bullhorn sound");
    println!("   Use: Protest scenes, sports announcer");

    // 6. Whisper enhancement
    println!("\n6. Whisper Enhancement:");
    let output_path = Path::new("crates/cdp-distort/examples/whisper_enhance.wav");
    multiply(input_path, output_path, 3.0, 0.3)?;
    println!("   Created: whisper_enhance.wav");
    println!("   Effect: Enhanced breathy quality");
    println!("   Use: Intimate vocals, ASMR content");

    // 7. Vintage microphone
    println!("\n7. Vintage Microphone:");
    let output_path = Path::new("crates/cdp-distort/examples/vintage_mic.wav");
    overload(input_path, output_path, 0.7, 1.8, ClipType::Soft)?;
    println!("   Created: vintage_mic.wav");
    println!("   Effect: Old microphone character");
    println!("   Use: Retro productions, jazz vocals");

    println!("\nVocal Processing Guide:");
    println!("┌─────────────────┬────────────┬──────────┬─────────────┐");
    println!("│ Effect          │ Method     │ Amount   │ Character   │");
    println!("├─────────────────┼────────────┼──────────┼─────────────┤");
    println!("│ Warmth          │ Tube       │ Subtle   │ Smooth      │");
    println!("│ Telephone       │ Hard Clip  │ Medium   │ Lo-fi       │");
    println!("│ Robot           │ Multiply   │ High     │ Metallic    │");
    println!("│ Megaphone       │ Chain      │ High     │ Harsh       │");
    println!("│ Vintage         │ Soft Clip  │ Low      │ Compressed  │");
    println!("└─────────────────┴────────────┴──────────┴─────────────┘");

    println!("\nProduction Tips:");
    println!("- Use parallel processing (mix parameter) to maintain clarity");
    println!("- Apply EQ after distortion to shape tone");
    println!("- Automate distortion amount for dynamic effects");
    println!("- Layer clean and distorted vocals for thickness");
    println!("- Use subtle amounts for mix presence");

    Ok(())
}
