
use duckscript::runner;
use duckscript::types::runtime::Context;
use duckscriptsdk;
use std::io::Read;
use gag::BufferRedirect;

fn setup_context_with_args(context: &mut Context, args: Vec<String>) {
    // Set up the arguments as variables in the context
    for (index, arg) in args.iter().enumerate() {
        context.variables.insert(format!("arg{}", index), arg.clone());
    }
}

#[tauri::command]
fn run_duckscript(file_path: String, args: Vec<String>) -> Result<String, String> {
    // Initialize DS
    let mut context = Context::new();
    let _ = duckscriptsdk::load(&mut context.commands);
    setup_context_with_args(&mut context, args);

    // Run the DuckScript script
    match runner::run_script_file(&file_path, context) {
        Ok(ctx) => Ok(format!("{:?}", ctx.variables)),
        Err(error) => Err(format!("Error running script: {:?}", error)),
    }
}

#[tauri::command]
fn run_script(script_content: String) -> Result<String, String> {
    // Initialize DS
    let mut context = Context::new();
    let _ = duckscriptsdk::load(&mut context.commands);
    //setup_context_with_args(&mut context, args);

    // Create "gag"... buffer to capture stdout. Run the DuckScript script
    let mut buf = BufferRedirect::stdout().unwrap();
    let mut output = String::new();
    match runner::run_script(&script_content, context) {
        Ok(_) => {
            buf.read_to_string(&mut output).unwrap();
            drop(buf); // Dropping buffer
            Ok(format!("{:?}", output))
        },
        Err(error) => {
            buf.read_to_string(&mut output).unwrap();
            drop(buf);
            Err(format!("Error running script: {:?}\nStdout: {:#?}", error, output))
        },
    }
}

#[tauri::command]
fn get_sync_status() -> Result<bool, ()> {
    Ok(true)
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_sync_status,
            run_duckscript,
            run_script
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
