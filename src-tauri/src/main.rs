// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod commands;
use commands::render::{
  render_home,
  get_header_logo
};

#[tauri::command]
fn greet(name: &str) -> String {
    
    format!("Hello, {}! You've been greeted by Mud!", name)
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            render_home,
            get_header_logo,
            greet
        ])
        .run(tauri::generate_context!())
        .expect("error while running mud application");
}
