# Database Layer

This module handles embedded database distribution and connection management for the SWADE character creator.

## Overview

The SQLite database containing all SWADE game rules is **embedded directly into the compiled binary**. At runtime, the database is extracted to the user's data directory and only updated when the embedded version changes.

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Compile Time                              │
├─────────────────────────────────────────────────────────────┤
│  ./db/swade.db  ──────►  include_bytes!()  ──────►  Binary  │
│                          (embedded.rs)                       │
└─────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌─────────────────────────────────────────────────────────────┐
│                     Runtime                                  │
├─────────────────────────────────────────────────────────────┤
│  1. Calculate checksum of embedded database                  │
│  2. Check if extracted database exists with matching version │
│  3. If missing or outdated → extract embedded database       │
│  4. Open connection to extracted database                    │
└─────────────────────────────────────────────────────────────┘
```

## Files

| File | Purpose |
|------|---------|
| `mod.rs` | Module exports |
| `connection.rs` | Database connection management |
| `embedded.rs` | Embedded database extraction and version checking |

## Embedded Database Strategy

### Why Embed?

1. **Zero Setup** - Users don't need to download or configure a database
2. **Version Consistency** - Game rules data always matches the application version
3. **Offline Support** - No network required for seed data
4. **Atomic Updates** - Database updates happen with application updates

### Version Tracking

The embedded database uses a **checksum-based versioning** strategy:

```rust
// At compile time, embed the database and its checksum
const EMBEDDED_DB: &[u8] = include_bytes!("../../../db/swade.db");
const EMBEDDED_DB_CHECKSUM: &str = env!("SWADE_DB_CHECKSUM");
```

The checksum is calculated during build via `build.rs` and stored alongside the extracted database. On startup:

1. Read the stored checksum from the user data directory
2. Compare against `EMBEDDED_DB_CHECKSUM`
3. If different (or missing), extract the new database

### User Data Directory

The extracted database is stored in the platform-appropriate user data directory:

| Platform | Location |
|----------|----------|
| Linux | `~/.local/share/swade/swade.db` |
| macOS | `~/Library/Application Support/swade/swade.db` |
| Windows | `%APPDATA%\swade\swade.db` |

### Preserving User Data

The embedded database only contains **seed data** (game rules). User-created characters are stored in the same database but in separate tables. When updating:

1. Extract embedded database to a temporary location
2. Migrate user data from the old database to the new one
3. Replace the old database with the updated one

This ensures game rules updates don't destroy player characters.

## Connection Management

### Connection Factory

```rust
pub struct Database {
    path: PathBuf,
}

impl Database {
    /// Initialize the database, extracting embedded data if needed
    pub fn init() -> Result<Self>;

    /// Get a new connection to the database
    pub fn connection(&self) -> Result<Connection>;
}
```

### Usage Pattern

```rust
let db = Database::init()?;
let conn = db.connection()?;

// Use repositories with the connection
let edges = EdgeRepository::get_all(&conn)?;
```

## Build Integration

### build.rs

The build script calculates the database checksum and sets it as an environment variable:

```rust
// build.rs
use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;

fn main() {
    // Database is at workspace root, build.rs runs from swade-core/
    let db_path = Path::new("../db/swade.db");

    // Calculate SHA256 checksum of the database
    if db_path.exists() {
        let db_bytes = fs::read(db_path).expect("Failed to read ../db/swade.db");

        let mut hasher = Sha256::new();
        hasher.update(&db_bytes);
        let checksum = format!("{:x}", hasher.finalize());

        // Set as environment variable for compile-time inclusion
        println!("cargo:rustc-env=SWADE_DB_CHECKSUM={}", checksum);
    } else {
        // Provide a placeholder checksum if database doesn't exist yet
        // This allows the project to compile without the database for CI/testing
        println!("cargo:rustc-env=SWADE_DB_CHECKSUM=no_database");
    }

    // Rebuild if database changes
    println!("cargo:rerun-if-changed=../db/swade.db");
    println!("cargo:rerun-if-changed=build.rs");
}
```

### Dependencies

Add to `Cargo.toml`:

```toml
[build-dependencies]
sha2 = "0.10"
```

## Implementation Guidelines

### embedded.rs

Implements:

```rust
/// The embedded database bytes
const EMBEDDED_DB: &[u8] = include_bytes!("../../../db/swade.db");

/// The checksum of the embedded database (set by build.rs)
const EMBEDDED_DB_CHECKSUM: &str = env!("SWADE_DB_CHECKSUM");

// Public functions (exported via mod.rs)

/// Extract the embedded database to the user data directory if needed
pub fn ensure_database() -> Result<PathBuf>;

/// Returns the checksum of the embedded database
pub fn embedded_checksum() -> &'static str;

/// Returns the size of the embedded database in bytes
pub fn embedded_size() -> usize;

// Private helper functions

/// Get the path to the user data directory
fn user_data_dir() -> Result<PathBuf>;

/// Check if the extracted database matches the embedded version
fn needs_update(data_dir: &Path) -> Result<bool>;

/// Extract the embedded database, preserving user data
fn extract_database(db_path: &Path) -> Result<()>;

/// Write the current embedded checksum to the data directory
fn write_checksum(data_dir: &Path) -> Result<()>;

/// Migrate user data from old database to new database
fn migrate_user_data(old_db: &Path, new_db: &Path) -> Result<()>;

/// Gets the column names for a table
fn get_table_columns(conn: &Connection, table: &str) -> Result<Vec<String>>;
```

### connection.rs

Implements:

```rust
use rusqlite::Connection;

/// Database handle for obtaining connections
pub struct Database {
    path: PathBuf,
}

impl Database {
    /// Initialize the database system
    pub fn init() -> Result<Self>;

    /// Create a database handle from a specific path (useful for testing)
    pub fn from_path(path: PathBuf) -> Self;

    /// Get a new database connection (foreign keys disabled by default)
    pub fn connection(&self) -> Result<Connection>;

    /// Get a connection with foreign keys enabled
    pub fn connection_with_fk(&self) -> Result<Connection>;

    /// Returns the path to the database file
    pub fn path(&self) -> &PathBuf;
}
```

## Testing

### Unit Tests

Test the version checking and extraction logic with temporary directories:

```rust
#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    #[test]
    fn test_needs_update_missing_db() {
        let temp = TempDir::new().unwrap();

        assert!(needs_update(temp.path()).unwrap());
    }

    #[test]
    fn test_needs_update_current_version() {
        // Create a database with matching checksum
        // Assert no update needed
    }

    #[test]
    fn test_extract_preserves_user_data() {
        // Create old database with character data
        // Extract new embedded database
        // Assert character data is preserved
    }
}
```

### Integration Tests

Test the full initialization flow:

```rust
#[test]
fn test_database_init_creates_db() {
    let db = Database::init().unwrap();
    let conn = db.connection().unwrap();

    // Verify seed data is present
    let attrs = AttributeRepository::get_all(&conn).unwrap();
    assert_eq!(attrs.len(), 5); // 5 SWADE attributes
}
```

## Error Handling

The database layer uses the crate-wide `SwadeError` type defined in `src/error.rs`:

```rust
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
```
