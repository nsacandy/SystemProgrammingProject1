use std::io;
use csv::ReaderBuilder;
use std::fs::File;

fn main() -> io::Result<()> {
    // Specify the file path
    let file_path = "/Users/dillonemmons/SystemProgrammingProject1/data/data/CTONGA/branch_weekly_sales.txt";
    let file = File::open(file_path)?;
    // Read the file content into a String


    // Create the CSV reader (without headers) from the file
    let mut rdr = ReaderBuilder::new().has_headers(false).from_reader(file);

    // Iterate over each record
    for result in rdr.records() {
        let record = result?;

        // Print the fields of each record
        println!("{} {} {} {}", &record[0], &record[1], &record[2], &record[3]);
    }
    // Print the file content
    //println!("File Contents:\n{}", contents);
    
    Ok(())
}
