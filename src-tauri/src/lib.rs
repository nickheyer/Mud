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
        ])
        .setup(|app| {
            // HANDLE CLI ARGUMENTS
            if let Ok(matches) = app.cli().matches() {
                println!("{:#?}", matches);
                let args = &matches.args;
                let cli_mode = match (args.get("str"), args.get("file"), args.get("run")) {
                    (Some(str_arg), _, _) if str_arg.value.is_string() => true,
                    (_, Some(file_arg), _) if file_arg.value.is_string() => true,
                    (_, _, Some(run_arg)) if run_arg.value.is_string() => true,
                    _ => false
                };

                if cli_mode {
                    let runtime = Runtime::new().unwrap();
                    if let Some(res) = runtime.block_on(cli::handle_cli_execution(app.handle().clone(), matches)) {
                        app.manage(cli::ScriptState(Mutex::new(Some(res))));
                    }
                } else {
                    app.manage(cli::ScriptState(Mutex::new(None)));
                }
            }
            Ok(())
        });

    builder
        .run(tauri::generate_context!())
        .expect("ERROR WHILE RUNNING TAURI APPLICATION");
}
