use crate::invocable::runner;
use tauri_plugin_cli::Matches;
use std::process;
use std::sync::Mutex;

pub struct ScriptState(pub Mutex<Option<String>>);

#[tauri::command]
pub fn get_cli_script(state: tauri::State<ScriptState>) -> Option<String> {
    state.0.lock().unwrap().take()
}

pub async fn handle_cli_execution(app: tauri::AppHandle, matches: Matches) -> Option<String> {
    
    let fallback = "echo \"NO VALID SCRIPT CONTENT PROVIDED\"".to_string();
    let script_content = match (&matches.args.get("file"), &matches.args.get("str"), &matches.args.get("run")) {
        (Some(file_arg), _, _) if file_arg.value.is_string() => {
            std::fs::read_to_string(file_arg.value.as_str().unwrap())
                .map_err(|e| format!("echo \"FAILED TO READ SCRIPT FILE: {}\"", e))
                .unwrap()
        },
        (_, _, Some(run_arg)) if run_arg.value.is_string() => {
            std::fs::read_to_string(run_arg.value.as_str().unwrap())
                .map_err(|e| format!("echo \"FAILED TO READ SCRIPT FILE: {}\"", e))
                .unwrap()
        },
        (_, Some(str_arg), _) if str_arg.value.is_string() => {
            str_arg.value.as_str().unwrap().to_string()
        },
        _ =>  fallback
    };

    let quiet = matches.args.get("quiet")
        .and_then(|q| q.value.as_bool())
        .unwrap_or(false);

    if quiet {
        handle_script_exec(app, script_content).await
    } else {
        Some(script_content)
    }
}

async fn handle_script_exec(app: tauri::AppHandle, script_content: String) -> Option<String> {
    match runner::exec_script(app, script_content, None).await {
        Ok(output) => {
            println!("{:?}", output);
            process::exit(0);
        },
        Err(e) => {
            eprintln!("ERROR: {}", e);
            process::exit(1);
        }
    }
}
