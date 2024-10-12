mod lib;
use lib::process_input_file;
use std::fs::OpenOptions;
use std::io::{self, Write};
use std::path::Path;
use std::time::Instant;

fn main() -> io::Result<()> {

    let start_time = Instant::now();

    let duration = start_time.elapsed();
    println!("Time elapsed: {}\nPhew! I am done.", duration.as_secs_f64());
    Ok(())
}
