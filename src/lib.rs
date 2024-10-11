use csv::ReaderBuilder;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::Path;

pub fn process_input_file(base_dir: &str, output_file_path: &str) -> io::Result<()> {

    //Iterate through each branch folder
    for branch_entry in fs::read_dir(base_dir)? {
        let branch_entry = branch_entry?;
        let branch_path = branch_entry.path();

        //Make sure its a  directory
        if branch_path.is_dir() {
            //Specify the file name
            let file_path = branch_path.join("branch_weekly_sales.txt");

            //Check if the file exists
            if file_path.exists() && file_path.is_file() {
                let file = File::open(file_path)?;

                // Create the CSV reader (without headers) from the file
                let mut rdr = ReaderBuilder::new().has_headers(false).from_reader(file);

                // Create total weekly sales variable to sum up all sales
                let mut total_weekly_sales = 0;
                let mut branch_name = String::new();

                //Iterate over each record
                for (index, result) in rdr.records().enumerate() {
                    let record = result?;
                    if index == 0 {
                        branch_name = record[0].trim().to_string();
                    }
                    let sales_string = &record[2].trim();

                    //Convert String to i32
                    let total_sales = sales_string.parse::<i32>().unwrap();
                    total_weekly_sales += total_sales;
                } 

                //Call write_to_summary_file to write branch's sales data
                write_to_summary_file(output_file_path, &branch_name, total_weekly_sales)?;
            } 
        }
    }

    Ok(())
}

fn write_to_summary_file(
    output_file_path: &str,
    branch_name: &str,
    total_sales: i32,
) -> io::Result<()> {
    let output_dir = Path::new(output_file_path).parent().unwrap();
    if !output_dir.exists() {
        fs::create_dir_all(output_dir)?;
    }
    //Open the output file with the option  to append data to it
    let mut output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(output_file_path)?;

    //Write the branch name and total sales to  the output file
    writeln!(output_file, "{}, {}", branch_name, total_sales)?;

    Ok(())
}

// Function to write the duration to the output file
fn write_duration_to_file(output_file_path: &str, duration: std::time::Duration) -> io::Result<()> {
    // Open the output file with the option to append data to it
    let mut output_file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(output_file_path)?;

    // Write the duration to the output file
    writeln!(
        output_file,
        "\nTotal execution time: {:.2?} seconds",
        duration.as_secs_f64()
    )?;

    Ok(())
}
