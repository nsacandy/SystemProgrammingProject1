
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
//use csv::ReaderBuilder;
//use std::fs::{self, File, OpenOptions};
//use std::io::{self, Write};
//use std::path::Path;
//use std::sync::mpsc::Sender;
//use log::{info, error}; // Import log macros
//
//pub fn process_input_file(folder_list: Vec<String>, tx: Sender<String>) -> io::Result<&'static str> {
//    // Iterate through each folder in the list
//    for folder_path in folder_list {
//        // Log the folder being processed
//        info!("Processing folder: {}", folder_path);
//        
//        // Specify the file name
//        let file_path = Path::new(&folder_path).join("branch_weekly_sales.txt");
//
//        // Check if the file exists
//        if file_path.exists() && file_path.is_file() {
//            let file = File::open(&file_path)?;
//
//            // Create the CSV reader (without headers) from the file
//            let mut rdr = ReaderBuilder::new().has_headers(false).from_reader(file);
//
//            // Create total weekly sales variable to sum up all sales
//            let mut total_weekly_sales = 0;
//            let mut branch_code = String::new();
//            let mut product_code = String::new();
//
//            // Iterate over each record
//            for (index, result) in rdr.records().enumerate() {
//                let record = result?;
//                if index == 0 {
//                    branch_code = record[0].trim().to_string(); // First column as branch code
//                    product_code = record[1].trim().to_string(); // Second column as product code
//                }
//                let sales_string = &record[2].trim();
//
//                // Convert String to i32 and handle parsing errors
//                match sales_string.parse::<i32>() {
//                    Ok(total_sales) => {
//                        total_weekly_sales += total_sales;
//                    }
//                    Err(e) => {
//                        error!("Failed to parse sales data for {}: {}", folder_path, e);
//                        continue; // Skip this record if parsing fails
//                    }
//                }
//            }
//
//            // Send the result to the main thread through the channel
//            let message = format!("{}, {}, {}", branch_code, product_code, total_weekly_sales);
//            tx.send(message).expect("Failed to send message through channel");
//        } else {
//            error!("File not found: {:?}", file_path);
//        }
//    }
//
//    Ok("OK") // Return "OK" upon successful processing
//}
//
//// Initialize the logging configuration
//pub fn init_logging() {
//    // Initialize the logger with a simple configuration
//    env_logger::init();
//}
//
//fn write_to_summary_file(
//    output_file_path: &str,
//    branch_name: &str,
//    total_sales: i32,
//) -> io::Result<()> {
//    let output_dir = Path::new(output_file_path).parent().unwrap();
//    if !output_dir.exists() {
//        fs::create_dir_all(output_dir)?;
//    }
//    // Open the output file with the option to append data to it
//    let mut output_file = OpenOptions::new()
//        .write(true)
//        .create(true)
//        .append(true)
//        .open(output_file_path)?;
//
//    // Write the branch name and total sales to the output file
//    writeln!(output_file, "{}, {}", branch_name, total_sales)?;
//
//    Ok(())
//}
//
//// Function to write the duration to the output file
//fn write_duration_to_file(output_file_path: &str, duration: std::time::Duration) -> io::Result<()> {
//    // Open the output file with the option to append data to it
//    let mut output_file = OpenOptions::new()
//        .write(true)
//        .create(true)
//        .append(true)
//        .open(output_file_path)?;
//
//    // Write the duration to the output file
//    writeln!(
//        output_file,
//        "\nTotal execution time: {:.2?} seconds",
//        duration.as_secs_f64()
//    )?;
//
//    Ok(())
//}
