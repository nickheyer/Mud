use git2::Repository;
use std::path::PathBuf;
use tauri::{AppHandle, Manager};
use tauri_plugin_dialog::{DialogExt, FilePath};
use tokio::fs::{create_dir, remove_dir_all};

pub static COMMUNITY_REPO_URL: &str = "https://github.com/nickheyer/Mud.Community.git";
pub static COMMUNITY_REPO_PATH: &str = "git";

#[tauri::command]
pub async fn get_sync_status() -> Result<bool, ()> {
    Ok(true)
}

#[tauri::command]
pub async fn try_sync_repo(app_data_dir: PathBuf) -> bool {
    let repo_path = app_data_dir.join(&COMMUNITY_REPO_PATH);
    let is_target_ws = check_if_git(&repo_path);
    if is_target_ws {
        match pull_repo_updates(&repo_path) {
            Ok(_) => {
                println!("Successfully synced existing local repo to community repo.");
                true
            }
            Err(e) => {
                eprintln!("Failed to pull updates: {}", e);
                false
            }
        }
    } else {
        println!(
            "{:#?} is not an existing git workspace - attempting to clone repo from {:#?}",
            repo_path, &COMMUNITY_REPO_URL
        );
        match clone_repo(&COMMUNITY_REPO_URL, &repo_path).await {
            Ok(_) => {
                println!("Successfully cloned community repo from github.");
                true
            }
            Err(e) => {
                eprintln!("Failed to initialize local repo: {}", e);
                false
            }
        }
    }
}

#[tauri::command]
pub async fn get_appdata_path(handle: AppHandle) -> Result<PathBuf, tauri::Error> {
    let local_app_data_dir = handle.path().app_local_data_dir()?;
    Ok(local_app_data_dir)
}

#[tauri::command]
pub async fn select_appdata_path(handle: AppHandle) -> Result<FilePath, tauri::Error> {
    let req_app_data_dir = handle.dialog().file().blocking_pick_folder();
    let local_app_data_dir = handle.path().app_local_data_dir()?;

    let resolved_path = req_app_data_dir.unwrap_or(FilePath::from(local_app_data_dir));
    println!("{:#?}", resolved_path);
    Ok(resolved_path)
}

pub fn check_if_git(local_path: &PathBuf) -> bool {
    let repo = match Repository::open(&local_path) {
        Ok(r) => r,
        Err(err) => {
            eprintln!("{:#?}", err);
            return false; // Not a git workspace
        }
    };
    let remote = match repo.is_empty() {
        Ok(ans) => !ans,
        Err(err) => {
            eprintln!("{:#?}", err);
            return false; // Not a git workspace
        }
    };

    remote
}

pub async fn clone_repo(repo_url: &str, repo_path: &PathBuf) -> Result<(), git2::Error> {
    if repo_path.exists() {
        println!(
            "Repository already exists at {:?}, removing before pull....",
            repo_path
        );
        match remove_dir_all(&repo_path).await {
            Ok(_) => {
                println!("Dir rm successfully!");
            }
            Err(e) => {
                eprintln!("Dir rm failed: {}", e);
            }
        }
    } else {
        match create_dir(&repo_path).await {
            Ok(_) => {
                println!("Dir created successfully!");
            }
            Err(e) => {
                eprintln!("Dir creation failed: {}", e);
            }
        }
    }
    println!("Cloning repository from {} to {:?}", repo_url, repo_path);
    match Repository::clone(repo_url, &repo_path) {
        Ok(_) => {
            println!("Repository cloned successfully!");
            Ok(())
        }
        Err(e) => {
            eprintln!("Failed to clone repository: {}", e);
            Err(e)
        }
    }
}

pub fn find_last_commit(repo: &Repository) -> Result<git2::Commit, git2::Error> {
    let obj = repo.head()?.resolve()?.peel(git2::ObjectType::Commit)?;
    match obj.into_commit() {
        Ok(c) => Ok(c),
        _ => Err(git2::Error::new(
            git2::ErrorCode::NotFound,
            git2::ErrorClass::Object,
            "commit error",
        )),
    }
}

fn pull_repo_updates(local_path: &PathBuf) -> Result<(), git2::Error> {
    let repo = Repository::open(local_path)?;
    let commit = find_last_commit(&repo)?;
    let obj = commit.into_object();
    let mut checkout_builder = git2::build::CheckoutBuilder::new();

    // Hard Reset
    repo.reset(&obj, git2::ResetType::Hard, Some(checkout_builder.force()))
}
