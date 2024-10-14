mod lib;
use lib::process_input_file;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
use std::path::Path;
use std::sync::mpsc;
use std::thread;
use std::time::Instant;

fn write_to_summary_file(output_file_path: &str, data: Vec<String>) -> io::Result<()> {
    let output_dir = Path::new(output_file_path).parent().unwrap();
    if !output_dir.exists() {
        fs::create_dir_all(output_dir)?;
    }

    let  mut output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(output_file_path)?;

    for entry in data {
        writeln!(output_file, "{}", entry)?;
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let start_time = Instant::now();

    let base_dir = "../data";
    let output_file_path = "../data/weekly_summary/weekly_sales_summary_version2.txt";
    
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

        let output_file_path_thread = output_file_path.to_string();
        let handle = thread::spawn(move || {
            // Reusing the process_input_file function from lib.rs
            process_input_file(folders_for_thread, &output_file_path_thread).expect("Thread processing failed");
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

    // Write the final output if needed (results)
    write_to_summary_file(output_file_path, results)?;

    let duration = start_time.elapsed();
    println!("Multithreaded Time elapsed: {:.2?} seconds", duration);

    // Return success
    Ok(())
}
