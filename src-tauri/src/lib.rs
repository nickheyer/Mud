mod context;
mod invocable;
mod output;
mod utils;
mod cli;

use tauri_plugin_cli::CliExt;
use tokio::runtime::Runtime;
use std::sync::Mutex;
use tauri::Manager;


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let builder = tauri::Builder::default()
        .setup(|app| {
            // HANDLE CLI ARGUMENTS
            if let Ok(matches) = app.cli().matches() {
                match (&matches.args.get("help"), &matches.args.get("file"), &matches.args.get("code")) {
                    (Some(help_arg), _, _) if help_arg.value.as_bool().unwrap_or(false) => {
                        println!(
                            "Usage: mud [OPTIONS] [FILE]\n\n\
                            Options:\n\
                            \t-c, --code <CODE>\tInline code to execute instead of a file\n\
                            \t-d, --display\t\tFlag to enable GUI interpretation using Mud's Repl interface\n\
                            \t-h, --help\t\tShow this help message and exit\n\n\
                            Positional Arguments:\n\
                            \t<FILE>\t\t\tFile path to execute. Overrides if -c is used."
                        );
                        std::process::exit(0);
                    },
                    (_, Some(file_arg), Some(code_arg)) if file_arg.value.is_string() || code_arg.value.is_string() => {
                        let runtime = Runtime::new().unwrap();
                        if let Some(res) = runtime.block_on(cli::handle_cli_execution(app.handle().clone(), matches)) {
                            app.manage(cli::ScriptState(Mutex::new(Some(res))));
                        }
                    },
                    _ => { app.manage(cli::ScriptState(Mutex::new(None))); }
                };
            }

            Ok(())
        })
        .plugin(tauri_plugin_cli::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepAll)
                .build(),
        )
        .plugin(tauri_plugin_notification::init())
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            invocable::get_sync_status,
            invocable::try_sync_repo,
            invocable::get_appdata_path,
            invocable::select_appdata_path,
            invocable::run_scriptfile,
            invocable::run_script,
            invocable::get_all_commands,
            invocable::build_form_html,
            invocable::build_form_json,
            invocable::submit_form,
            cli::get_cli_script
        ]);

    builder
        .run(tauri::generate_context!())
        .expect("ERROR WHILE RUNNING TAURI APPLICATION");
}
