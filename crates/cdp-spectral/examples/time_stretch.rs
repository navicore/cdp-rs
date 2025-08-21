//! Example demonstrating time-stretching effects

use cdp_pvoc::pvoc_anal;
use cdp_spectral::stretch_time;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Time Stretching Examples");
    println!("========================");

    // Create examples directory if it doesn't exist
    let examples_dir = Path::new("crates/cdp-spectral/examples");
    fs::create_dir_all(examples_dir)?;

    // Generate sample audio if needed
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

    // Use a rhythmic sample for better stretch demonstration
    let input_wav = Path::new("crates/cdp-housekeep/examples/complex_tone.wav");
    if !input_wav.exists() {
        // Fall back to sine tone if complex tone doesn't exist
        let alt_input = Path::new("crates/cdp-housekeep/examples/sine_tone.wav");
        if !alt_input.exists() {
            eprintln!("Sample file not found. Please run cdp-housekeep generate_samples first.");
            return Ok(());
        }
        fs::copy(alt_input, examples_dir.join("input.wav"))?;
    } else {
        fs::copy(input_wav, examples_dir.join("input.wav"))?;
    }

    let input_wav = examples_dir.join("input.wav");

    // Convert to spectral domain
    println!("\nConverting to spectral domain...");
    let ana_file = examples_dir.join("input.ana");
    pvoc_anal(&input_wav, &ana_file, 1, Some(2048), Some(4))?;

    // Example 1: Slow down 2x (half speed)
    println!("\n1. Slow down 2x (half speed):");
    let slow_2x = examples_dir.join("slow_2x.ana");
    stretch_time(&ana_file, &slow_2x, 2.0)?;
    println!("   Created: {}", slow_2x.display());
    println!("   Effect: Doubles duration, maintains pitch");

    // Example 2: Slow down 4x (quarter speed)
    println!("\n2. Slow down 4x (quarter speed):");
    let slow_4x = examples_dir.join("slow_4x.ana");
    stretch_time(&ana_file, &slow_4x, 4.0)?;
    println!("   Created: {}", slow_4x.display());
    println!("   Effect: Quadruples duration, extreme time expansion");

    // Example 3: Speed up 2x (double speed)
    println!("\n3. Speed up 2x (double speed):");
    let fast_2x = examples_dir.join("fast_2x.ana");
    stretch_time(&ana_file, &fast_2x, 0.5)?;
    println!("   Created: {}", fast_2x.display());
    println!("   Effect: Halves duration, maintains pitch");

    // Example 4: Slight slow (1.5x)
    println!("\n4. Slight slow (1.5x):");
    let slow_1_5x = examples_dir.join("slow_1_5x.ana");
    stretch_time(&ana_file, &slow_1_5x, 1.5)?;
    println!("   Created: {}", slow_1_5x.display());
    println!("   Effect: Subtle time expansion, natural feel");

    // Example 5: Slight fast (0.75x)
    println!("\n5. Slight fast (0.75x):");
    let fast_1_33x = examples_dir.join("fast_1_33x.ana");
    stretch_time(&ana_file, &fast_1_33x, 0.75)?;
    println!("   Created: {}", fast_1_33x.display());
    println!("   Effect: Subtle time compression");

    // Convert back to audio for listening
    println!("\nConverting stretched spectra back to audio...");

    // Use pvoc synth to convert back
    for (ana, wav_name) in &[
        (&slow_2x, "slow_2x.wav"),
        (&slow_4x, "slow_4x.wav"),
        (&fast_2x, "fast_2x.wav"),
        (&slow_1_5x, "slow_1_5x.wav"),
        (&fast_1_33x, "fast_1_33x.wav"),
    ] {
        let output_wav = examples_dir.join(wav_name);
        Command::new("cargo")
            .args([
                "run",
                "--bin",
                "pvoc",
                "--",
                "synth",
                ana.to_str().unwrap(),
                output_wav.to_str().unwrap(),
            ])
            .output()?;
        println!("   Created audio: {}", output_wav.display());
    }

    println!("\nTips:");
    println!("- Time stretching maintains pitch while changing duration");
    println!("- Extreme stretches (>4x) can introduce artifacts");
    println!("- Works best on harmonic material");
    println!("- Combine with blur for smoother extreme stretches");
    println!("- Use smaller stretch factors (0.8-1.2) for natural results");

    Ok(())
}
