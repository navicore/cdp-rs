//! Example demonstrating spectral blurring effects

use cdp_pvoc::pvoc_anal;
use cdp_spectral::blur;
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Spectral Blur Examples");
    println!("======================");

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

    // Use a harmonic-rich sample for better blur demonstration
    let input_wav = Path::new("crates/cdp-housekeep/examples/sawtooth_tone.wav");
    if !input_wav.exists() {
        eprintln!("Sample file not found. Please run cdp-housekeep generate_samples first.");
        return Ok(());
    }

    // Convert to spectral domain
    println!("\nConverting to spectral domain...");
    let ana_file = examples_dir.join("input.ana");
    pvoc_anal(input_wav, &ana_file, 1, Some(1024), Some(3))?;

    // Example 1: Light blur (3 windows)
    println!("\n1. Light blur (3 windows):");
    let light_blur = examples_dir.join("light_blur.ana");
    blur(&ana_file, &light_blur, 3)?;
    println!("   Created: {}", light_blur.display());
    println!("   Effect: Subtle smoothing of spectral changes");

    // Example 2: Medium blur (7 windows)
    println!("\n2. Medium blur (7 windows):");
    let medium_blur = examples_dir.join("medium_blur.ana");
    blur(&ana_file, &medium_blur, 7)?;
    println!("   Created: {}", medium_blur.display());
    println!("   Effect: Noticeable spectral smearing, transients softened");

    // Example 3: Heavy blur (15 windows)
    println!("\n3. Heavy blur (15 windows):");
    let heavy_blur = examples_dir.join("heavy_blur.ana");
    blur(&ana_file, &heavy_blur, 15)?;
    println!("   Created: {}", heavy_blur.display());
    println!("   Effect: Strong spectral averaging, wash-like texture");

    // Example 4: Extreme blur (31 windows)
    println!("\n4. Extreme blur (31 windows):");
    let extreme_blur = examples_dir.join("extreme_blur.ana");
    blur(&ana_file, &extreme_blur, 31)?;
    println!("   Created: {}", extreme_blur.display());
    println!("   Effect: Very smooth, drone-like spectrum");

    // Convert back to audio for listening
    println!("\nConverting blurred spectra back to audio...");

    // Use pvoc synth to convert back
    for (ana, wav_name) in &[
        (&light_blur, "light_blur.wav"),
        (&medium_blur, "medium_blur.wav"),
        (&heavy_blur, "heavy_blur.wav"),
        (&extreme_blur, "extreme_blur.wav"),
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
    println!("- Blur works best on sounds with clear transients or rapid spectral changes");
    println!("- Higher blur values create more sustained, drone-like textures");
    println!("- Try blur on percussive sounds for interesting smearing effects");
    println!("- Combine with other spectral processes for complex textures");

    Ok(())
}
