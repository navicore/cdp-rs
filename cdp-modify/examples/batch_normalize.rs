//! Example: Batch normalize multiple audio files
//!
//! First generate the sample files:
//!   cargo run -p cdp-housekeep --example generate_samples
//!
//! Then run this example:
//!   cargo run -p cdp-modify --example batch_normalize

use cdp_modify::loudness;
use std::path::Path;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("CDP-RS Batch Normalization Example\n");
    println!("===================================\n");

    // Input files to normalize (in current directory)
    let input_files = [
        "mono_sine.wav",
        "quiet_sine.wav",
        "white_noise.wav",
        "chirp.wav",
    ];

    // Check if sample files exist
    if !Path::new(&input_files[0]).exists() {
        println!("Sample files not found!");
        println!("Please run: cargo run -p cdp-housekeep --example generate_samples");
        return Ok(());
    }

    // Target normalization level (95% to leave headroom)
    let target_level: f32 = 0.95;

    println!(
        "Normalizing {} files to {:.1}% ({:.2} dB)",
        input_files.len(),
        target_level * 100.0,
        20.0 * target_level.log10()
    );
    println!();

    // Process each file
    for (i, input_path) in input_files.iter().enumerate() {
        let input = Path::new(input_path);
        let filename = input.file_stem().unwrap().to_string_lossy();
        let output_path = format!("{}_normalized.wav", filename);
        let output = Path::new(&output_path);

        print!(
            "[{}/{}] Normalizing {}... ",
            i + 1,
            input_files.len(),
            input.file_name().unwrap().to_string_lossy()
        );

        match loudness::normalize(input, output, Some(target_level)) {
            Ok(_) => println!("✓"),
            Err(e) => {
                println!("✗ Error: {}", e);
                continue;
            }
        }
    }

    println!("\n✓ Batch normalization complete!");
    println!("\nNormalized files created:");
    println!("  - mono_sine_normalized.wav");
    println!("  - quiet_sine_normalized.wav");
    println!("  - white_noise_normalized.wav");
    println!("  - chirp_normalized.wav");
    println!(
        "\nAll files normalized to {:.1}% of maximum level",
        target_level * 100.0
    );

    Ok(())
}
