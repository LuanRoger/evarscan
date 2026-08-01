use std::time::Instant;

use evarscan::sniffer::scan_folder;

const PROJECT_FOLDER: &str = "./src";

fn main() {
    let start = Instant::now();

    let result = scan_folder(PROJECT_FOLDER);

    let execution_time = start.elapsed();
    println!("Execution time: {:?}", execution_time);
    match result {
        Ok(scan) => println!("Found: {} entries", scan.processed_files()),
        Err(e) => println!("Error: {}", e.0),
    }
}
