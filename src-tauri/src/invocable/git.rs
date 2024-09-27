use std::path::{PathBuf, Path};
use tauri::{AppHandle, Manager};
use tauri_plugin_dialog::{DialogExt, FilePath};
use tauri_plugin_fs;
use git2::Repository;

#[tauri::command]
pub async fn get_sync_status() -> Result<bool, ()> {
    Ok(true)
}

#[tauri::command]
pub async fn try_sync_repo(handle: AppHandle) -> Result<bool, tauri::Error> {
    let local_app_data_dir = handle.path().app_local_data_dir()?;

    let local_app_data = local_app_data_dir.to_str().unwrap();

    println!("{:#?}", local_app_data);
    Ok(true)
}

#[tauri::command]
pub async fn get_appdata_path(handle: AppHandle) -> Result<PathBuf, tauri::Error> {
    let local_app_data_dir = handle.path().app_local_data_dir()?;

    println!("{:#?}", local_app_data_dir);
    Ok(local_app_data_dir)
}

#[tauri::command]
pub async fn select_appdata_path(handle: AppHandle) -> Result<Vec<FilePath>, tauri::Error> {
    let req_app_data_dir = handle.dialog().file().blocking_pick_folders();
    let local_app_data_dir = handle.path().app_local_data_dir()?;
    
    let resolved_path = req_app_data_dir.unwrap_or(vec!(FilePath::from(local_app_data_dir)));
    println!("{:#?}", resolved_path);
    Ok(resolved_path)
}

// fn clone_repo(repo_url: &str, local_path: &str) -> Result<(), git2::Error> {
//     let repo_path = Path::new(local_path).join("mudCommunity");
//     if repo_path.exists() {
//         println!("Repository already exists at {:?}, removing before pull....", repo_path);
//         match remove_dir_all(&repo_path) {
//           Ok(_) => {
//             println!("Dir rm successfully!");
//           }
//           Err(e) => {
//             eprintln!("Dir rm failed: {}", e);
//           }
//         }
//     } else {
//       match create_dir(&repo_path) {
//         Ok(_) => {
//           println!("Dir created successfully!");
//         }
//         Err(e) => {
//           eprintln!("Dir creation failed: {}", e);
//         }
//       }
//     }
//     println!("Cloning repository from {} to {:?}", repo_url, repo_path);
//     match Repository::clone(repo_url, &repo_path) {
//         Ok(_) => {
//             println!("Repository cloned successfully!");
//             Ok(())
//         }
//         Err(e) => {
//             eprintln!("Failed to clone repository: {}", e);
//             Err(e)
//         }
//     }
//   }