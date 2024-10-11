mod lib;
use std::io::{self, Write};
use std::fs::OpenOptions;
use std::path::Path;
use lib::process_input_file;
use std::time::Instant;

fn main() -> io::Result<()> {
    // Start the timer
    let start_time = Instant::now();
    let base_dir = "../data";
    
    // Define the output file path
    let output_file_path = "../data/weekly_summary/weekly_sales_summary.txt";

    let output_file = Path::new(output_file_path);
    if output_file.exists() {
        // Open the file with the truncate option to clear its contents
        OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(output_file_path)?;
    }

    // Call the process_input_file function
    process_input_file(base_dir, output_file_path)?;

    // Stop the timer and get the elapsed time
    let duration = start_time.elapsed();
    println!("Time elapsed: {}", duration.as_secs_f64());
    Ok(())
}
