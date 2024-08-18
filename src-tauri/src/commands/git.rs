use git2::Repository;
use std::path::Path;
use std::fs::{remove_dir_all, create_dir};
use tauri;



#[tauri::command]
pub fn sync_with_repo(repo_url: String, local_path: String) -> String {
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

pub fn clone_repo(repo_url: &str, local_path: &str) -> Result<(), git2::Error> {
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

pub fn pull_repo_updates(local_path: &str) -> Result<(), git2::Error> {
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

