//! CDP-compatible blur command-line interface

use cdp_spectral::blur;
use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("CDP Release 7.1 2016");
        eprintln!("blur     avrg     blur     bounce     ...other modes not implemented...");
        eprintln!();
        eprintln!("USAGE: blur NAME");
        std::process::exit(1);
    }

    let mode = &args[1];

    match mode.as_str() {
        "blur" => {
            if args.len() < 5 {
                eprintln!("CDP Release 7.1 2016");
                eprintln!("blur blur infile outfile blurring");
                eprintln!();
                eprintln!("TIME-AVERAGE THE SPECTRUM");
                eprintln!();
                eprintln!("blurring   is number of windows over which to average the spectrum.");
                eprintln!();
                eprintln!("blurring may vary over time.");
                std::process::exit(1);
            }

            let infile = Path::new(&args[2]);
            let outfile = Path::new(&args[3]);
            let blurring = args[4].parse::<u32>().unwrap_or_else(|_| {
                eprintln!("ERROR: Invalid blurring value: {}", args[4]);
                std::process::exit(1);
            });

            if blurring == 0 {
                eprintln!("ERROR: Blurring value must be greater than 0");
                std::process::exit(1);
            }

            eprintln!("CDP Release 7.1 2016");
            eprintln!("blur blur infile outfile blurring");
            eprintln!();
            eprintln!("TIME-AVERAGE THE SPECTRUM");
            eprintln!();
            eprintln!("blurring   is number of windows over which to average the spectrum.");
            eprintln!();
            eprintln!("blurring may vary over time.");
            eprintln!();
            eprintln!("spectral manipulation beginning");

            match blur(infile, outfile, blurring) {
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
        "avrg" => {
            eprintln!("CDP Release 7.1 2016");
            eprintln!("blur avrg    NOT YET IMPLEMENTED");
            std::process::exit(1);
        }
        _ => {
            eprintln!("CDP Release 7.1 2016");
            eprintln!("ERROR: Unknown mode: {}", mode);
            eprintln!();
            eprintln!("blur     avrg     blur     bounce     ...other modes not implemented...");
            eprintln!();
            eprintln!("USAGE: blur NAME");
            std::process::exit(1);
        }
    }
}
