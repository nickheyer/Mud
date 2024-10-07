use std::path::PathBuf;
use serde_json::json;
use tauri_plugin_store::JsonValue;
use tauri::{AppHandle, Manager};
use tauri_plugin_store::StoreBuilder;
use crate::context::{forms, parser};
use crate::invocable::git::check_if_git;

#[tauri::command]
pub async fn build_form_html(app: AppHandle) -> Result<String, tauri::Error> {
    let def_html = "Error: Unable to load form data\n";
    
    let store = StoreBuilder::new(&app, "store.bin").build();
    let app_data_default = app.path().app_local_data_dir()?;

    let app_data_dir = store.get("app-data-custom")
        .and_then(|s| s["value"].as_str().map(String::from))
        .unwrap_or_else(|| app_data_default.into_os_string().into_string().unwrap());
    
    let app_data_path = PathBuf::from(app_data_dir.replace("\"", ""));
    let repo_path = app_data_path.join("git");
    let is_target_ws = check_if_git(&repo_path);

    let json_object = if is_target_ws {
        let config_path = repo_path.join("games/baldursGate3/config.json");
        parser::load_json(config_path).map_err(|e| e.to_string())
    } else {
        Err(def_html.to_string())
    };

    let rendered_html = match json_object {
        Ok(json) => Ok(forms::generate_form_html(json, None).unwrap_or(def_html.to_string())),
        Err(e) => {
            println!("{:#?}", e);
            Err(tauri::Error::AssetNotFound(def_html.to_string()))
        }
    };

    rendered_html
}

#[tauri::command]
pub async fn build_form_json(app: AppHandle) -> Result<JsonValue, JsonValue> {
    let def_json = json!({ "error": "Could not parse settings file. Try resyncing repo." });

    let store = StoreBuilder::new(&app, "store.bin").build();
    let app_data_default = app.path().app_local_data_dir().unwrap();

    let app_data_dir = store.get("app-data-custom")
        .and_then(|s| s["value"].as_str().map(String::from))
        .unwrap_or_else(|| app_data_default.into_os_string().into_string().unwrap());
    
    let app_data_path = PathBuf::from(app_data_dir.replace("\"", ""));
    let repo_path = app_data_path.join("git");
    let is_target_ws = check_if_git(&repo_path);

    if is_target_ws {
        let config_path = repo_path.join("games/baldursGate3/config.json");
        match parser::load_json(config_path) {
            Ok(parsed_json) => Ok(parsed_json),
            Err(e) => Err(json!({ "error": format!("Failed to load config: {}", e) })),
        }
    } else {
        Err(def_json)
    }
}