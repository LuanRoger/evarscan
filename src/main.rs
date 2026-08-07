use std::time::Instant;

use clap::Parser;
use evarscan::cli::Args;
use evarscan::file::write_result_env_metadata;
use evarscan::patterns::RESULT_FILE;
use evarscan::sniffer::scan_folder;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _profiler = dhat::Profiler::new_heap();

    let args = Args::parse();
    let start = Instant::now();

    let result = scan_folder(&args.path, args.parallel);

    let execution_time = start.elapsed();
    println!("Execution time: {:?}", execution_time);
    match &result {
        Ok(scan) => println!("Scanned: {} files", scan.processed_files()),
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
