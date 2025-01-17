use wasm_bindgen::prelude::*; // This imports the wasm_bindgen attribute and other necessary items
use rust_xlsxwriter::*;
use std::path::PathBuf;
use std::io::{Cursor, Write};
use csv::ReaderBuilder;
use napi::bindgen_prelude::*;
use napi_derive::napi;
use tokio::fs::read;

// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_namespace = console)]
//     fn log(msg: &str);
// }

// #[wasm_bindgen]
// pub fn convert_csv_to_excel(file_path: &str, csv_data: &str) -> Result<(), JsValue> {
// 	log(&format!("Converting CSV data to Excel: {}", file_path));
//     // Create a new Excel workbook
//     let mut workbook = Workbook::new();
//     let  worksheet = workbook.add_worksheet();

//     // Create a CSV reader from the provided CSV data
//     let mut rdr = ReaderBuilder::new()
//         .has_headers(false) // Adjust if CSV has headers
//         .from_reader(Cursor::new(csv_data));

//     // Iterate through the CSV records and add them to the worksheet
//     for (row_index, result) in rdr.records().enumerate() {
//         let record = result.map_err(|e| JsValue::from_str(&e.to_string()))?;

//         // Write each field in the record to the corresponding cell
//         for (col_index, field) in record.iter().enumerate() {
//             worksheet.write_string(row_index as u32, col_index as u16, field)
//                 .map_err(|e| JsValue::from_str(&e.to_string()))?;
//         }
//     }
// 	let file = File::create(file_path).map_err(|e| JsValue::from_str(&e.to_string()))?;
// 	let buf = workbook.save_to_writer(file).map_err(|e| JsValue::from_str(&e.to_string()))?;

// 	Ok(())
// }

// #[wasm_bindgen]
// pub fn save_csv_to_file(file_path: &str, csv_data: &str) -> Result<(), JsValue> {
// 	log(&format!("Saving CSV data to file: {}", file_path));
//     // Try to create and write to the file
//     match File::create(file_path) {
//         Ok(mut file) => {
//             // Write the CSV data (String) to the file
//             if let Err(e) = file.write_all(csv_data.as_bytes()) {
//                 // If writing fails, return an error as a JsValue
//                 return Err(JsValue::from_str(&e.to_string()));
//             }

//             // Ensure data is written by flushing the file
//             if let Err(e) = file.flush() {
//                 return Err(JsValue::from_str(&e.to_string()));
//             }
//         }
//         Err(e) => {
//             // If file creation fails, return an error as a JsValue
//             return Err(JsValue::from_str(&e.to_string()));
//         }
//     }

//     Ok(())
// }


/// module registration is done by the runtime, no need to explicitly do it now.
#[napi]
fn fibonacci(n: u32) -> u32 {
  match n {
    1 | 2 => 1,
    _ => fibonacci(n - 1) + fibonacci(n - 2),
  }
}

/// use `Fn`, `FnMut` or `FnOnce` traits to defined JavaScript callbacks
/// the return type of callbacks can only be `Result`.
// #[napi]
// fn get_cwd<T: Fn(String) -> Result<()>>(callback: T) {
//   callback(env::current_dir().unwrap().to_string_lossy().to_string()).unwrap();
// }

/// or, define the callback signature in where clause
#[napi]
fn test_callback<T>(callback: T)
where T: Fn(String) -> Result<()>
{}

#[napi]
async fn read_file_async(path: String) -> Result<Buffer> {
  read(path)
    .map(|r| match r {
      Ok(content) => Ok(content.into()),
      Err(e) => Err(Error::new(
        Status::GenericFailure,
        format!("failed to read file, {}", e),
      )),
    })
    .await
}
