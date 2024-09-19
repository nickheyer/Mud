use std::io::{Cursor, Write};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::Sender;
use duckscript::types::env::Env;

// Using arc + mutex, TODO: improve w/ performant and efficient handling instead of reference counting
pub struct OutputCapture {
    stdout_buf: Arc<Mutex<Cursor<Vec<u8>>>>,
    stderr_buf: Arc<Mutex<Cursor<Vec<u8>>>>,
    stdout_tx: Sender<String>, // channel to send stdout
    stderr_tx: Sender<String>, // channel to send stderr
}

impl OutputCapture {
    pub fn new(stdout_tx: Sender<String>, stderr_tx: Sender<String>) -> Self {
        Self {
            stdout_buf: Arc::new(Mutex::new(Cursor::new(Vec::new()))),
            stderr_buf: Arc::new(Mutex::new(Cursor::new(Vec::new()))),
            stdout_tx,
            stderr_tx,
        }
    }

    pub fn as_env(&self) -> Env {
        Env::new(
            Some(Box::new(ArcWriter(self.stdout_buf.clone(), self.stdout_tx.clone())) as Box<dyn Write>),
            Some(Box::new(ArcWriter(self.stderr_buf.clone(), self.stderr_tx.clone())) as Box<dyn Write>),
        )
    }

    pub fn get_stdout(&self) -> String {
        let stdout_buf = self.stdout_buf.lock().unwrap();
        let stdout = String::from_utf8(stdout_buf.get_ref().clone()).unwrap_or_default();
        stdout
    }

    pub fn get_stderr(&self) -> String {
        let stderr_buf = self.stderr_buf.lock().unwrap();
        let stderr = String::from_utf8(stderr_buf.get_ref().clone()).unwrap_or_default();
        stderr
    }
}

// Wrapper for Arc<Mutex<Cursor<Vec<u8>>>> that implements Write
struct ArcWriter(Arc<Mutex<Cursor<Vec<u8>>>>, Sender<String>);

impl Write for ArcWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut cursor = self.0.lock().unwrap();
        let size = cursor.write(buf)?;

        // Send the new data through the channel
        let output = String::from_utf8(buf.to_vec()).unwrap_or_default();
        let _ = self.1.blocking_send(output);  // Send it to the channel

        Ok(size)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        let mut cursor = self.0.lock().unwrap();
        cursor.flush()
    }
}
