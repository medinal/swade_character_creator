//! Structured error type for Tauri commands.

use serde::Serialize;
use specta::Type;
use swade_core::error::SwadeError;

/// Error type for Tauri commands, providing structured error information to the frontend.
#[derive(Debug, Clone, Serialize, Type)]
#[serde(tag = "kind", content = "message")]
pub enum CommandError {
    /// Database operation failed
    Database(String),

    /// Requested entity was not found
    NotFound(String),

    /// Validation constraint was violated
    Validation(String),

    /// No draft character in progress
    NoDraft(String),

    /// Internal state error (mutex, etc.)
    State(String),
}

impl std::fmt::Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CommandError::Database(msg) => write!(f, "Database error: {}", msg),
            CommandError::NotFound(msg) => write!(f, "{}", msg),
            CommandError::Validation(msg) => write!(f, "{}", msg),
            CommandError::NoDraft(msg) => write!(f, "{}", msg),
            CommandError::State(msg) => write!(f, "State error: {}", msg),
        }
    }
}

impl std::error::Error for CommandError {}

impl From<SwadeError> for CommandError {
    fn from(err: SwadeError) -> Self {
        match err {
            SwadeError::Database(e) => CommandError::Database(e.to_string()),
            SwadeError::NotFound(msg) => CommandError::NotFound(msg),
            SwadeError::Validation(msg) => CommandError::Validation(msg),
            SwadeError::UserDataDir => CommandError::State("Failed to determine user data directory".to_string()),
            SwadeError::Extraction(e) => CommandError::State(format!("Failed to extract database: {}", e)),
            SwadeError::Migration(msg) => CommandError::State(format!("Migration failed: {}", msg)),
            SwadeError::Io(e) => CommandError::State(format!("IO error: {}", e)),
        }
    }
}

/// Shorthand result type for Tauri commands.
pub type CommandResult<T> = Result<T, CommandError>;
