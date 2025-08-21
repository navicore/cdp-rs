//! Example demonstrating creative stretch variations

use cdp_pvoc::{pvoc_anal, pvoc_synth};
use cdp_spectral::{blur, stretch_time, stretch_time_varying};
use std::fs;
use std::path::Path;
use std::process::Command;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Creative Stretch Variations");
    println!("===========================");

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

    // Get input sample
    let input_wav = Path::new("crates/cdp-housekeep/examples/sawtooth_tone.wav");
    if !input_wav.exists() {
        let alt_input = Path::new("crates/cdp-housekeep/examples/sine_tone.wav");
        if !alt_input.exists() {
            eprintln!("No sample files found. Please run generate_samples first.");
            return Ok(());
        }
        fs::copy(alt_input, examples_dir.join("input.wav"))?;
    } else {
        fs::copy(input_wav, examples_dir.join("input.wav"))?;
    }

    let input_wav = examples_dir.join("input.wav");
    let ana_file = examples_dir.join("input.ana");

    // Convert to spectral
    println!("\nConverting to spectral domain...");
    pvoc_anal(&input_wav, &ana_file, 1, Some(2048), Some(4))?;

    // Example 1: Time-varying stretch (accelerando)
    println!("\n1. Accelerando (gradual speed up):");
    let accelerando = examples_dir.join("accelerando.ana");
    let stretch_envelope = vec![
        (0.0, 2.0), // Start slow (2x stretch)
        (2.0, 1.0), // Normal speed in middle
        (4.0, 0.5), // End fast (2x speed)
    ];
    stretch_time_varying(&ana_file, &accelerando, &stretch_envelope)?;
    println!("   Created: {}", accelerando.display());
    println!("   Effect: Gradually speeds up over time");

    // Example 2: Time-varying stretch (ritardando)
    println!("\n2. Ritardando (gradual slow down):");
    let ritardando = examples_dir.join("ritardando.ana");
    let stretch_envelope = vec![
        (0.0, 0.5), // Start fast
        (2.0, 1.0), // Normal speed
        (4.0, 3.0), // End very slow
    ];
    stretch_time_varying(&ana_file, &ritardando, &stretch_envelope)?;
    println!("   Created: {}", ritardando.display());
    println!("   Effect: Gradually slows down over time");

    // Example 3: Stretch + Blur combo (smooth extreme stretch)
    println!("\n3. Smooth Extreme Stretch (stretch + blur):");
    let extreme_stretch = examples_dir.join("extreme_stretch.ana");
    let smoothed_stretch = examples_dir.join("smoothed_stretch.ana");

    // First apply extreme stretch
    stretch_time(&ana_file, &extreme_stretch, 8.0)?;
    // Then blur to smooth artifacts
    blur(&extreme_stretch, &smoothed_stretch, 7)?;
    println!("   Created: {}", smoothed_stretch.display());
    println!("   Effect: Extreme time expansion with smoothing");

    // Example 4: Rhythmic stretch pattern
    println!("\n4. Rhythmic Stretch Pattern:");
    let rhythmic_stretch = examples_dir.join("rhythmic_stretch.ana");
    let rhythm_pattern = vec![
        (0.0, 1.0),
        (0.5, 2.0),
        (1.0, 1.0),
        (1.5, 2.0),
        (2.0, 1.0),
        (2.5, 0.5),
        (3.0, 1.0),
    ];
    stretch_time_varying(&ana_file, &rhythmic_stretch, &rhythm_pattern)?;
    println!("   Created: {}", rhythmic_stretch.display());
    println!("   Effect: Creates rhythmic time variations");

    // Example 5: Reverse stretch effect (fast-slow-fast)
    println!("\n5. Elastic Time (fast-slow-fast):");
    let elastic = examples_dir.join("elastic.ana");
    let elastic_pattern = vec![(0.0, 0.5), (1.0, 3.0), (2.0, 0.5), (3.0, 3.0), (4.0, 0.5)];
    stretch_time_varying(&ana_file, &elastic, &elastic_pattern)?;
    println!("   Created: {}", elastic.display());
    println!("   Effect: Elastic, bouncing time effect");

    // Convert all to audio
    println!("\nConverting to audio...");
    for (ana, name) in &[
        (&accelerando, "accelerando.wav"),
        (&ritardando, "ritardando.wav"),
        (&smoothed_stretch, "smoothed_stretch.wav"),
        (&rhythmic_stretch, "rhythmic_stretch.wav"),
        (&elastic, "elastic.wav"),
    ] {
        if ana.exists() {
            let output_wav = examples_dir.join(name);
            pvoc_synth(ana, &output_wav)?;
            println!("   Created: {}", output_wav.display());
        }
    }

    println!("\nCreative Tips:");
    println!("- Use time-varying stretch for musical expression");
    println!("- Combine stretch with blur for smoother extreme effects");
    println!("- Create rhythmic patterns with alternating stretch values");
    println!("- Use elastic stretching for bouncing, rubber-band effects");
    println!("- Apply different stretches to different frequency bands");

    Ok(())
}
