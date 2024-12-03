use crate::context::{forms, parser};
use crate::invocable::git::check_if_git;
use crate::utils::error_handler::AppError;
use serde_json::json;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;
use std::sync::Arc;
use tauri::{AppHandle, Manager, Wry};
use tauri_plugin_store::JsonValue;
use tauri_plugin_store::Store;
use tauri_plugin_store::StoreExt;

#[tauri::command]
pub async fn build_form_html(app: AppHandle) -> Result<String, AppError> {
    let def_html = "Error: Unable to load form data\n";
    let form_path = "games/baldursGate3/config.json".to_string(); // stub

    let store = app.store("store.bin")?;

    let app_data_path = get_res_appdata_path(app, &store);
    let repo_path = app_data_path.join("git");
    let is_target_ws = check_if_git(&repo_path);

    let json_object = if is_target_ws {
        let config_path = repo_path.join(&form_path);
        parser::load_json(config_path).map_err(|e| e.to_string())
    } else {
        Err(def_html.to_string())
    };

    match json_object {
        Ok(json) => Ok(forms::generate_form_html(json, None).unwrap_or(def_html.to_string())),
        Err(_e) => {
            Err(AppError::ParsingError(def_html.to_string()))
        }
    }
}

#[tauri::command]
pub async fn build_form_json(app: AppHandle) -> Result<JsonValue, AppError> {
    let def_json = json!({ "error": "Could not parse settings file. Try resyncing repo." });
    let form_path = "games/baldursGate3/config.json".to_string(); // stub

    let store = app.store("store.bin")?;

    let app_data_path = get_res_appdata_path(app, &store);
    let repo_path = app_data_path.join("git");
    let is_target_ws = check_if_git(&repo_path);

    if is_target_ws {
        let config_path = repo_path.join(&form_path);
        let hash_key = hash_path(&config_path);
        match parser::load_json(config_path) {
            Ok(mut parsed_json) => {
                if let Some(existing_data) = load_existing_data(&store, hash_key) {
                    parsed_json = merge_form_data(parsed_json, existing_data);
                }
                Ok(parsed_json)
            }
            Err(_e) => Err(AppError::ParsingError(def_json.to_string())),
        }
    } else {
        Err(AppError::ParsingError(def_json.to_string()))
    }
}

#[tauri::command]
pub async fn submit_form(app: AppHandle, form_data: JsonValue) -> Result<(), AppError> {
    let store = app.store("store.bin")?;
    let form_path = "games/baldursGate3/config.json".to_string(); // stub

    let app_data_path = get_res_appdata_path(app, &store);
    let repo_path = app_data_path.join("git");
    let config_path = repo_path.join(&form_path);
    let hash_key = hash_path(&config_path);

    if let Some(_map) = form_data.as_object() {
        store.set(hash_key.to_string(), form_data);
        store.save()?
    } else {
        return Err(AppError::ParsingError(
            "Invalid form data format".to_string(),
        ));
    }
    Ok(())
}

fn get_res_appdata_path(app: AppHandle, store: &Arc<Store<Wry>>) -> PathBuf {
    let app_data_default = app.path().app_local_data_dir().unwrap();

    let app_data_dir = store
        .get("app-data-custom")
        .and_then(|s| s["value"].as_str().map(String::from))
        .unwrap_or_else(|| app_data_default.into_os_string().into_string().unwrap());

    PathBuf::from(app_data_dir.replace("\"", ""))
}

// Using this to store our form data
fn hash_path(path: &PathBuf) -> u64 {
    let mut hasher = DefaultHasher::new();
    path.hash(&mut hasher);
    hasher.finish()
}

// Load the form data if it exists
fn load_existing_data(store: &Arc<Store<Wry>>, key: u64) -> Option<JsonValue> {
    store.get(&key.to_string())
}

fn merge_form_data(mut form_json: JsonValue, existing_data: JsonValue) -> JsonValue {
    if let Some(existing_map) = existing_data.as_object() {
        if let Some(settings) = form_json.get_mut("settings").and_then(|s| s.as_array_mut()) {
            for setting in settings.iter_mut() {
                if let Some(key) = setting.get("key").and_then(|k| k.as_str()) {
                    if let Some(value) = existing_map.get(key) {
                        setting["value"] = value.clone();
                    }
                }
            }
        }
    }
    form_json
}
