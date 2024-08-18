use tauri;
use std::process::Command;
use std::io::{ErrorKind, Error};

#[tauri::command]
pub fn install_mod(mod_name: String) -> Result<String, String> {
    // Placeholder logic for installing a mod
    Ok(format!("Mod {} installed successfully!", mod_name))
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted by Mud!", name)
}

#[tauri::command]
fn install_mods(files: Vec<String>) -> String {
    let python_script = "/home/nick/code/Mud/scripts/test.py";

    for file in files {
        let output = Command::new("python3")
            .arg(python_script)
            .arg(&file)
            .output()
            .map_err(|e| Error::new(ErrorKind::Other, format!("Failed to run Python script: {}", e)))
            .unwrap();

        if !output.status.success() {
            return format!("Failed to install mod: {}", String::from_utf8_lossy(&output.stderr));
        }
    }

    "All mods installed successfully.".to_string()
}
