use std::fs;
use std::io;

fn main() -> io::Result<()> {
    // Specify the file path
    let file_path = "../data/data/CTONGA/branch_weekly_sales.txt";
    
    // Read the file content into a String
    let contents = fs::read_to_string(file_path)?;

    // Print the file content
    println!("File Contents:\n{}", contents);
    
    Ok(())
}
