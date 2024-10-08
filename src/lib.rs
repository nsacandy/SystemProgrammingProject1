use std::fs::{self, File};
use std::io;
use cs::ReaderBuilder;
use std::path::Path;

pub fn process_input_file(base_dir: &str) -> io::Result<()> {
    //Iterate through each branch folder
    for branch_entry in fs::read_dir(base_dir)? {
        let branch_entry = branch_entry?;
        let branch_path = branch_entry.path();

        //Make sure its a  directory
        if branch_path.is_dir() {

            //Specify the file name
            let file_path  = branch_path.join("branch_weekly_sales.txt");

            //Check if the file exists
            if file_path.exists() && file_path.is_file() {
                let file = File::open(file_path)?;

                // Create the CSV reader (without headers) from the file
                let mut rdr = ReaderBuilder::new().has_headers(false).from_reader(file);

                // Create total weekly sales variable to sum up all sales
                let mut total_weekly_sales = 0;

                //Iterate over each record
                for result in rdr.records() {
                    let record = result?;
                    let branch_name  = &record[0].trim();
                    let sales_string = &record[2].trim();

                    //Convert String to i32
                    let total_sales = sales_string.parse::<i32>().unwrap();
                    total_weekly_sales += total_sales;
                }

            }
        }
    }
}