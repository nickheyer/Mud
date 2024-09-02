use duckscript::runner;
use duckscript::types::runtime::Context;
use duckscriptsdk;
use crate::output::OutputCapture;
use crate::context::setup_context_with_args;
use crate::utils::handle_script_error;
use std::collections::HashMap;

use serde::Serialize;

#[derive(Serialize)]
pub struct ScriptResponse {
    stdout: String,
    stderr: String,
    variables: HashMap<String, String>
}

#[tauri::command]
pub fn run_script(script_content: String) -> Result<String, String> {
    
    println!("{:?}", script_content);
    // Init ds
    let mut context = Context::new();
    let _ = duckscriptsdk::load(&mut context.commands);

    // Create an OutputCapture instance for stdio
    let output_capture = OutputCapture::new();
    let env = output_capture.as_env();

    // Run the script w/ env
    match runner::run_script(&script_content, context, Some(env)) {
        Ok(ctx) => {
            let stdout = output_capture.get_stdout();
            let stderr = output_capture.get_stderr();
            let variables = ctx.variables;

            // Parse successful execution results
            let script_resp = ScriptResponse { stdout, stderr, variables };
            let json_resp = serde_json::to_string(&script_resp)
                // Return default on failure to serialize
                .unwrap_or_else(|_| "{\"message\": \"Failed to serialize response\"".to_string());
            println!("Results of script serialized:\n{:#?}", json_resp);
            Ok(json_resp)
        }
        Err(err) => {
            let stdout = output_capture.get_stdout();
            let stderr = output_capture.get_stderr();
            println!("ERROR IN MATCH: {:#?}\nERROR IN STDERR: {:#?}", err, stderr);

            // Parse error to json serializable           
            let json_error = handle_script_error(err, stderr, stdout);
            Err(json_error)
        }
    }
}


#[tauri::command]
pub fn run_scriptfile(file_path: String, args: Vec<String>) -> Result<String, String> {
     // Init ds
    let mut context = Context::new();
    let _ = duckscriptsdk::load(&mut context.commands);
    setup_context_with_args(&mut context, args);

    // Run the script file, log variable output
    match runner::run_script_file(&file_path, context, None) {
        Ok(ctx) => Ok(format!("{:?}", ctx.variables)),
        Err(error) => Err(format!("Error running script: {:?}", error)),
    }
}

#[tauri::command]
pub fn get_all_commands() -> Vec<std::string::String> {
    let mut context = Context::new();
    let _ = duckscriptsdk::load(&mut context.commands);
    context.commands.get_all_command_names()
}
