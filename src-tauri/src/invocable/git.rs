
#[tauri::command]
pub fn get_sync_status() -> Result<bool, ()> {
    Ok(true)
}
