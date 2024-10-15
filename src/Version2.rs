mod lib;
use lib::process_input_file;
use log::{error, info};
use simplelog::*;
use std::fs::{self, OpenOptions, File};
use std::io::{self, Write};
use std::path::Path;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Instant;

// This function writes to the summary file with thread safety using a Mutex
fn write_to_summary_file(output_file: Arc<Mutex<File>>, data: &str) -> io::Result<()> {
    let mut output_file = output_file.lock().unwrap();
    writeln!(output_file, "{}", data)?;
    Ok(())
}

fn main() -> io::Result<()> {
    // Initialize the logger
    CombinedLogger::init(vec![
        WriteLogger::new(
            LevelFilter::Info,
            Config::default(),
            File::create("../log.txt")?,
        ),
    ]).unwrap();

    let start_time = Instant::now();

    info!("Program started.");

    let base_dir = "../data";
    let output_file_path = "../data/weekly_summary/weekly_sales_summary_version2.txt";
    let output_file = Path::new(output_file_path);

    if output_file.exists() {
        OpenOptions::new()
            .write(true)
            .truncate(true)
            .open(output_file_path)?;
        info!("Output file truncated.");
    }

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

    info!("Found {} branch folders.", branch_folders.len());

    let (tx, rx) = mpsc::channel();
    let mut handles = vec![];

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

        let output_file_thread = Arc::clone(&output_file);

        let handle = thread::spawn(move || {
            for folder in &folders_for_thread {
                match process_input_file(vec![folder.clone()], "../data/weekly_summary/weekly_sales_summary_version2.txt") {
                    Ok(_) => info!("Thread {} processed folder: {}", i, folder),
                    Err(e) => error!("Error processing folder {}: {}", folder, e),
                }
            }

            write_to_summary_file(output_file_thread, &format!("Thread {} finished", i))
                .expect("Failed to write to output file");
            tx.send("Thread finished".to_string()).expect("Failed to send message");
        });

        handles.push(handle);
    }

    drop(tx);

    let mut results = vec![];

    for _ in rx {
        results.push("Thread completed".to_string());
    }

    for handle in handles {
        handle.join().expect("Failed to join thread");
    }

    write_to_summary_file(Arc::clone(&output_file), "All threads completed")?;
    info!("All threads completed.");

    let duration = start_time.elapsed();
    println!("Time elapsed: {}\nPhew! I am done.", duration.as_secs_f64());
    info!("Program finished successfully. Time elapsed: {:.2} seconds", duration.as_secs_f64());

    Ok(())
}

