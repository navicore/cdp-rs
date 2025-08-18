//! Thin binary wrapper for modify operations
//!
//! This exists purely for oracle validation against CDP.

use cdp_sandbox::modify;
use std::env;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        eprintln!("CDP-RS Modify (Oracle Validation Binary)");
        eprintln!("Usage: modify <operation> <mode> <infile> <outfile> [args...]");
        eprintln!("Operations: loudness, space, speed, etc.");
        process::exit(1);
    }

    let operation = &args[1];
    let mode = args[2].parse::<i32>().unwrap_or_else(|_| {
        eprintln!("ERROR: Invalid mode number");
        process::exit(1);
    });

    let op_args: Vec<&str> = args[3..].iter().map(|s| s.as_str()).collect();

    if let Err(e) = modify::modify(operation, mode, &op_args) {
        eprintln!("ERROR: {}", e);
        process::exit(1);
    }
}
