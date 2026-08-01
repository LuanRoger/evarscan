use std::time::Instant;

use clap::Parser;
use evarscan::cli::Args;
use evarscan::file::write_result_env_metadata;
use evarscan::patterns::RESULT_FILE;
use evarscan::sniffer::scan_folder;

fn main() {
    let args = Args::parse();
    let start = Instant::now();

    let result = scan_folder(&args.path);

    let execution_time = start.elapsed();
    println!("Execution time: {:?}", execution_time);
    match &result {
        Ok(scan) => println!("Found: {} entries", scan.processed_files()),
        Err(e) => println!("Error: {}", e.0),
    }

    if let Ok(scan) = result
        && args.write
    {
        match write_result_env_metadata(&scan) {
            Ok(_) => {
                println!("Result saved in {}", RESULT_FILE)
            }
            Err(e) => println!("Error: {}", e.0),
        };
    }
}
