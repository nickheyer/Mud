use duckscript::runner;
use duckscript::types::runtime::Context;
use duckscriptsdk;
use crate::output::OutputCapture;
use crate::context::setup_context_with_args;

#[tauri::command]
pub fn run_script(script_content: String) -> Result<String, String> {
    // Init ds
    let mut context = Context::new();
    let _ = duckscriptsdk::load(&mut context.commands);

    // Create an OutputCapture instance for stdio
    let output_capture = OutputCapture::new();
    let env = output_capture.as_env();

    // Run the script w/ env
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
