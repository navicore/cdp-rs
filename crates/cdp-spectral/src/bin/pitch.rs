//! Simple pitch shift command-line interface

use cdp_spectral::{pitch_shift, pitch_shift_formant, semitones_to_factor};
use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 4 {
        eprintln!("CDP-RS Pitch Shift");
        eprintln!();
        eprintln!("USAGE: pitch infile outfile shift [options]");
        eprintln!();
        eprintln!("  shift: Pitch shift in semitones (12 = octave up, -12 = octave down)");
        eprintln!("         or as ratio (2.0 = octave up, 0.5 = octave down)");
        eprintln!();
        eprintln!("OPTIONS:");
        eprintln!("  -f    Preserve formants (spectral envelope)");
        eprintln!();
        eprintln!("EXAMPLES:");
        eprintln!("  pitch input.ana output.ana 12      # Octave up");
        eprintln!("  pitch input.ana output.ana -7      # Perfect fifth down");
        eprintln!("  pitch input.ana output.ana 2.0     # Octave up (ratio)");
        eprintln!("  pitch input.ana output.ana 3 -f    # Minor third up, preserve formants");
        std::process::exit(1);
    }

    let infile = Path::new(&args[1]);
    let outfile = Path::new(&args[2]);
    let shift_str = &args[3];

    // Check for formant preservation flag
    let preserve_formants = args.len() > 4 && args[4] == "-f";

    // Parse shift value (could be semitones or ratio)
    let shift_factor = if shift_str.contains('.') {
        // Treat as ratio
        shift_str.parse::<f64>().unwrap_or_else(|_| {
            eprintln!("ERROR: Invalid shift ratio: {}", shift_str);
            std::process::exit(1);
        })
    } else {
        // Treat as semitones
        let semitones = shift_str.parse::<f64>().unwrap_or_else(|_| {
            eprintln!("ERROR: Invalid semitone value: {}", shift_str);
            std::process::exit(1);
        });
        semitones_to_factor(semitones)
    };

    if shift_factor <= 0.0 || !(0.1..=10.0).contains(&shift_factor) {
        eprintln!(
            "ERROR: Shift factor must be between 0.1 and 10.0 (got {})",
            shift_factor
        );
        std::process::exit(1);
    }

    eprintln!("CDP-RS Pitch Shift");
    eprintln!("Shifting by factor: {:.3}", shift_factor);
    if preserve_formants {
        eprintln!("Preserving formants");
    }
    eprintln!("Processing...");

    let result = if preserve_formants {
        pitch_shift_formant(infile, outfile, shift_factor, true)
    } else {
        pitch_shift(infile, outfile, shift_factor)
    };

    match result {
        Ok(()) => {
            eprintln!("COMPLETED");
            std::process::exit(0);
        }
        Err(e) => {
            eprintln!("ERROR: {}", e);
            std::process::exit(1);
        }
    }
}
