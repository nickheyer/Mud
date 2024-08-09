// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs::{self, File};
use std::io::{ErrorKind, Error, Write};
use dirs::home_dir;

fn create_mud_directory_and_file(name: &str) -> Result<String, Error> {
    let home_dir = home_dir().expect("Failed to get home directory");
    let mud_path = home_dir.join(".mud");

    if !mud_path.exists() {
        println!("Attempting to create dir: {:?}", mud_path);
        fs::create_dir(&mud_path)
            .map_err(|e| Error::new(ErrorKind::Other, format!("Failed to create .mud directory: {}", e)))?;
    }

    let mud_test_file_path = mud_path.join("mudTest.txt");

    if !mud_test_file_path.exists() {
        let mut mud_buf = File::create(&mud_test_file_path)
            .map_err(|e| Error::new(ErrorKind::Other, format!("Failed to create mudTest file: {}", e)))?;
        write!(mud_buf, "{}, welcome to the magical world of MUD!", name)?;
    }

    Ok(mud_test_file_path
        .into_os_string()
        .into_string()
        .unwrap()
      )
}

#[tauri::command]
fn create_mud(name: &str) -> String {
    match create_mud_directory_and_file(&name) {
        Ok(fp) => {
          let msg = format!("MUD directory and file created successfully: {:?}", fp);
          println!("Results of command: {}", &msg);
          msg
        },
        Err(e) => format!("Failed to create MUD directory and file: {}", e),
    }
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted by Mud!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
          greet,
          create_mud
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
