//! Showcase various distortion effects

use cdp_distort::{divide, multiply, overload, ClipType};
use hound::{SampleFormat, WavSpec, WavWriter};
use std::f32::consts::PI;
use std::fs;
use std::path::Path;

fn generate_test_signal(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    // Generate a complex test signal with multiple frequencies
    let sample_rate = 44100;
    let duration = 2.0;
    let num_samples = (sample_rate as f32 * duration) as usize;

    let mut samples = Vec::with_capacity(num_samples);

    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;

        // Mix of frequencies
        let fundamental = (2.0 * PI * 220.0 * t).sin() * 0.3;
        let harmonic = (2.0 * PI * 440.0 * t).sin() * 0.2;
        let sub = (2.0 * PI * 110.0 * t).sin() * 0.2;

        // Envelope
        let envelope = (0.5 + 0.5 * (2.0 * PI * 0.5 * t).sin()) * 0.8;

        let sample = (fundamental + harmonic + sub) * envelope;
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
    println!("Distortion Effects Showcase");
    println!("===========================\n");

    // Create examples directory
    fs::create_dir_all("crates/cdp-distort/examples")?;

    // Generate test signal
    let input_path = Path::new("crates/cdp-distort/examples/test_signal.wav");
    println!("Generating test signal...");
    generate_test_signal(input_path)?;

    // 1. Harmonic Multiplication
    println!("\n1. Harmonic Multiplication (2x):");
    let output_path = Path::new("crates/cdp-distort/examples/multiply_2x.wav");
    multiply(input_path, output_path, 2.0, 1.0)?;
    println!("   Created: multiply_2x.wav");
    println!("   Effect: Adds octave harmonics");

    println!("\n2. Harmonic Multiplication (4x with mix):");
    let output_path = Path::new("crates/cdp-distort/examples/multiply_4x_mixed.wav");
    multiply(input_path, output_path, 4.0, 0.5)?;
    println!("   Created: multiply_4x_mixed.wav");
    println!("   Effect: Stronger harmonics, 50% mix with dry");

    // 2. Subharmonic Division
    println!("\n3. Subharmonic Division (÷2):");
    let output_path = Path::new("crates/cdp-distort/examples/divide_2.wav");
    divide(input_path, output_path, 2, 1.0)?;
    println!("   Created: divide_2.wav");
    println!("   Effect: Octave down, bass enhancement");

    println!("\n4. Subharmonic Division (÷4 with mix):");
    let output_path = Path::new("crates/cdp-distort/examples/divide_4_mixed.wav");
    divide(input_path, output_path, 4, 0.3)?;
    println!("   Created: divide_4_mixed.wav");
    println!("   Effect: Deep sub-bass, 30% mix");

    // 3. Clipping Distortion
    println!("\n5. Hard Clipping:");
    let output_path = Path::new("crates/cdp-distort/examples/hard_clip.wav");
    overload(input_path, output_path, 0.5, 3.0, ClipType::Hard)?;
    println!("   Created: hard_clip.wav");
    println!("   Effect: Digital distortion, harsh");

    println!("\n6. Soft Clipping:");
    let output_path = Path::new("crates/cdp-distort/examples/soft_clip.wav");
    overload(input_path, output_path, 0.6, 2.5, ClipType::Soft)?;
    println!("   Created: soft_clip.wav");
    println!("   Effect: Smooth saturation, warm");

    println!("\n7. Tube Saturation:");
    let output_path = Path::new("crates/cdp-distort/examples/tube_saturation.wav");
    overload(input_path, output_path, 0.7, 2.0, ClipType::Tube)?;
    println!("   Created: tube_saturation.wav");
    println!("   Effect: Analog-style warmth");

    println!("\n8. Asymmetric Clipping:");
    let output_path = Path::new("crates/cdp-distort/examples/asymmetric_clip.wav");
    overload(input_path, output_path, 0.5, 3.5, ClipType::Asymmetric)?;
    println!("   Created: asymmetric_clip.wav");
    println!("   Effect: Even harmonics, guitar amp-like");

    // Combined effects
    println!("\n9. Extreme Distortion Chain:");
    let temp_path = Path::new("crates/cdp-distort/examples/temp.wav");
    let output_path = Path::new("crates/cdp-distort/examples/extreme_chain.wav");

    // First multiply
    multiply(input_path, temp_path, 3.0, 0.7)?;
    // Then overdrive
    overload(temp_path, output_path, 0.4, 5.0, ClipType::Tube)?;
    fs::remove_file(temp_path)?;

    println!("   Created: extreme_chain.wav");
    println!("   Effect: Heavy distortion with harmonics");

    println!("\nTips:");
    println!("- Multiply adds upper harmonics (brightness)");
    println!("- Divide adds subharmonics (bass/warmth)");
    println!("- Hard clipping = digital/harsh distortion");
    println!("- Soft clipping = analog/smooth saturation");
    println!("- Tube saturation = vintage warmth");
    println!("- Chain effects for complex textures");
    println!("- Use mix parameter to blend with dry signal");

    Ok(())
}
