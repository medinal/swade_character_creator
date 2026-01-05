//! Database connection management
//!
//! This module provides a `Database` struct for managing connections to the
//! SWADE SQLite database.

use crate::error::Result;
use rusqlite::Connection;
use std::path::PathBuf;

use super::embedded::ensure_database;

/// Database handle for obtaining connections to the SWADE database.
///
/// The `Database` struct manages the path to the SQLite database and provides
/// methods for obtaining new connections. It ensures the embedded database is
/// extracted and up-to-date before providing connections.
///
/// # Example
///
/// ```ignore
/// use swade::db::Database;
/// use swade::repositories::EdgeRepository;
///
/// let db = Database::init()?;
/// let conn = db.connection()?;
///
/// let edges = EdgeRepository::get_all(&conn)?;
/// ```
pub struct Database {
    path: PathBuf,
}

impl Database {
    /// Initialize the database system.
    ///
    /// This will:
    /// 1. Determine the user data directory
    /// 2. Check if the database needs to be extracted or updated
    /// 3. Extract the embedded database if necessary
    /// 4. Return a `Database` handle for obtaining connections
    ///
    /// # Errors
    ///
    /// Returns an error if:
    /// - The user data directory cannot be determined
    /// - The database cannot be extracted
    /// - The database migration fails
    pub fn init() -> Result<Self> {
        let path = ensure_database()?;
        Ok(Self { path })
    }

    /// Create a database handle from a specific path.
    ///
    /// This is useful for testing or when you want to use a database
    /// at a specific location rather than the default user data directory.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the SQLite database file
    pub fn from_path(path: PathBuf) -> Self {
        Self { path }
    }

    /// Get a new connection to the database.
    ///
    /// Each call returns a new connection. The connection has foreign keys
    /// disabled by default for compatibility with the repository pattern
    /// (allows inserting test data without worrying about FK order).
    ///
    /// # Errors
    ///
    /// Returns an error if the connection cannot be opened.
    pub fn connection(&self) -> Result<Connection> {
        let conn = Connection::open(&self.path)?;
        conn.execute("PRAGMA foreign_keys = OFF", [])?;
        Ok(conn)
    }

    /// Get a connection with foreign key constraints enabled.
    ///
    /// Use this when you need to ensure referential integrity during
    /// write operations.
    ///
    /// # Errors
    ///
    /// Returns an error if the connection cannot be opened.
    pub fn connection_with_fk(&self) -> Result<Connection> {
        let conn = Connection::open(&self.path)?;
        conn.execute("PRAGMA foreign_keys = ON", [])?;
        Ok(conn)
    }

    /// Returns the path to the database file.
    pub fn path(&self) -> &PathBuf {
        &self.path
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::repositories::AttributeRepository;
    use tempfile::TempDir;

    fn setup_test_db() -> (TempDir, Database) {
        let temp = TempDir::new().unwrap();
        let db_path = temp.path().join("test.db");

        // Extract the embedded database to temp location
        std::fs::write(&db_path, include_bytes!("../../../db/swade.db")).unwrap();

        let db = Database::from_path(db_path);
        (temp, db)
    }

    #[test]
    fn test_database_from_path() {
        let (_temp, db) = setup_test_db();
        assert!(db.path().exists());
    }

    #[test]
    fn test_connection() {
        let (_temp, db) = setup_test_db();
        let conn = db.connection().unwrap();

        // Verify we can query the database
        let result: i64 = conn
            .query_row("SELECT COUNT(*) FROM attributes", [], |row| row.get(0))
            .unwrap();
        assert_eq!(result, 5); // 5 SWADE attributes
    }

    #[test]
    fn test_connection_with_fk() {
        let (_temp, db) = setup_test_db();
        let conn = db.connection_with_fk().unwrap();

        // Verify foreign keys are enabled
        let fk_enabled: i64 = conn
            .query_row("PRAGMA foreign_keys", [], |row| row.get(0))
            .unwrap();
        assert_eq!(fk_enabled, 1);
    }

    #[test]
    fn test_connection_without_fk() {
        let (_temp, db) = setup_test_db();
        let conn = db.connection().unwrap();

        // Verify foreign keys are disabled by default
        let fk_enabled: i64 = conn
            .query_row("PRAGMA foreign_keys", [], |row| row.get(0))
            .unwrap();
        assert_eq!(fk_enabled, 0);
    }

    #[test]
    fn test_multiple_connections() {
        let (_temp, db) = setup_test_db();

        // Should be able to get multiple connections
        let conn1 = db.connection().unwrap();
        let conn2 = db.connection().unwrap();

        // Both should work
        let count1: i64 = conn1
            .query_row("SELECT COUNT(*) FROM skills", [], |row| row.get(0))
            .unwrap();
        let count2: i64 = conn2
            .query_row("SELECT COUNT(*) FROM skills", [], |row| row.get(0))
            .unwrap();

        assert_eq!(count1, count2);
    }

    #[test]
    fn test_connection_with_repository() {
        let (_temp, db) = setup_test_db();
        let conn = db.connection().unwrap();

        // Test that repositories work with the connection
        let attributes = AttributeRepository::get_all(&conn).unwrap();
        assert_eq!(attributes.len(), 5);

        // Verify the attributes are the expected SWADE attributes
        let names: Vec<&str> = attributes.iter().map(|a| a.name.as_str()).collect();
        assert!(names.contains(&"Agility"));
        assert!(names.contains(&"Smarts"));
        assert!(names.contains(&"Spirit"));
        assert!(names.contains(&"Strength"));
        assert!(names.contains(&"Vigor"));
    }

    #[test]
    fn test_database_path() {
        let temp = TempDir::new().unwrap();
        let db_path = temp.path().join("custom.db");
        std::fs::write(&db_path, include_bytes!("../../../db/swade.db")).unwrap();

        let db = Database::from_path(db_path.clone());
        assert_eq!(db.path(), &db_path);
    }
}
