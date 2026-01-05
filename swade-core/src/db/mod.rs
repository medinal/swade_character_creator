//! Database layer for SWADE character creator.
//!
//! This module handles:
//! - Embedded database distribution (the SQLite database is compiled into the binary)
//! - Automatic extraction and versioning (database is extracted to user data directory)
//! - Connection management
//!
//! # Usage
//!
//! ```ignore
//! use swade::db::Database;
//! use swade::repositories::EdgeRepository;
//!
//! // Initialize the database (extracts if needed)
//! let db = Database::init()?;
//!
//! // Get a connection
//! let conn = db.connection()?;
//!
//! // Use with repositories
//! let edges = EdgeRepository::get_all(&conn)?;
//! ```
//!
//! # Architecture
//!
//! The database is embedded into the binary at compile time using `include_bytes!()`.
//! At runtime, it's extracted to the user's data directory. A checksum is used to
//! determine if the database needs to be updated when a new version of the application
//! is installed.
//!
//! User data (characters) is preserved during updates through a migration process
//! that copies character tables from the old database to the new one.

mod connection;
mod embedded;

pub use connection::Database;
pub use embedded::{embedded_checksum, embedded_size, ensure_database};
