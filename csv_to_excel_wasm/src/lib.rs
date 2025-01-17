#![deny(clippy::all)]

use napi::bindgen_prelude::*;
use napi_derive::napi;  // ✅ Import the `napi` macro
use std::fs::File;
use std::io::Write;

#[napi]  // ✅ This macro is now recognized
pub fn write_to_file(file_path: String, content: String) -> Result<()> {
    let mut file = File::create(file_path).map_err(|e| Error::new(Status::GenericFailure, format!("Failed to create file: {}", e)))?;
    file.write_all(content.as_bytes())
        .map_err(|e| Error::new(Status::GenericFailure, format!("Failed to write to file: {}", e)))?;
    Ok(())
}