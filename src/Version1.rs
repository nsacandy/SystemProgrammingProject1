 mod lib;
 use lib::process_input_file;
 use std::io;
 use std::path::Path;
 use std::time::Instant;
 use std::fs::{self, OpenOptions};

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

	let branch_folders: Vec<String> = fs::read_dir(base_dir)?
		.filter_map(|entry| entry.ok())
		.filter_map(|entry| {
			let path = entry.path();
			if path.is_dir() {
				Some(path.to_string_lossy().to_string())
			} else {
				None
}})
		.collect();

    // Call the process_input_file function
    process_input_file(branch_folders, output_file_path)?;
    let duration = start_time.elapsed();
    println!("Time elapsed: {}\nPhew! I am done.", duration.as_secs_f64());
    Ok(())
  }
