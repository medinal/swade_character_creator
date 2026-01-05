use thiserror::Error;

#[derive(Error, Debug)]
pub enum SwadeError {
    #[error("Database error: {0}")]
    Database(#[from] rusqlite::Error),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Failed to determine user data directory")]
    UserDataDir,

    #[error("Failed to extract embedded database: {0}")]
    Extraction(#[source] std::io::Error),

    #[error("Failed to migrate user data: {0}")]
    Migration(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}

pub type Result<T> = std::result::Result<T, SwadeError>;
