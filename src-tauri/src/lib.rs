mod context;
mod invocable;
mod output;
mod utils;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_log::Builder::new()
            .rotation_strategy(tauri_plugin_log::RotationStrategy::KeepAll)
            .build())
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
            invocable::build_form_json
        ])
        .run(tauri::generate_context!())
        .expect("error while running Tauri application");
}
