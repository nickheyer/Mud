use duckscript::types::error::ScriptError;
use serde::Serialize;
use tauri::Error as TauriError;

#[derive(Serialize)]
pub struct ScriptErrorResponse {
    stdout: String,
    stderr: String,
    message: String,
    line: Option<usize>,
}

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("Failed to read file: {0}")]
    Io(#[from] std::io::Error),
    #[error("File is not valid utf8: {0}")]
    Utf8(#[from] std::string::FromUtf8Error),
    #[error("Store Error: {0}")]
    StoreError(#[from] tauri_plugin_store::Error),
    #[error("Parsing Error: {0}")]
    ParsingError(String),
    #[error("Tauri Error: {0}")]
    Tauri(#[from] TauriError),
}

// we must also implement serde::Serialize
impl serde::Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

/*
Handle script errors and return a serialized JSON string.
*/
pub fn handle_script_error(
    error: ScriptError,
    stderr_output: String,
    stdout_output: String,
) -> String {
    // Determine the line number and construct the error message based on the error type
    let (error_message, line_number) = match error {
        ScriptError::Runtime(ref message, ref meta_info) => {
            let line = meta_info.as_ref().and_then(|info| info.line);
            (format!("Runtime Error: {}", message), line)
        }
        ScriptError::PreProcessNoCommandFound(ref meta_info) => (
            format!("PreProcessor Error: preprocessor is missing a command"),
            meta_info.line,
        ),
        ScriptError::ControlWithoutValidValue(ref meta_info) => format_error_message(
            "Error: control character found without a valid value",
            meta_info.line,
        ),
        ScriptError::InvalidControlLocation(ref meta_info) => {
            format_error_message("Error: invalid control character location", meta_info.line)
        }
        ScriptError::MissingEndQuotes(ref meta_info) => {
            format_error_message("Error: missing end quotes", meta_info.line)
        }
        ScriptError::MissingOutputVariableName(ref meta_info) => {
            format_error_message("Error: missing variable name", meta_info.line)
        }
        ScriptError::InvalidEqualsLocation(ref meta_info) => {
            format_error_message("Error: invalid equals sign location", meta_info.line)
        }
        ScriptError::InvalidQuotesLocation(ref meta_info) => {
            format_error_message("Error: invalid quotes location", meta_info.line)
        }
        ScriptError::EmptyLabel(ref meta_info) => {
            format_error_message("Error: empty label found", meta_info.line)
        }
        ScriptError::UnknownPreProcessorCommand(ref meta_info) => {
            format_error_message("Error: unknown preprocessor command", meta_info.line)
        }
        _ => (format!("Unknown error occurred"), None),
    };

    // Construct error json
    let script_error_response = ScriptErrorResponse {
        message: error_message,
        line: line_number,
        stderr: stderr_output,
        stdout: stdout_output,
    };

    // Serialize error json to json string, return to client
    serde_json::to_string(&script_error_response)
        // Return default on failure to serialize
        .unwrap_or_else(|_| {
            "{\"message\": \"Failed to serialize error\", \"line\": null}".to_string()
        })
}

fn format_error_message(message: &str, line: Option<usize>) -> (String, Option<usize>) {
    (message.to_string(), line)
}
