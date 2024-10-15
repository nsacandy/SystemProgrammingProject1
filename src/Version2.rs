mod lib;
use lib::process_input_file;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::Path;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Instant;

// This function writes to the summary file with thread safety using a Mutex
fn write_to_summary_file(output_file: Arc<Mutex<fs::File>>, data: &str) -> io::Result<()> {
    
    let mut output_file = output_file.lock().unwrap();
    writeln!(output_file, "{}", data)?;
    Ok(())
}

fn main() -> io::Result<()> {
    let start_time = Instant::now();

    let base_dir = "../data";
    let output_file_path = "../data/weekly_summary/weekly_sales_summary_version2.txt";
    let output_file = Path::new(output_file_path);
    
    // Clear the file if it exists
    if output_file.exists() {
        OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(output_file_path)?;
    }

    // Open the output file and wrap it in an Arc<Mutex<_>> for safe shared access
    let output_file = Arc::new(Mutex::new(
        OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(output_file_path)?
    ));

    let branch_folders: Vec<String> = fs::read_dir(base_dir)?
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            let path = entry.path();
            if path.is_dir() {
                Some(path.to_string_lossy().to_string())
            } else {
                None
            }
        })
        .collect();

    let (tx, rx) = mpsc::channel();
    let mut handles = vec![];

    // Create 4 threads and distribute the folders using a modulo operation
    let num_threads = 4;
    for i in 0..num_threads {
        let tx = tx.clone();
        let folders_for_thread: Vec<String> = branch_folders
            .iter()
            .enumerate()
            .filter_map(|(index, folder)| {
                if index % num_threads == i {
                    Some(folder.clone())
                } else {
                    None
                }
            })
            .collect();

        // Clone the Arc<Mutex<_>> for the thread
        let output_file_thread = Arc::clone(&output_file);

        let handle = thread::spawn(move || {
            // Reusing the process_input_file function from lib.rs
            process_input_file(folders_for_thread, "../data/weekly_summary/weekly_sales_summary_version2.txt").expect("Thread processing failed");

            // Writing a message to the output file in a thread-safe way
            write_to_summary_file(output_file_thread, &format!("Thread {} finished", i))
                .expect("Failed to write to output file");

            tx.send("Thread finished".to_string()).expect("Failed to send message");
        });

        handles.push(handle);
    }

    drop(tx); // Close the sending side so `rx` knows when the threads are done

    let mut results = vec![];

    // Collect results from all threads
    for _ in rx {
        results.push("Thread completed".to_string());
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().expect("Failed to join thread");
    }

    // Optionally write the final summary result
    write_to_summary_file(Arc::clone(&output_file), "All threads completed")?;

    let duration = start_time.elapsed();
    println!("Time elapsed: {}\nPhew! I am done.", duration.as_secs_f64());

    // Return success
    Ok(())
}
