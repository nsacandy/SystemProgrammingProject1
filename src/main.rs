use std::fs::{self, File};
use std::io;
use std::io::Write;
use csv::ReaderBuilder;

fn main() -> io::Result<()> {
    //Select base directory
    let base_dir = "../data/data";
    let mut file = File::create("outputFile.txt")?;
    let fake_output = "Hello partners";
    file.write_all(&fake_output.as_bytes())?;

    //Iterate through each  branch folder
    for branch_entry in fs::read_dir(base_dir)? {
        let branch_entry = branch_entry?;
        let branch_path = branch_entry.path();

        //Make sure it's a directory
        if branch_path.is_dir() {
            //Specify the file name
            let file_path = branch_path.join("branch_weekly_sales.txt");

            //Check file exists
            if file_path.exists() && file_path.is_file() {
                let file = File::open(file_path)?;

                // Create the CSV reader (without headers) from the file
                let mut rdr = ReaderBuilder::new().has_headers(false).from_reader(file);

                //Create total weekly sales variable to sum up all sales
                let mut total_weekly_sales = 0;


                // Iterate over each record
                for result in rdr.records() {
                    let record = result?;
                    let sales_string = &record[2].trim();
        
                // Converting a String to an i32
                let total_sales = sales_string.parse::<i32>().unwrap();
                total_weekly_sales += total_sales;

                // Print the fields of each record, and the total sales for each day as an int
                println!("{} {} {} {}", &record[0], &record[1], &record[2], &record[3]);
                }
                
                //Print the total number of sales for the  week
                println!("There were {} sales this week", total_weekly_sales);
            }
        }
    }

    Ok(())
}
