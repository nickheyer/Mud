use std::path::{PathBuf, Path};
use tauri::{AppHandle, Manager};
use tauri_plugin_dialog::{DialogExt, FilePath};
use git2::Repository;
use tokio::fs::{remove_dir_all, create_dir};

static COMMUNITY_REPO_URL: &str = "https://github.com/nickheyer/Mud.Community.git";
static COMMUNITY_REPO_ROOT_DIR: &str = "/mudCommunity";

#[tauri::command]
pub async fn get_sync_status() -> Result<bool, ()> {
    Ok(true)
}

#[tauri::command]
pub async fn try_sync_repo(app_data_dir: PathBuf) -> Result<bool, ()> {
    let is_target_ws = check_if_git(&app_data_dir);
    if is_target_ws {
        println!("{:#?} is an existing git workspace - attempting to clone repo from {:#?}",
            app_data_dir,
            &COMMUNITY_REPO_URL
        );
        match pull_repo_updates(&app_data_dir) {
            Ok(_) => Ok(true),
            Err(e) => {
                eprintln!("Failed to pull updates: {}", e);
                return Ok(false);
            },
        }
    } else {
        println!("{:#?} is not an existing git workspace - attempting to clone repo from {:#?}",
            app_data_dir,
            &COMMUNITY_REPO_URL
        );
        match clone_repo(&COMMUNITY_REPO_URL, &app_data_dir).await {
            Ok(_) => Ok(true),
            Err(e) => {
                eprintln!("Failed to initialize local repo: {}", e);
                return Ok(false);
            },
        }
    }
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

pub fn check_if_git(local_path: &PathBuf) -> bool {
    let repo_path = local_path.join(&COMMUNITY_REPO_ROOT_DIR);
    let repo = match Repository::open(&repo_path) {
        Ok(r) => r,
        Err(_) => return false // Not a git workspace
    };

    let remote = match repo.is_empty() {
        Ok(ans) => !ans,
        Err(_) => false // Not a git workspace
    };

    remote
}

pub async fn clone_repo(repo_url: &str, repo_path: &PathBuf) -> Result<(), git2::Error> {

    if repo_path.exists() {
        println!("Repository already exists at {:?}, removing before pull....", repo_path);
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

fn pull_repo_updates(local_path: &PathBuf) -> Result<(), git2::Error> {
    let repo = Repository::open(local_path)?;
    let mut remote = repo.find_remote("origin")?;
    
    // Fetch repo
    remote.fetch(&["main"], None, None)?;

    // Get latest
    let fetch_head = repo.find_reference("FETCH_HEAD")?;
    let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?;

    // Merge commits
    let analysis = repo.merge_analysis(&[&fetch_commit])?;
    if analysis.0.is_fast_forward() {
        let mut reference = repo.find_reference("refs/heads/main")?;
        reference
            .set_target(fetch_commit.id(), "Fast-forward")?;
        repo
            .set_head("refs/heads/main")?;
        repo
            .checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;
        println!("Repository updated successfully!");
    } else {
        println!("No fast-forward possible. Manual merge required.");
    }
    Ok(())
}
