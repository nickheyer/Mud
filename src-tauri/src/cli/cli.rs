use crate::invocable::runner;
use tauri_plugin_cli::Matches;
use std::process;
use std::sync::Mutex;
use serde_json::Value;

pub struct ScriptState(pub Mutex<Option<String>>);

#[tauri::command]
pub fn get_cli_script(state: tauri::State<ScriptState>) -> Option<String> {
    state.0.lock().unwrap().take()
}

pub async fn handle_cli_execution(app: tauri::AppHandle, matches: Matches) -> Option<String> {
    let fallback = "echo \"NO VALID MUD PROVIDED\"".to_string();
    let script_content = match (&matches.args.get("file"), &matches.args.get("code")) {
        (Some(file_arg), _) if file_arg.value.is_string() => {
            std::fs::read_to_string(file_arg.value.as_str().unwrap()).unwrap_or_else(|e| {
                format!("echo \"FAILED TO READ FILE: {}\"", e)
            })
        },
        (_, Some(code_arg)) if code_arg.value.is_string() => {
            code_arg.value.as_str().unwrap().to_string()
        },
        _ => fallback,
    };

    if matches.args.get("display").and_then(|d| d.value.as_bool()).unwrap_or(false) {
        // Return the script content in display mode
        return Some(script_content);
    }

    // Execute the script
    handle_script_exec(app, script_content).await
}

async fn handle_script_exec(app: tauri::AppHandle, script_content: String) -> Option<String> {
    match runner::exec_script(app, script_content, None).await {
        Ok(_output) => {
            process::exit(0);
        },
        Err(e) => {
            if let Ok(parsed) = serde_json::from_str::<Value>(&e) {
                let message = parsed.get("message").and_then(|v| v.as_str()).unwrap_or("Unknown error");
                let line = parsed.get("line").and_then(|v| v.as_i64()).unwrap_or(-1);
                if line != -1 {
                    eprintln!("Traceback (most recent call last):\n  Line {}: {}", line, message);
                } else {
                    eprintln!("Error: {}", message);
                }
            }
            process::exit(1);
        }
    }
}
