//! CDP pvoc command-line interface

use std::env;
use std::path::Path;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        process::exit(1);
    }

    match args[1].as_str() {
        "anal" => handle_anal(&args[2..]),
        "synth" => handle_synth(&args[2..]),
        "extract" => handle_extract(&args[2..]),
        _ => {
            print_usage();
            process::exit(1);
        }
    }
}

fn print_usage() {
    eprintln!("CDP Release 7.1 2016");
    eprintln!("USAGE: pvoc NAME (mode) infile outfile (parameters)");
    eprintln!();
    eprintln!("where NAME can be any one of");
    eprintln!();
    eprintln!("anal   synth 	extract");
    eprintln!();
    eprintln!("Type 'pvoc anal'  for more info on pvoc anal option... ETC.");
}

fn handle_anal(args: &[String]) {
    if args.is_empty() {
        eprintln!("CDP Release 7.1 2016");
        eprintln!("CONVERT SOUNDFILE TO SPECTRAL FILE");
        eprintln!();
        eprintln!("USAGE: pvoc anal  mode infile outfile [-cpoints] [-ooverlap]");
        eprintln!();
        eprintln!("MODES ARE....");
        eprintln!("1) STANDARD ANALYSIS");
        eprintln!("2) OUTPUT SPECTRAL ENVELOPE VALS ONLY");
        eprintln!("3) OUTPUT SPECTRAL MAGNITUDE VALS ONLY");
        eprintln!("POINTS   No of analysis points (2-32768 (power of 2)): default 1024");
        eprintln!("         More points give better freq resolution");
        eprintln!("         but worse time-resolution (e.g. rapidly changing spectrum).");
        eprintln!("OVERLAP  Filter overlap factor (1-4): default 3");
        process::exit(1);
    }

    if args.len() < 3 {
        eprintln!("ERROR: Insufficient arguments");
        process::exit(1);
    }

    let mode: u32 = match args[0].parse() {
        Ok(m) if (1..=3).contains(&m) => m,
        _ => {
            eprintln!("ERROR: Invalid mode (must be 1-3)");
            process::exit(1);
        }
    };

    let infile = Path::new(&args[1]);
    let outfile = Path::new(&args[2]);

    // Parse optional parameters
    let mut channels = None;
    let mut overlap = None;

    let mut i = 3;
    while i < args.len() {
        if args[i].starts_with("-c") {
            if let Ok(c) = args[i][2..].parse::<u32>() {
                // Verify power of 2
                if (2..=32768).contains(&c) && (c & (c - 1)) == 0 {
                    channels = Some(c);
                } else {
                    eprintln!("ERROR: Channels must be power of 2 between 2 and 32768");
                    process::exit(1);
                }
            }
        } else if args[i].starts_with("-o") {
            if let Ok(o) = args[i][2..].parse::<u32>() {
                if (1..=4).contains(&o) {
                    overlap = Some(o);
                } else {
                    eprintln!("ERROR: Overlap must be between 1 and 4");
                    process::exit(1);
                }
            }
        }
        i += 1;
    }

    // Call the library function
    eprintln!("analysis/synthesis beginning");
    match cdp_pvoc::pvoc_anal(infile, outfile, mode, channels, overlap) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("ERROR: {}", e);
            process::exit(1);
        }
    }
}

fn handle_synth(args: &[String]) {
    if args.is_empty() {
        eprintln!("CDP Release 7.1 2016");
        eprintln!("CONVERT SPECTRAL FILE TO SOUNDFILE");
        eprintln!();
        eprintln!("USAGE: pvoc synth infile outfile");
        process::exit(1);
    }

    if args.len() < 2 {
        eprintln!("ERROR: Insufficient arguments");
        process::exit(1);
    }

    let infile = Path::new(&args[0]);
    let outfile = Path::new(&args[1]);

    eprintln!("analysis/synthesis beginning");
    match cdp_pvoc::pvoc_synth(infile, outfile) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("ERROR: {}", e);
            process::exit(1);
        }
    }
}

fn handle_extract(args: &[String]) {
    if args.is_empty() {
        eprintln!("CDP Release 7.1 2016");
        eprintln!("EXTRACT FREQUENCY BAND FROM SPECTRAL FILE");
        eprintln!();
        eprintln!("USAGE: pvoc extract infile outfile lo_freq hi_freq");
        process::exit(1);
    }

    if args.len() < 4 {
        eprintln!("ERROR: Insufficient arguments");
        process::exit(1);
    }

    let infile = Path::new(&args[0]);
    let outfile = Path::new(&args[1]);
    let lo_freq: f32 = match args[2].parse() {
        Ok(f) => f,
        Err(_) => {
            eprintln!("ERROR: Invalid lo_freq");
            process::exit(1);
        }
    };
    let hi_freq: f32 = match args[3].parse() {
        Ok(f) => f,
        Err(_) => {
            eprintln!("ERROR: Invalid hi_freq");
            process::exit(1);
        }
    };

    match cdp_pvoc::pvoc_extract(infile, outfile, lo_freq, hi_freq) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("ERROR: {}", e);
            process::exit(1);
        }
    }
}
