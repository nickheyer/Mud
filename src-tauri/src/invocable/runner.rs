use duckscript::runner;
use duckscript::types::runtime::Context;
use duckscriptsdk;

use crate::context::setup_context_with_args;
use crate::output::OutputCapture;
use crate::utils::handle_script_error;
use serde::Serialize;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::{self, task};

use tauri::{ipc::Channel, AppHandle, Listener};

#[derive(Serialize)]
pub struct ScriptResponse {
    stdout: String,
    stderr: String,
    variables: HashMap<String, String>,
}

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase", tag = "event", content = "data")]
pub enum PayloadEvent {
    Stdout { message: String },
    _Stderr { message: String },
    _Started { message: String },
    _Finished { message: String },
}

#[tauri::command]
pub async fn run_script(
    handle: AppHandle,
    script_content: String,
    on_event: Channel<PayloadEvent>,
) -> Result<String, String> {
    println!("{:?}", script_content);

    // Atomic booleans for quiting tasks prematurely
    let halt_flag = Arc::new(AtomicBool::new(false));
    let task_halt_token = halt_flag.clone();
    let stdout_halt_token = halt_flag.clone();
    let stderr_halt_token = halt_flag.clone();

    handle.once_any("page-nav", move |_| {
        println!("Killing tasks due to REPL exit / page navigation.");
        task_halt_token.store(true, Ordering::SeqCst);
    });

    // Channels
    let (stdout_tx, mut stdout_rx) = mpsc::channel(10);
    let (stderr_tx, mut stderr_rx) = mpsc::channel(10);

    // Tasks
    let script_task: task::JoinHandle<Result<String, String>> = task::spawn_blocking(move || {
        // Env
        let output_capture = OutputCapture::new(stdout_tx, stderr_tx, Some(halt_flag));
        let env = output_capture.as_env();
        let mut context = Context::new();
        duckscriptsdk::load(&mut context.commands).unwrap();

        match runner::run_script(&script_content, context, Some(env)) {
            Ok(ctx) => {
                let response = ScriptResponse {
                    stdout: output_capture.get_stdout(),
                    stderr: output_capture.get_stderr(),
                    variables: ctx.variables,
                };
                let json = serde_json::to_string(&response).unwrap_or_else(|_| {
                    "{\"message\": \"Failed to serialize response\"".to_string()
                });
                Ok(json)
            }
            Err(err) => {
                let stdout = output_capture.get_stdout();
                let stderr = output_capture.get_stderr();
                let json = handle_script_error(err, stderr, stdout);
                Err(json)
            }
        }
    });

    let stdout_task: task::JoinHandle<Result<(), String>> = task::spawn(async move {
        while let Some(line) = stdout_rx.recv().await {
            if stdout_halt_token.load(Ordering::SeqCst) {
                return Err("Stdout was cancelled.".to_string());
            }

            println!("STDOUT: {:#?}", line);
            if let Err(err) = on_event.send(PayloadEvent::Stdout { message: line }) {
                return Err(format!("Failed to send event: {:?}", err));
            }
        }

        Ok(())
    });

    let stderr_task: task::JoinHandle<Result<(), String>> = task::spawn(async move {
        while let Some(line) = stderr_rx.recv().await {
            if stderr_halt_token.load(Ordering::SeqCst) {
                return Err("Stderr was cancelled.".to_string());
            }
            println!("STDERR: {:#?}", line);
        }

        Ok(())
    });

    // Waiting For Tasks
    match tokio::try_join!(script_task, stdout_task, stderr_task) {
        Ok((res, _, _)) => res,
        Err(err) => {
            println!("Error in async task runtime: {:?}", err);
            Err(err.to_string())
        }
    }
}

#[tauri::command]
pub fn run_scriptfile(file_path: String, args: Vec<String>) -> Result<String, String> {
    let mut context = Context::new();
    duckscriptsdk::load(&mut context.commands).unwrap();
    setup_context_with_args(&mut context, args);

    runner::run_script_file(&file_path, context, None)
        .map(|ctx| format!("{:?}", ctx.variables))
        .map_err(|error| format!("Error running script: {:?}", error))
}

#[tauri::command]
pub fn get_all_commands() -> Vec<String> {
    let mut context = Context::new();
    duckscriptsdk::load(&mut context.commands).unwrap();
    context.commands.get_all_command_names()
}
