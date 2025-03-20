use std::fs::File;
use std::io::{Write, BufWriter, BufReader, BufRead};
use rust_xlsxwriter::*;
use std::thread;
use std::sync::{Arc, Mutex};
use std::io::Error;

pub fn csv_to_excel(file_path: String, file_content: String) -> Result<(), Error> {
    let mut workbook = Workbook::new();
    let mut worksheet = workbook.add_worksheet();

    // Split the file content into 4 chunks for multi-threading
    let num_threads = 4;
    let chunk_size = file_content.lines().count() / num_threads;
    let chunks: Vec<Vec<String>> = file_content
        .lines()
        .collect::<Vec<&str>>()
        .chunks(chunk_size)
        .map(|chunk| chunk.iter().map(|&s| s.to_string()).collect())
        .collect();

    // Vector to store results from each thread
    let results = Arc::new(Mutex::new(Vec::new()));

    let mut handles = vec![];

    // Spawn threads to process data
    for chunk in chunks.into_iter() {
        let results = Arc::clone(&results);

        let handle = thread::spawn(move || {
            let mut local_rows = Vec::new();

            // Process each line in the chunk
            for line in chunk.iter() {
                let values: Vec<String> = line.split(',').map(|s| s.to_string()).collect();
                local_rows.push(values);
            }


            // Lock results vector and push the local rows
            let mut results = results.lock().unwrap();
            results.push(local_rows);
        });

        handles.push(handle);
    }

    // Wait for all threads to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Now that all threads are done, write the data to the worksheet
    let results = results.lock().unwrap();
    let mut row_idx = 0;
    for chunk in results.iter() {
        for row in chunk.iter() {
            for (col_idx, value) in row.iter().enumerate() {
                worksheet.write_string(row_idx as u32, col_idx as u16, value)
                    .map_err(|e| Error::new(std::io::ErrorKind::Other, format!("Excel Write Error: {}", e)))?;
            }
            println!("row_idx: {}", row_idx);
            row_idx += 1;
        }
    }

    println!("Excel file saved to: {}", &file_path);

    // Save the Excel file
    workbook.save(&file_path)
        .map_err(|e| Error::new(std::io::ErrorKind::Other, format!("Excel Save Error: {}", e)))?;
    Ok(())
}

// Function to save a CSV file
fn save_file(file_path: String, content: String) -> Result<(), Error> {
    let file = File::create(file_path)?;
    let mut writer = BufWriter::new(file);
    writer.write_all(content.as_bytes())?;
    Ok(())
}

// Main function
fn main() {
    println!("Hello, world!");
    
    // Generate CSV content
    let mut csv_content = String::new();
    for i in 0..10000u64 {
        csv_content.push_str(&format!("{},{}\n", i, i * i));
    }

    let csv_path = "test.csv".to_string();
    let excel_path = "test.xlsx".to_string();

    // Save the CSV file
    save_file(csv_path.clone(), csv_content.clone()).unwrap();
    
    // Convert CSV to Excel with multi-threading
    csv_to_excel(excel_path, csv_content).unwrap();
}