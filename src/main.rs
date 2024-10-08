mod lib;
use std::io::{self, Write};
use std::fs::OpenOptions;
use std::path::Path;
use lib::process_input_file;

fn main() -> io::Result<()> {
    // Define the base directory where the branch folders are located
    let base_dir = "../data/data";
    
    // Define the output file path
    let output_file_path = "..data/data/branch_sales_summary.txt";

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

    Ok(())
}