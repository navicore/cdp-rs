//! Bass enhancement and sub-bass generation examples

use cdp_distort::{divide, multiply, overload, ClipType};
use hound::{SampleFormat, WavSpec, WavWriter};
use std::f32::consts::PI;
use std::fs;
use std::path::Path;

fn generate_bass_line(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let sample_rate = 44100;
    let duration = 4.0;
    let num_samples = (sample_rate as f32 * duration) as usize;

    let mut samples = Vec::with_capacity(num_samples);

    // Bass line pattern (simple groove)
    let notes = [
        (41.2, 0.5), // E1
        (41.2, 0.5), // E1
        (55.0, 0.5), // A1
        (41.2, 0.5), // E1
        (61.7, 0.5), // B1
        (55.0, 0.5), // A1
        (41.2, 0.5), // E1
        (41.2, 0.5), // E1
    ];

    let note_duration = 0.5;
    let samples_per_note = (sample_rate as f32 * note_duration) as usize;

    for (freq, _) in notes.iter().cycle().take(8) {
        for i in 0..samples_per_note {
            let t = i as f32 / sample_rate as f32;

            // Bass with harmonics
            let fundamental = (2.0 * PI * freq * t).sin() * 0.6;
            let second = (2.0 * PI * freq * 2.0 * t).sin() * 0.2;
            let third = (2.0 * PI * freq * 3.0 * t).sin() * 0.1;

            // Pluck envelope
            let envelope = (-t * 4.0).exp();

            let sample = (fundamental + second + third) * envelope;
            samples.push(sample);

            if samples.len() >= num_samples {
                break;
            }
        }
        if samples.len() >= num_samples {
            break;
        }
    }

    // Ensure we have exactly the right number of samples
    samples.resize(num_samples, 0.0);

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
    println!("Bass Enhancement Examples");
    println!("=========================\n");

    // Create examples directory
    fs::create_dir_all("crates/cdp-distort/examples")?;

    // Generate bass line
    let input_path = Path::new("crates/cdp-distort/examples/bass_line.wav");
    println!("Generating bass line...");
    generate_bass_line(input_path)?;

    // 1. Sub-bass enhancement (octave down)
    println!("\n1. Sub-Bass Enhancement:");
    let output_path = Path::new("crates/cdp-distort/examples/sub_bass.wav");
    divide(input_path, output_path, 2, 0.4)?;
    println!("   Created: sub_bass.wav");
    println!("   Effect: Adds sub-octave for deep bass");
    println!("   Use: EDM, hip-hop, dubstep");

    // 2. Bass amp warmth
    println!("\n2. Warm Bass Amp:");
    let output_path = Path::new("crates/cdp-distort/examples/warm_bass.wav");
    overload(input_path, output_path, 0.8, 1.5, ClipType::Tube)?;
    println!("   Created: warm_bass.wav");
    println!("   Effect: Tube amp warmth");
    println!("   Use: Jazz, blues, vintage rock");

    // 3. Aggressive rock bass
    println!("\n3. Rock Bass Distortion:");
    let output_path = Path::new("crates/cdp-distort/examples/rock_bass.wav");
    overload(input_path, output_path, 0.5, 4.0, ClipType::Asymmetric)?;
    println!("   Created: rock_bass.wav");
    println!("   Effect: Gritty rock bass tone");
    println!("   Use: Rock, metal, punk");

    // 4. Synth bass (harmonics)
    println!("\n4. Synth Bass Enhancement:");
    let output_path = Path::new("crates/cdp-distort/examples/synth_bass.wav");
    multiply(input_path, output_path, 2.0, 0.6)?;
    println!("   Created: synth_bass.wav");
    println!("   Effect: Added harmonics for brightness");
    println!("   Use: Electronic music, modern pop");

    // 5. Deep sub generator
    println!("\n5. Deep Sub Generator:");
    let output_path = Path::new("crates/cdp-distort/examples/deep_sub.wav");
    divide(input_path, output_path, 4, 0.3)?;
    println!("   Created: deep_sub.wav");
    println!("   Effect: Two octaves down sub-bass");
    println!("   Use: Cinema, trap, bass drops");

    // 6. Fuzz bass
    println!("\n6. Fuzz Bass:");
    let output_path = Path::new("crates/cdp-distort/examples/fuzz_bass.wav");
    overload(input_path, output_path, 0.2, 10.0, ClipType::Hard)?;
    println!("   Created: fuzz_bass.wav");
    println!("   Effect: Heavy fuzz distortion");
    println!("   Use: Stoner rock, doom metal");

    // 7. Parallel processing (clean + distorted)
    println!("\n7. Parallel Bass Processing:");
    let temp_distorted = Path::new("crates/cdp-distort/examples/temp_dist.wav");

    // Create distorted version
    overload(input_path, temp_distorted, 0.4, 5.0, ClipType::Tube)?;

    // Mix with original (50/50 for parallel processing effect)
    // In production, you'd mix these two signals
    println!("   Created: temp_dist.wav (mix with original for parallel processing)");
    println!("   Effect: Maintains clean low end with distorted harmonics");
    println!("   Use: Modern production technique");

    // 8. 808-style bass
    println!("\n8. 808-Style Bass:");
    let temp1 = Path::new("crates/cdp-distort/examples/temp1.wav");
    let output_path = Path::new("crates/cdp-distort/examples/bass_808.wav");

    // Add sub
    divide(input_path, temp1, 2, 0.5)?;
    // Then saturate
    overload(temp1, output_path, 0.6, 2.5, ClipType::Soft)?;
    fs::remove_file(temp1)?;

    println!("   Created: bass_808.wav");
    println!("   Effect: 808-style bass with sub and saturation");
    println!("   Use: Hip-hop, trap, modern pop");

    println!("\nBass Processing Guide:");
    println!("┌─────────────────┬────────────┬──────────┬─────────────┐");
    println!("│ Style           │ Process    │ Amount   │ Character   │");
    println!("├─────────────────┼────────────┼──────────┼─────────────┤");
    println!("│ Sub Bass        │ Divide /2  │ 30-50%   │ Deep        │");
    println!("│ Warm/Vintage    │ Tube       │ Low      │ Round       │");
    println!("│ Rock/Metal      │ Asymmetric │ Medium   │ Aggressive  │");
    println!("│ Modern/EDM      │ Multiply   │ Medium   │ Bright      │");
    println!("│ Fuzz            │ Hard Clip  │ High     │ Destroyed   │");
    println!("└─────────────────┴────────────┴──────────┴─────────────┘");

    println!("\nFrequency Guidelines:");
    println!("- Sub-bass: 20-60 Hz (felt more than heard)");
    println!("- Bass: 60-250 Hz (fundamental frequencies)");
    println!("- Low-mids: 250-500 Hz (warmth and body)");
    println!("- Use divide for sub generation");
    println!("- Use multiply for harmonic enhancement");

    println!("\nMixing Tips:");
    println!("- High-pass filter before distortion to control low end");
    println!("- Use parallel processing to maintain clean lows");
    println!("- Compress after distortion for consistency");
    println!("- Layer clean sub with distorted mids/highs");
    println!("- Monitor on different systems (headphones, speakers, sub)");

    Ok(())
}
