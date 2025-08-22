//! Guitar amp and effects chain examples

use cdp_distort::{overload, ClipType};
use hound::{SampleFormat, WavSpec, WavWriter};
use std::f32::consts::PI;
use std::fs;
use std::path::Path;

fn generate_guitar_note(path: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let sample_rate = 44100;
    let duration = 3.0;
    let num_samples = (sample_rate as f32 * duration) as usize;

    let mut samples = Vec::with_capacity(num_samples);

    for i in 0..num_samples {
        let t = i as f32 / sample_rate as f32;

        // E note (82.4 Hz) with harmonics like a guitar
        let fundamental = (2.0 * PI * 82.4 * t).sin() * 0.5;
        let second = (2.0 * PI * 164.8 * t).sin() * 0.3;
        let third = (2.0 * PI * 247.2 * t).sin() * 0.15;
        let fourth = (2.0 * PI * 329.6 * t).sin() * 0.1;

        // Attack/decay envelope
        let attack = (t * 20.0).min(1.0);
        let decay = (-t * 0.5).exp();
        let envelope = attack * decay;

        let sample = (fundamental + second + third + fourth) * envelope;
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
    println!("Guitar Amp Effects Examples");
    println!("===========================\n");

    // Create examples directory
    fs::create_dir_all("crates/cdp-distort/examples")?;

    // Generate guitar-like sound
    let input_path = Path::new("crates/cdp-distort/examples/guitar_note.wav");
    println!("Generating guitar note...");
    generate_guitar_note(input_path)?;

    // 1. Clean boost
    println!("\n1. Clean Boost (warm up the signal):");
    let output_path = Path::new("crates/cdp-distort/examples/clean_boost.wav");
    overload(input_path, output_path, 0.95, 1.5, ClipType::Tube)?;
    println!("   Created: clean_boost.wav");
    println!("   Settings: Low drive, high threshold");
    println!("   Sound: Slightly warmer, no distortion");

    // 2. Vintage overdrive
    println!("\n2. Vintage Overdrive:");
    let output_path = Path::new("crates/cdp-distort/examples/vintage_overdrive.wav");
    overload(input_path, output_path, 0.7, 3.0, ClipType::Tube)?;
    println!("   Created: vintage_overdrive.wav");
    println!("   Settings: Medium drive, tube saturation");
    println!("   Sound: Classic tube amp breakup");

    // 3. Modern distortion
    println!("\n3. Modern High-Gain Distortion:");
    let output_path = Path::new("crates/cdp-distort/examples/modern_distortion.wav");
    overload(input_path, output_path, 0.4, 8.0, ClipType::Asymmetric)?;
    println!("   Created: modern_distortion.wav");
    println!("   Settings: High drive, asymmetric clipping");
    println!("   Sound: Heavy metal/rock distortion");

    // 4. Fuzz pedal
    println!("\n4. Fuzz Pedal Effect:");
    let output_path = Path::new("crates/cdp-distort/examples/fuzz_pedal.wav");
    overload(input_path, output_path, 0.2, 15.0, ClipType::Hard)?;
    println!("   Created: fuzz_pedal.wav");
    println!("   Settings: Extreme drive, hard clipping");
    println!("   Sound: Classic 60s fuzz tone");

    // 5. Crunch rhythm
    println!("\n5. Crunch Rhythm Tone:");
    let output_path = Path::new("crates/cdp-distort/examples/crunch_rhythm.wav");
    overload(input_path, output_path, 0.6, 4.0, ClipType::Soft)?;
    println!("   Created: crunch_rhythm.wav");
    println!("   Settings: Moderate drive, soft clipping");
    println!("   Sound: Great for rhythm guitar");

    // 6. Lead tone
    println!("\n6. Screaming Lead Tone:");
    let output_path = Path::new("crates/cdp-distort/examples/lead_tone.wav");
    overload(input_path, output_path, 0.5, 6.0, ClipType::Tube)?;
    println!("   Created: lead_tone.wav");
    println!("   Settings: High drive with tube warmth");
    println!("   Sound: Sustaining lead guitar tone");

    // 7. Bass amp simulation
    println!("\n7. Bass Amp Simulation:");
    let output_path = Path::new("crates/cdp-distort/examples/bass_amp.wav");
    overload(input_path, output_path, 0.8, 2.0, ClipType::Asymmetric)?;
    println!("   Created: bass_amp.wav");
    println!("   Settings: Low drive, gentle asymmetric");
    println!("   Sound: Warm bass amp with slight grit");

    println!("\nAmp Settings Guide:");
    println!("┌─────────────────┬────────────┬──────────┬─────────────┐");
    println!("│ Effect          │ Threshold  │ Drive    │ Clip Type   │");
    println!("├─────────────────┼────────────┼──────────┼─────────────┤");
    println!("│ Clean Boost     │ 0.9-1.0    │ 1.0-2.0  │ Tube        │");
    println!("│ Overdrive       │ 0.6-0.8    │ 2.0-4.0  │ Tube/Soft   │");
    println!("│ Distortion      │ 0.3-0.6    │ 4.0-10.0 │ Asymmetric  │");
    println!("│ Fuzz            │ 0.1-0.3    │ 10.0-20.0│ Hard        │");
    println!("└─────────────────┴────────────┴──────────┴─────────────┘");

    println!("\nTips:");
    println!("- Lower threshold = more distortion");
    println!("- Higher drive = more gain/saturation");
    println!("- Tube = warm, musical distortion");
    println!("- Asymmetric = even harmonics (amp-like)");
    println!("- Hard clip = harsh, digital distortion");
    println!("- Soft clip = smooth, compressed distortion");

    Ok(())
}
