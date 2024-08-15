// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::Command;
use std::fs::{self, File, create_dir, remove_dir_all};
use std::io::{ErrorKind, Error, Write};
use git2::Repository;
use std::path::Path;
use dirs::home_dir;

fn create_mud_directory_and_file(name: &str) -> Result<String, Error> {
    let home_dir = home_dir().expect("Failed to get home directory");
    let mud_path = home_dir.join(".mud");

    if !mud_path.exists() {
        println!("Attempting to create dir: {:?}", mud_path);
        fs::create_dir(&mud_path)
            .map_err(|e| Error::new(ErrorKind::Other, format!("Failed to create .mud directory: {}", e)))?;
    }

    let mud_test_file_path = mud_path.join("mudTest.txt");

    if !mud_test_file_path.exists() {
        let mut mud_buf = File::create(&mud_test_file_path)
            .map_err(|e| Error::new(ErrorKind::Other, format!("Failed to create mudTest file: {}", e)))?;
        write!(mud_buf, "{}, welcome to the magical world of MUD!", name)?;
    }

    Ok(mud_test_file_path
        .into_os_string()
        .into_string()
        .unwrap()
      )
}

fn clone_repo(repo_url: &str, local_path: &str) -> Result<(), git2::Error> {
  let repo_path = Path::new(local_path).join("mudCommunity");

  if repo_path.exists() {
      println!("Repository already exists at {:?}, removing before pull....", repo_path);
      match remove_dir_all(&repo_path) {
        Ok(_) => {
          println!("Dir rm successfully!");
        }
        Err(e) => {
          eprintln!("Dir rm failed: {}", e);
        }
      }
  } else {
    match create_dir(&repo_path) {
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

fn pull_repo_updates(local_path: &str) -> Result<(), git2::Error> {
  let repo = Repository::open(local_path)?;
  let mut remote = repo.find_remote("origin")?;
  
  // Fetch updates from repo
  remote.fetch(&["main"], None, None)?;

  // Get the latest commit from the fetched branch
  let fetch_head = repo.find_reference("FETCH_HEAD")?;
  let fetch_commit = repo.reference_to_annotated_commit(&fetch_head)?;

  // Merge the fetched commit into the local branch
  let analysis = repo.merge_analysis(&[&fetch_commit])?;
  if analysis.0.is_fast_forward() {
      let mut reference = repo.find_reference("refs/heads/main")?;
      reference.set_target(fetch_commit.id(), "Fast-forward")?;
      repo.set_head("refs/heads/main")?;
      repo.checkout_head(Some(git2::build::CheckoutBuilder::default().force()))?;
      println!("Repository updated successfully!");
  } else {
      println!("No fast-forward possible. Manual merge required.");
  }

  Ok(())
}



#[tauri::command]
fn create_mud(name: &str) -> String {
    match create_mud_directory_and_file(&name) {
        Ok(fp) => {
          let msg = format!("MUD directory and file created successfully: {:?}", fp);
          println!("Results of command: {}", &msg);
          msg
        },
        Err(e) => format!("Failed to create MUD directory and file: {}", e),
    }
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted by Mud!", name)
}

#[tauri::command]
fn install_mods(files: Vec<String>) -> String {
    let python_script = "/home/nick/code/Mud/scripts/test.py";

    for file in files {
        let output = Command::new("python3")
            .arg(python_script)
            .arg(&file)
            .output()
            .map_err(|e| Error::new(ErrorKind::Other, format!("Failed to run Python script: {}", e)))
            .unwrap();

        if !output.status.success() {
            return format!("Failed to install mod: {}", String::from_utf8_lossy(&output.stderr));
        }
    }

    "All mods installed successfully.".to_string()
}

#[tauri::command]
fn sync_with_repo(repo_url: String, local_path: String) -> String {
    match clone_repo(&repo_url, &local_path) {
        Ok(_) => {
            match pull_repo_updates(&local_path) {
                Ok(_) => "Repository synced successfully!".to_string(),
                Err(e) => format!("Failed to pull updates: {}", e),
            }
        }
        Err(e) => format!("Failed to clone repository: {}", e),
    }
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, create_mud, install_mods, sync_with_repo])
        .run(tauri::generate_context!())
        .expect("error while running mud application");
}
