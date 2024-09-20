use duckscript::runner;
use duckscript::types::runtime::Context;
use duckscriptsdk;
use crate::output::OutputCapture;
use crate::context::setup_context_with_args;
use crate::utils::handle_script_error;
use tokio::sync::mpsc;
use tokio::{self, task};
use std::collections::HashMap;
use serde::Serialize;
use tauri::{AppHandle, ipc::Channel};

#[derive(Serialize)]
pub struct ScriptResponse {
    stdout: String,
    stderr: String,
    variables: HashMap<String, String>
}


#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "event", content = "data")]
pub enum PayloadEvent {
  #[serde(rename_all = "camelCase")]
  Stdout {
    message: String
  },
  #[serde(rename_all = "camelCase")]
  _Stderr {
    message: String
  },
  #[serde(rename_all = "camelCase")]
  _Started {
    message: String
  },
  #[serde(rename_all = "camelCase")]
  _Finished {
    message: String
  },
}


#[tauri::command]
pub async fn run_script(
    _app: AppHandle,
    script_content: String,
    on_event: Channel<PayloadEvent>
) -> Result<String, String> {
    println!("{:?}", script_content);

    // Create async channels for stdout and stderr
    let (stdout_tx, mut stdout_rx) = mpsc::channel(10);
    let (stderr_tx, mut stderr_rx) = mpsc::channel(10);

    // Spawn a blocking task to run the script on a separate thread
    let script_task = task::spawn_blocking(move || {
        // Init duckscript
        let mut context = Context::new();
        let _ = duckscriptsdk::load(&mut context.commands);

        // Create an OutputCapture instance for stdio
        let output_capture = OutputCapture::new(stdout_tx, stderr_tx);
        let env = output_capture.as_env();

        // Run the script with env
        let result = match runner::run_script(&script_content, context, Some(env)) {
            Ok(ctx) => {
                let stdout = output_capture.get_stdout();
                let stderr = output_capture.get_stderr();
                let variables = ctx.variables;

                // Create ScriptResponse and serialize it to JSON
                let script_resp = ScriptResponse { stdout, stderr, variables };
                let json_resp = serde_json::to_string(&script_resp)
                    // Return default on failure to serialize
                    .unwrap_or_else(|_| "{\"message\": \"Failed to serialize response\"".to_string());
                
                // RETURNING RESULTS FROM THREAD HERE
                println!("Results of script:\n{:#?}", json_resp);
                Ok(json_resp)
            }
            Err(err) => {
                let stdout = output_capture.get_stdout();
                let stderr = output_capture.get_stderr();
                println!("ERROR IN MATCH: {:#?}\nERROR IN STDERR: {:#?}", err, stderr);

                // RETURNING ERROR FROM THREAD HERE        
                let json_error = handle_script_error(err, stderr, stdout);
                Err(json_error)
            }
        };

        result
    });

    // Spawn tasks to listen to the stdout and stderr channels
    let stdout_task = task::spawn(async move {
        while let Some(line) = stdout_rx.recv().await {
            println!("STDOUT: {}", line);
            on_event.send(PayloadEvent::Stdout { message: line }).unwrap();
        }
    });

    let stderr_task = task::spawn(async move {
        while let Some(line) = stderr_rx.recv().await {
            println!("STDERR: {}", line);
        }
    });

    // Wait for all tasks to finish
    match tokio::try_join!(
        script_task,
        stdout_task,
        stderr_task
    ) {

        Ok((res, _, _)) => res,

        Err(err) => {
            println!("ERROR IN ASYNC TASK RUNTIME: {:#?}", err);
            Err(err.to_string())
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
