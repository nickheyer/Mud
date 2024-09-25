use tauri::{AppHandle, Manager};

#[tauri::command]
pub async fn get_sync_status() -> Result<bool, ()> {
    Ok(true)
}

#[tauri::command]
pub async fn try_sync_repo(handle: AppHandle) -> Result<bool, tauri::Error> {
    let local_app_data_dir = handle
                                        .path()
                                        .app_local_data_dir()?;

    let local_app_data = local_app_data_dir
                                .to_str()
                                .unwrap();

    println!("{:#?}", local_app_data);
    Ok(true)
}
