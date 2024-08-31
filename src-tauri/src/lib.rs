use duckscript::runner;
use duckscript::types::runtime::Context;
use duckscript::types::env::Env;
use duckscriptsdk;
use std::io::{Cursor, Write};
use std::sync::{Arc, Mutex};

// Helper function to set up context with arguments
fn setup_context_with_args(context: &mut Context, args: Vec<String>) {
    for (index, arg) in args.iter().enumerate() {
        context.variables.insert(format!("arg{}", index), arg.clone());
    }
}

// Command to run a Duckscript file with arguments
#[tauri::command]
fn run_duckscript(file_path: String, args: Vec<String>) -> Result<String, String> {
    // Initialize Duckscript context
    let mut context = Context::new();
    let _ = duckscriptsdk::load(&mut context.commands);
    setup_context_with_args(&mut context, args);

    // Run the Duckscript file
    match runner::run_script_file(&file_path, context, None) {
        Ok(ctx) => Ok(format!("{:?}", ctx.variables)),
        Err(error) => Err(format!("Error running script: {:?}", error)),
    }
}

// Struct to manage output capturing
struct OutputCapture {
    stdout_buf: Arc<Mutex<Cursor<Vec<u8>>>>,
    stderr_buf: Arc<Mutex<Cursor<Vec<u8>>>>,
}

impl OutputCapture {
    fn new() -> Self {
        Self {
            stdout_buf: Arc::new(Mutex::new(Cursor::new(Vec::new()))),
            stderr_buf: Arc::new(Mutex::new(Cursor::new(Vec::new()))),
        }
    }

    fn as_env(&self) -> Env {
        Env::new(
            Some(Box::new(ArcWriter(self.stdout_buf.clone())) as Box<dyn Write>),
            Some(Box::new(ArcWriter(self.stderr_buf.clone())) as Box<dyn Write>),
        )
    }

    fn get_stdout(&self) -> String {
        let stdout_buf = self.stdout_buf.lock().unwrap();
        String::from_utf8(stdout_buf.get_ref().clone()).unwrap_or_default()
    }

    fn get_stderr(&self) -> String {
        let stderr_buf = self.stderr_buf.lock().unwrap();
        String::from_utf8(stderr_buf.get_ref().clone()).unwrap_or_default()
    }
}

// Wrapper for Arc<Mutex<Cursor<Vec<u8>>>> that implements Write
struct ArcWriter(Arc<Mutex<Cursor<Vec<u8>>>>);

impl Write for ArcWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut cursor = self.0.lock().unwrap();
        cursor.write(buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        let mut cursor = self.0.lock().unwrap();
        cursor.flush()
    }
}

// Command to run a Duckscript script from a string
#[tauri::command]
fn run_script(script_content: String) -> Result<String, String> {
    // Initialize Duckscript context
    let mut context = Context::new();
    let _ = duckscriptsdk::load(&mut context.commands);

    // Create an OutputCapture instance to handle stdout and stderr
    let output_capture = OutputCapture::new();
    let env = output_capture.as_env();

    // Run the script with the specified environment
    match runner::run_script(&script_content, context, Some(env)) {
        Ok(_) => {
            let output = output_capture.get_stdout();
            Ok(output)
        }
        Err(error) => {
            let error_output = output_capture.get_stderr();
            Err(format!("Error running script: {:?}\n{}", error, error_output))
        }
    }
}

// Command to get synchronization status
#[tauri::command]
fn get_sync_status() -> Result<bool, ()> {
    Ok(true)
}

// Main function to run the Tauri application
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_sync_status,
            run_duckscript,
            run_script
        ])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
