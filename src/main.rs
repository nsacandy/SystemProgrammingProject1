// mod lib;
// use lib::process_input_file;
// use std::fs::OpenOptions;
// use std::io::{self, Write};
// use std::path::Path;
// use std::time::Instant;

// fn main() -> io::Result<()> {

//     let start_time = Instant::now();

//     let duration = start_time.elapsed();
//     println!("Time elapsed: {}\nPhew! I am done.", duration.as_secs_f64());
//     Ok(())
// }
mod lib;
use lib::{process_input_file, write_to_summary_file}; // Import write_to_summary_file
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::mpsc;  // Import the mpsc module for channels
use std::thread;      // Import the thread module for creating threads
use std::time::Instant;

fn main() -> io::Result<()> {
    // Start the timer at the beginning of the main function
    let start_time = Instant::now();

    // Define the base directory containing the branch data
    let base_dir = "../data";

    // Define the output file path
    let output_file_path = "../data/weekly_summary/weekly_sales_summary.txt";

    // Ensure the output directory exists
    let output_dir = Path::new("../data/weekly_summary");
    if !output_dir.exists() {
        fs::create_dir_all(output_dir)?; // Create the directory if it doesn't exist
    }

    // If the output file exists, truncate it to clear its contents
    let output_file = Path::new(output_file_path);
    if output_file.exists() {
        OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(output_file_path)?;
    }

    // Create a channel for communication
    let (tx, rx) = mpsc::channel();

    // Collect all branch directories
    let branch_dirs: Vec<PathBuf> = fs::read_dir(base_dir)?
        .filter_map(|entry| {
            let entry = entry.ok()?;
            let path = entry.path();
            if path.is_dir() {
                Some(path)
            } else {
                None
            }
        })
        .collect();

    // Split branch directories into groups of 10
    let mut groups: Vec<Vec<PathBuf>> = Vec::new();
    for chunk in branch_dirs.chunks(10) {
        groups.push(chunk.to_vec());
    }

    // Create threads for each group (up to 4 groups)
    let mut handles = Vec::new(); // Store handles for each thread
    for group in groups.into_iter().take(4) { // Take up to 4 groups
        let tx = tx.clone();
        let output_file_path = output_file_path.to_string(); // Clone for thread ownership
        let base_dir = base_dir.to_string(); // Clone for thread ownership

        // Spawn a thread for each group
        let handle = thread::spawn(move || {
            // Call the process_input_file function for this group
            if let Err(e) = process_input_file(&base_dir, &output_file_path) {
                eprintln!("Error processing input file: {}", e);
            }

            // Optionally send a message through the channel if needed
            tx.send(format!("Processed group with {} folders", group.len()))
                .expect("Failed to send message");
        });

        handles.push(handle); // Store the handle
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().expect("Thread failed");
    }

    // Process messages from the channel
    while let Ok(message) = rx.recv() {
        // Print the received message
        println!("Received: {}", message);
        
        // Write the message to the output file using write_to_summary_file
        // Assuming the message format is "Branch Code, Product Code, Total Sales"
        // You'll need to parse this according to your actual message format.
        let parts: Vec<&str> = message.split(',').collect();
        if parts.len() == 3 {
            let branch_code = parts[0].trim();
            let product_code = parts[1].trim();
            let total_sales: i32 = parts[2].trim().parse().unwrap_or(0); // Default to 0 if parse fails
            write_to_summary_file(&output_file_path, branch_code, total_sales)?;
        }
    }

    // Stop the timer and measure the elapsed time after all processing is done
    let duration = start_time.elapsed();

    // Print the total time elapsed in the console
    println!("Total execution time: {:.6} seconds\nPhew! I am done.", duration.as_secs_f64());
    
    Ok(())
}
