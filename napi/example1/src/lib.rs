#![deny(clippy::all)]
use napi::bindgen_prelude::*;
use napi_derive::napi;
use std::fs::File;
use std::io::{Write, BufWriter,BufReader,BufRead};
use rust_xlsxwriter::*;

#[macro_use]
extern crate napi_derive;

#[napi]
pub fn sum(a: i32, b: i32) -> i32 {
  a + b
}

#[napi]
pub fn save_file(file_path: String, content: String) -> Result<()> {
  let file = File::create(&file_path).unwrap();
  let mut file = BufWriter::new(file);
  file.write_all(content.as_bytes()).unwrap();
  println!("File saved to: {}", file_path);
  Ok(())
}

#[napi]
pub fn csv_to_excel(file_path: String, file_content: String) -> Result<()> {
    let mut workbook = Workbook::new(); // Create a new Excel workbook
    let mut worksheet = workbook.add_worksheet(); // Add a worksheet

    // Read CSV data from `file_content`
    let cursor = std::io::Cursor::new(file_content);
    let reader = BufReader::new(cursor);

    for (row_idx, line) in reader.lines().enumerate() {
        let line = line.map_err(|e| Error::new(Status::GenericFailure, format!("CSV Read Error: {}", e)))?;
        let values: Vec<String> = line.split(',').map(|s| s.to_string()).collect();

        for (col_idx, value) in values.iter().enumerate() {
            worksheet.write_string(row_idx as u32, col_idx as u16, value)
                .map_err(|e| Error::new(Status::GenericFailure, format!("Excel Write Error: {}", e)))?;
              println!("row_idx: {}, col_idx: {}, value: {}", row_idx, col_idx, value);
        }
    }

    println!("Excel file saved to: {}", &file_path);

    // Save Excel file
    workbook.save(file_path)
        .map_err(|e| Error::new(Status::GenericFailure, format!("Excel Save Error: {}", e)))?;
    Ok(())
}