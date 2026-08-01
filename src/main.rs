use std::time::Instant;

use clap::Parser;
use evarscan::cli::Args;
use evarscan::sniffer::scan_folder;

fn main() {
    let args = Args::parse();
    let start = Instant::now();

    let result = scan_folder(&args.path);

    let execution_time = start.elapsed();
    println!("Execution time: {:?}", execution_time);
    match result {
        Ok(scan) => println!("Found: {} entries", scan.processed_files()),
        Err(e) => println!("Error: {}", e.0),
    }
}
