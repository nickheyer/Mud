use tokio::task;

/// Pass in sync, spins up a task, profit
/// Going to try to prioritize channels for bidirectional communication,
/// but this is perfect for set and forget task.
pub async fn run_sync_task<T, F>(task: F) -> Result<T, String>
where
    F: FnOnce() -> T + Send + 'static,
    T: Send + 'static,
{
    // Using tokio spawn blocking
    match task::spawn_blocking(task).await {
        Ok(result) => Ok(result),
        Err(err) => Err(err.to_string()),
    }
}
