//! Embedded database extraction and versioning
//!
//! This module handles extracting the embedded SQLite database to the user's
//! data directory and updating it when the embedded version changes.

use crate::error::{Result, SwadeError};
use rusqlite::Connection;
use std::fs;
use std::path::{Path, PathBuf};

/// The embedded database bytes (compiled into the binary)
const EMBEDDED_DB: &[u8] = include_bytes!("../../../db/swade.db");

/// The checksum of the embedded database (set by build.rs)
const EMBEDDED_DB_CHECKSUM: &str = env!("SWADE_DB_CHECKSUM");

/// Name of the application for the data directory
const APP_NAME: &str = "swade";

/// Name of the database file
const DB_FILENAME: &str = "swade.db";

/// Name of the checksum file stored alongside the database
const CHECKSUM_FILENAME: &str = "swade.db.checksum";

/// Ensures the database exists and is up to date in the user data directory.
/// Returns the path to the database file.
pub fn ensure_database() -> Result<PathBuf> {
    let data_dir = user_data_dir()?;
    let db_path = data_dir.join(DB_FILENAME);

    // Create the data directory if it doesn't exist
    if !data_dir.exists() {
        fs::create_dir_all(&data_dir)?;
    }

    // Check if we need to update the database
    if needs_update(&data_dir)? {
        extract_database(&db_path)?;
        write_checksum(&data_dir)?;
    }

    Ok(db_path)
}

/// Returns the path to the user data directory for this application.
fn user_data_dir() -> Result<PathBuf> {
    #[cfg(target_os = "linux")]
    {
        if let Ok(xdg_data) = std::env::var("XDG_DATA_HOME") {
            return Ok(PathBuf::from(xdg_data).join(APP_NAME));
        }
        if let Ok(home) = std::env::var("HOME") {
            return Ok(PathBuf::from(home).join(".local/share").join(APP_NAME));
        }
    }

    #[cfg(target_os = "macos")]
    {
        if let Ok(home) = std::env::var("HOME") {
            return Ok(PathBuf::from(home)
                .join("Library/Application Support")
                .join(APP_NAME));
        }
    }

    #[cfg(target_os = "windows")]
    {
        if let Ok(appdata) = std::env::var("APPDATA") {
            return Ok(PathBuf::from(appdata).join(APP_NAME));
        }
    }

    // Fallback: try HOME on any platform
    if let Ok(home) = std::env::var("HOME") {
        return Ok(PathBuf::from(home).join(format!(".{}", APP_NAME)));
    }

    Err(SwadeError::UserDataDir)
}

/// Checks if the database needs to be updated by comparing checksums.
fn needs_update(data_dir: &Path) -> Result<bool> {
    let db_path = data_dir.join(DB_FILENAME);
    let checksum_path = data_dir.join(CHECKSUM_FILENAME);

    // Need update if database doesn't exist
    if !db_path.exists() {
        return Ok(true);
    }

    // Need update if checksum file doesn't exist
    if !checksum_path.exists() {
        return Ok(true);
    }

    // Compare stored checksum with embedded checksum
    let stored_checksum = fs::read_to_string(&checksum_path)?;
    Ok(stored_checksum.trim() != EMBEDDED_DB_CHECKSUM)
}

/// Extracts the embedded database to the specified path.
/// If a database already exists with user data, migrates the user data.
fn extract_database(db_path: &Path) -> Result<()> {
    let has_existing_db = db_path.exists();

    if has_existing_db {
        // Database exists - need to migrate user data
        let temp_path = db_path.with_extension("db.new");

        // Write embedded database to temp location
        fs::write(&temp_path, EMBEDDED_DB).map_err(SwadeError::Extraction)?;

        // Migrate user data from old database to new
        migrate_user_data(db_path, &temp_path)?;

        // Replace old database with new one
        fs::rename(&temp_path, db_path).map_err(SwadeError::Extraction)?;
    } else {
        // No existing database - just extract
        fs::write(db_path, EMBEDDED_DB).map_err(SwadeError::Extraction)?;
    }

    Ok(())
}

/// Writes the current embedded checksum to the data directory.
fn write_checksum(data_dir: &Path) -> Result<()> {
    let checksum_path = data_dir.join(CHECKSUM_FILENAME);
    fs::write(&checksum_path, EMBEDDED_DB_CHECKSUM)?;
    Ok(())
}

/// Migrates user data (characters) from the old database to the new database.
fn migrate_user_data(old_db_path: &Path, new_db_path: &Path) -> Result<()> {
    let old_conn = Connection::open(old_db_path)?;
    let new_conn = Connection::open(new_db_path)?;

    // Attach the old database to the new connection
    new_conn.execute(
        "ATTACH DATABASE ?1 AS old_db",
        [old_db_path.to_string_lossy().as_ref()],
    )?;

    // List of character tables to migrate (order matters for foreign keys)
    let character_tables = [
        "characters",
        "character_attributes",
        "character_skills",
        "character_edges",
        "character_hindrances",
        "character_powers",
        "character_modifiers",
        "character_arcane_backgrounds",
        "character_ancestry_choices",
        "character_notes",
        "character_gear",
        "character_advancements",
    ];

    // Migrate each character table
    for table in character_tables {
        // Check if table exists in old database and has data
        let has_data: bool = old_conn
            .query_row(
                "SELECT EXISTS(SELECT 1 FROM sqlite_master WHERE type='table' AND name=?1)",
                [table],
                |row| row.get(0),
            )
            .unwrap_or(false);

        if has_data {
            // Get column names from the new table
            let columns = get_table_columns(&new_conn, table)?;

            if !columns.is_empty() {
                let columns_str = columns.join(", ");

                // Copy data from old table to new table
                let sql = format!(
                    "INSERT OR REPLACE INTO main.{} ({}) SELECT {} FROM old_db.{}",
                    table, columns_str, columns_str, table
                );

                // Ignore errors if old table doesn't have the data
                let _ = new_conn.execute(&sql, []);
            }
        }
    }

    // Detach the old database
    new_conn.execute("DETACH DATABASE old_db", [])?;

    Ok(())
}

/// Gets the column names for a table.
fn get_table_columns(conn: &Connection, table: &str) -> Result<Vec<String>> {
    let mut stmt = conn.prepare(&format!("PRAGMA table_info({})", table))?;
    let columns: Vec<String> = stmt
        .query_map([], |row| row.get::<_, String>(1))?
        .filter_map(|r| r.ok())
        .collect();
    Ok(columns)
}

/// Returns the checksum of the embedded database.
/// Useful for debugging and verification.
pub fn embedded_checksum() -> &'static str {
    EMBEDDED_DB_CHECKSUM
}

/// Returns the size of the embedded database in bytes.
pub fn embedded_size() -> usize {
    EMBEDDED_DB.len()
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_embedded_db_is_not_empty() {
        assert!(
            !EMBEDDED_DB.is_empty(),
            "Embedded database should not be empty"
        );
    }

    #[test]
    fn test_embedded_checksum_is_set() {
        assert!(
            !EMBEDDED_DB_CHECKSUM.is_empty(),
            "Embedded checksum should not be empty"
        );
        assert_ne!(
            EMBEDDED_DB_CHECKSUM, "no_database",
            "Embedded checksum should be a real checksum, not placeholder"
        );
    }

    #[test]
    fn test_needs_update_missing_db() {
        let temp = TempDir::new().unwrap();
        assert!(needs_update(temp.path()).unwrap());
    }

    #[test]
    fn test_needs_update_missing_checksum() {
        let temp = TempDir::new().unwrap();
        let db_path = temp.path().join(DB_FILENAME);
        fs::write(&db_path, b"test").unwrap();

        assert!(needs_update(temp.path()).unwrap());
    }

    #[test]
    fn test_needs_update_outdated_checksum() {
        let temp = TempDir::new().unwrap();
        let db_path = temp.path().join(DB_FILENAME);
        let checksum_path = temp.path().join(CHECKSUM_FILENAME);

        fs::write(&db_path, b"test").unwrap();
        fs::write(&checksum_path, "old_checksum").unwrap();

        assert!(needs_update(temp.path()).unwrap());
    }

    #[test]
    fn test_needs_update_current_version() {
        let temp = TempDir::new().unwrap();
        let db_path = temp.path().join(DB_FILENAME);
        let checksum_path = temp.path().join(CHECKSUM_FILENAME);

        fs::write(&db_path, b"test").unwrap();
        fs::write(&checksum_path, EMBEDDED_DB_CHECKSUM).unwrap();

        assert!(!needs_update(temp.path()).unwrap());
    }

    #[test]
    fn test_extract_database_new() {
        let temp = TempDir::new().unwrap();
        let db_path = temp.path().join(DB_FILENAME);

        extract_database(&db_path).unwrap();

        assert!(db_path.exists());
        let extracted = fs::read(&db_path).unwrap();
        assert_eq!(extracted, EMBEDDED_DB);
    }

    #[test]
    fn test_write_checksum() {
        let temp = TempDir::new().unwrap();

        write_checksum(temp.path()).unwrap();

        let checksum_path = temp.path().join(CHECKSUM_FILENAME);
        assert!(checksum_path.exists());

        let stored = fs::read_to_string(&checksum_path).unwrap();
        assert_eq!(stored, EMBEDDED_DB_CHECKSUM);
    }

    #[test]
    fn test_full_extraction_workflow() {
        let temp = TempDir::new().unwrap();
        let data_dir = temp.path().join("subdir");

        // Test the extraction workflow directly (not ensure_database which depends on HOME)
        fs::create_dir_all(&data_dir).unwrap();
        let db_path = data_dir.join(DB_FILENAME);

        // Initially needs update (no db exists)
        assert!(needs_update(&data_dir).unwrap());

        // Extract the database
        extract_database(&db_path).unwrap();
        write_checksum(&data_dir).unwrap();

        // Verify files exist
        assert!(db_path.exists());
        assert!(data_dir.join(CHECKSUM_FILENAME).exists());

        // No longer needs update
        assert!(!needs_update(&data_dir).unwrap());
    }

    #[test]
    fn test_extract_preserves_user_data() {
        let temp = TempDir::new().unwrap();
        let db_path = temp.path().join(DB_FILENAME);

        // First, extract the embedded database
        extract_database(&db_path).unwrap();

        // Insert a test character
        let conn = Connection::open(&db_path).unwrap();
        conn.execute(
            "INSERT INTO characters (id, is_wild_card, name,
                attribute_points_spent, attribute_points_earned, skill_points_spent,
                skill_points_earned, hindrance_points_spent, hindrance_points_earned,
                hindrance_points_to_edges, hindrance_points_to_attributes,
                hindrance_points_to_skills, hindrance_points_to_wealth, power_points, wealth)
             VALUES (1, 1, 'Test Hero', 0, 5, 0, 12, 0, 0, 0, 0, 0, 0, 0, 0)",
            [],
        )
        .unwrap();
        drop(conn);

        // Simulate an update by re-extracting
        extract_database(&db_path).unwrap();

        // Verify the character data was preserved
        let conn = Connection::open(&db_path).unwrap();
        let name: String = conn
            .query_row("SELECT name FROM characters WHERE id = 1", [], |row| {
                row.get(0)
            })
            .unwrap();
        assert_eq!(name, "Test Hero");
    }

    #[test]
    fn test_get_table_columns() {
        let temp = TempDir::new().unwrap();
        let db_path = temp.path().join(DB_FILENAME);
        extract_database(&db_path).unwrap();

        let conn = Connection::open(&db_path).unwrap();
        let columns = get_table_columns(&conn, "characters").unwrap();

        assert!(columns.contains(&"id".to_string()));
        assert!(columns.contains(&"name".to_string()));
        assert!(columns.contains(&"is_wild_card".to_string()));
    }
}
