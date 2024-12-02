use serde_json::Value;
use std::error::Error;
use std::fs;
use std::path::PathBuf;

// Loads JSON file into a `serde_json::Value` struct.
pub fn load_json(json_path: PathBuf) -> Result<Value, Box<dyn Error>> {
    let json_data = fs::read_to_string(json_path)?;
    let parsed: Value = serde_json::from_str(&json_data)?;
    Ok(parsed)
}
