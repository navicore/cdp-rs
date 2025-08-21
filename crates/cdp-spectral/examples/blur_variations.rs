//! Example demonstrating creative blur variations

use cdp_pvoc::{pvoc_anal, pvoc_synth};
use cdp_spectral::{blur, blur_varying};
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Creative Blur Variations");
    println!("========================");

    // Create examples directory
    let examples_dir = Path::new("crates/cdp-spectral/examples");
    fs::create_dir_all(examples_dir)?;

    // Generate samples
    println!("\nGenerating sample audio...");
    Command::new("cargo")
        .args([
            "run",
            "-p",
            "cdp-housekeep",
            "--example",
            "generate_samples",
        ])
        .output()?;

    // Use a complex sample
    let input_wav = Path::new("crates/cdp-housekeep/examples/complex_tone.wav");
    if !input_wav.exists() {
        // Fall back to any available sample
        let alt_input = Path::new("crates/cdp-housekeep/examples/sine_tone.wav");
        if alt_input.exists() {
            fs::copy(alt_input, examples_dir.join("input.wav"))?;
        } else {
            eprintln!("No sample files found. Please run generate_samples first.");
            return Ok(());
        }
    } else {
        fs::copy(input_wav, examples_dir.join("input.wav"))?;
    }

    let input_wav = examples_dir.join("input.wav");
    let ana_file = examples_dir.join("input.ana");

    // Convert to spectral
    println!("\nConverting to spectral domain...");
    pvoc_anal(&input_wav, &ana_file, 1, Some(2048), Some(4))?;

    // Example 1: Cascaded blur (blur of blur)
    println!("\n1. Cascaded Blur:");
    let blur1 = examples_dir.join("blur_stage1.ana");
    let blur2 = examples_dir.join("blur_cascade.ana");

    blur(&ana_file, &blur1, 5)?;
    blur(&blur1, &blur2, 5)?;
    println!("   Created: {}", blur2.display());
    println!("   Effect: Double-blurred for extra smoothness");

    // Example 2: Time-varying blur (gradual increase)
    println!("\n2. Time-Varying Blur (gradual increase):");
    let varying_blur = examples_dir.join("varying_blur.ana");
    let blur_envelope = vec![
        (0.0, 1),  // Start with no blur
        (1.0, 5),  // Increase to medium blur
        (2.0, 15), // Increase to heavy blur
        (3.0, 25), // Peak blur
        (4.0, 5),  // Return to medium
        (5.0, 1),  // End with no blur
    ];
    blur_varying(&ana_file, &varying_blur, &blur_envelope)?;
    println!("   Created: {}", varying_blur.display());
    println!("   Effect: Dynamic blur that changes over time");

    // Example 3: Selective frequency blur (using extract + blur + combine)
    println!("\n3. Frequency-Selective Blur:");
    // This would require extracting frequency bands, blurring them differently,
    // and recombining - demonstrating the concept
    let low_band = examples_dir.join("low_band.ana");
    let high_band = examples_dir.join("high_band.ana");

    // Extract low frequencies (0-1000 Hz)
    Command::new("cargo")
        .args([
            "run",
            "--bin",
            "pvoc",
            "--",
            "extract",
            ana_file.to_str().unwrap(),
            low_band.to_str().unwrap(),
            "0",
            "1000",
        ])
        .output()?;

    // Extract high frequencies (1000-10000 Hz)
    Command::new("cargo")
        .args([
            "run",
            "--bin",
            "pvoc",
            "--",
            "extract",
            ana_file.to_str().unwrap(),
            high_band.to_str().unwrap(),
            "1000",
            "10000",
        ])
        .output()?;

    // Blur bands differently
    let low_blurred = examples_dir.join("low_blurred.ana");
    let high_blurred = examples_dir.join("high_blurred.ana");

    if low_band.exists() {
        blur(&low_band, &low_blurred, 15)?; // Heavy blur on lows
        println!("   Low band heavily blurred: {}", low_blurred.display());
    }

    if high_band.exists() {
        blur(&high_band, &high_blurred, 3)?; // Light blur on highs
        println!("   High band lightly blurred: {}", high_blurred.display());
    }

    // Example 4: Rhythmic blur pattern
    println!("\n4. Rhythmic Blur Pattern:");
    let rhythmic_blur = examples_dir.join("rhythmic_blur.ana");
    let rhythm_pattern = vec![
        (0.0, 1),
        (0.25, 11),
        (0.5, 1),
        (0.75, 11),
        (1.0, 1),
        (1.25, 11),
        (1.5, 1),
        (1.75, 11),
        (2.0, 1),
    ];
    blur_varying(&ana_file, &rhythmic_blur, &rhythm_pattern)?;
    println!("   Created: {}", rhythmic_blur.display());
    println!("   Effect: Pulsating blur effect synced to rhythm");

    // Convert all to audio
    println!("\nConverting to audio...");
    for (ana, name) in &[
        (&blur2, "cascade_blur.wav"),
        (&varying_blur, "varying_blur.wav"),
        (&rhythmic_blur, "rhythmic_blur.wav"),
    ] {
        if ana.exists() {
            let output_wav = examples_dir.join(name);
            pvoc_synth(ana, &output_wav)?;
            println!("   Created: {}", output_wav.display());
        }
    }

    println!("\nCreative Tips:");
    println!("- Cascade multiple blur passes for extreme smoothing");
    println!("- Use time-varying blur to create dynamic textures");
    println!("- Apply different blur amounts to different frequency bands");
    println!("- Create rhythmic patterns with alternating blur values");
    println!("- Combine blur with pitch shift for ethereal effects");

    Ok(())
}
