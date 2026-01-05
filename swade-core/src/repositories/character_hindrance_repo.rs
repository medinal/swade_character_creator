use crate::error::Result;
use crate::models::CharacterHindrance;
use rusqlite::{Connection, Row, params};

pub struct CharacterHindranceRepository;

impl CharacterHindranceRepository {
    /// Get a single character hindrance by ID
    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<CharacterHindrance>> {
        let mut stmt = conn.prepare(
            "SELECT id, character_id, hindrance_id, source, created_at, updated_at
             FROM character_hindrances WHERE id = ?",
        )?;

        let mut rows = stmt.query(params![id])?;

        match rows.next()? {
            Some(row) => Ok(Some(Self::row_to_model(row)?)),
            None => Ok(None),
        }
    }

    /// Get all character hindrances
    pub fn get_all(conn: &Connection) -> Result<Vec<CharacterHindrance>> {
        let mut stmt = conn.prepare(
            "SELECT id, character_id, hindrance_id, source, created_at, updated_at
             FROM character_hindrances ORDER BY character_id, id",
        )?;

        let rows = stmt.query_map([], Self::row_to_model)?;

        Ok(rows.filter_map(|r| r.ok()).collect())
    }

    /// Get all character hindrances for a specific character
    pub fn get_by_character_id(
        conn: &Connection,
        character_id: i64,
    ) -> Result<Vec<CharacterHindrance>> {
        let mut stmt = conn.prepare(
            "SELECT id, character_id, hindrance_id, source, created_at, updated_at
             FROM character_hindrances WHERE character_id = ?",
        )?;

        let rows = stmt.query_map(params![character_id], Self::row_to_model)?;

        Ok(rows.filter_map(|r| r.ok()).collect())
    }

    /// Insert a new character hindrance and return the generated ID
    pub fn insert(conn: &Connection, model: &CharacterHindrance) -> Result<i64> {
        conn.execute(
            "INSERT INTO character_hindrances (character_id, hindrance_id, source, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?)",
            params![
                model.character_id,
                model.hindrance_id,
                model.source,
                model.created_at,
                model.updated_at,
            ],
        )?;

        Ok(conn.last_insert_rowid())
    }

    /// Update an existing character hindrance
    pub fn update(conn: &Connection, model: &CharacterHindrance) -> Result<()> {
        conn.execute(
            "UPDATE character_hindrances
             SET character_id = ?, hindrance_id = ?, source = ?, updated_at = ?
             WHERE id = ?",
            params![
                model.character_id,
                model.hindrance_id,
                model.source,
                model.updated_at,
                model.id,
            ],
        )?;

        Ok(())
    }

    /// Delete a character hindrance by ID
    pub fn delete(conn: &Connection, id: i64) -> Result<()> {
        conn.execute("DELETE FROM character_hindrances WHERE id = ?", params![id])?;

        Ok(())
    }

    /// Delete all hindrances for a character with a specific source
    pub fn delete_by_character_id_and_source(
        conn: &Connection,
        character_id: i64,
        source: &str,
    ) -> Result<()> {
        conn.execute(
            "DELETE FROM character_hindrances WHERE character_id = ? AND source = ?",
            params![character_id, source],
        )?;
        Ok(())
    }

    /// Convert a database row to a CharacterHindrance model
    fn row_to_model(row: &Row) -> rusqlite::Result<CharacterHindrance> {
        Ok(CharacterHindrance {
            id: row.get(0)?,
            character_id: row.get(1)?,
            hindrance_id: row.get(2)?,
            source: row.get(3)?,
            created_at: row.get(4)?,
            updated_at: row.get(5)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;

    fn setup_test_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        let schema = include_str!("../../../db/schema.sql");
        conn.execute_batch(schema).unwrap();

        // Insert required rank for foreign key
        conn.execute(
            "INSERT INTO ranks (id, name, min_advances, max_advances, description,
                               created_at, updated_at)
             VALUES (1, 'Novice', 0, 3, 'Starting rank',
                     '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            [],
        )
        .unwrap();

        conn
    }

    fn insert_test_character(conn: &Connection, id: i64, name: &str) {
        conn.execute(
            "INSERT INTO characters (id, name, created_at, updated_at)
             VALUES (?, ?, '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            params![id, name],
        )
        .unwrap();
    }

    fn insert_test_hindrance(conn: &Connection, id: i64, name: &str) {
        conn.execute(
            "INSERT INTO hindrances (id, name, severity, point_value, source, description,
                                    created_at, updated_at)
             VALUES (?, ?, 'major', 2, 'core', 'Test description',
                     '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            params![id, name],
        )
        .unwrap();
    }

    fn insert_test_character_hindrance(
        conn: &Connection,
        id: i64,
        character_id: i64,
        hindrance_id: i64,
        source: &str,
    ) {
        conn.execute(
            "INSERT INTO character_hindrances (id, character_id, hindrance_id, source,
                                              created_at, updated_at)
             VALUES (?, ?, ?, ?, '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            params![id, character_id, hindrance_id, source],
        )
        .unwrap();
    }

    #[test]
    fn test_get_by_id_found() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_hindrance(&conn, 1, "Bad Luck");
        insert_test_character_hindrance(&conn, 1, 1, 1, "chosen");

        let result = CharacterHindranceRepository::get_by_id(&conn, 1).unwrap();

        assert!(result.is_some());
        let character_hindrance = result.unwrap();
        assert_eq!(character_hindrance.id, 1);
        assert_eq!(character_hindrance.character_id, 1);
        assert_eq!(character_hindrance.hindrance_id, 1);
        assert_eq!(character_hindrance.source, "chosen");
        assert_eq!(character_hindrance.created_at, "2024-01-01 00:00:00");
        assert_eq!(character_hindrance.updated_at, "2024-01-01 00:00:00");
    }

    #[test]
    fn test_get_by_id_not_found() {
        let conn = setup_test_db();

        let result = CharacterHindranceRepository::get_by_id(&conn, 999).unwrap();

        assert!(result.is_none());
    }

    #[test]
    fn test_get_all() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Character One");
        insert_test_character(&conn, 2, "Character Two");
        insert_test_hindrance(&conn, 1, "Bad Luck");
        insert_test_hindrance(&conn, 2, "Heroic");

        insert_test_character_hindrance(&conn, 1, 1, 1, "chosen");
        insert_test_character_hindrance(&conn, 2, 2, 2, "chosen");
        insert_test_character_hindrance(&conn, 3, 1, 2, "ancestry");

        let hindrances = CharacterHindranceRepository::get_all(&conn).unwrap();

        assert_eq!(hindrances.len(), 3);
    }

    #[test]
    fn test_get_all_empty() {
        let conn = setup_test_db();

        let hindrances = CharacterHindranceRepository::get_all(&conn).unwrap();

        assert_eq!(hindrances.len(), 0);
    }

    #[test]
    fn test_get_by_character_id_multiple_hindrances() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_hindrance(&conn, 1, "Bad Luck");
        insert_test_hindrance(&conn, 2, "Heroic");

        insert_test_character_hindrance(&conn, 1, 1, 1, "chosen");
        insert_test_character_hindrance(&conn, 2, 1, 2, "chosen");

        let hindrances = CharacterHindranceRepository::get_by_character_id(&conn, 1).unwrap();

        assert_eq!(hindrances.len(), 2);
        assert_eq!(hindrances[0].character_id, 1);
        assert_eq!(hindrances[0].hindrance_id, 1);
        assert_eq!(hindrances[1].character_id, 1);
        assert_eq!(hindrances[1].hindrance_id, 2);
    }

    #[test]
    fn test_get_by_character_id_empty() {
        let conn = setup_test_db();

        let hindrances = CharacterHindranceRepository::get_by_character_id(&conn, 999).unwrap();

        assert_eq!(hindrances.len(), 0);
    }

    #[test]
    fn test_get_by_character_id_filters_correctly() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Character One");
        insert_test_character(&conn, 2, "Character Two");
        insert_test_hindrance(&conn, 1, "Bad Luck");

        insert_test_character_hindrance(&conn, 1, 1, 1, "chosen");
        insert_test_character_hindrance(&conn, 2, 2, 1, "chosen");

        let hindrances = CharacterHindranceRepository::get_by_character_id(&conn, 1).unwrap();

        assert_eq!(hindrances.len(), 1);
        assert_eq!(hindrances[0].character_id, 1);
    }

    #[test]
    fn test_get_by_character_id_different_sources() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_hindrance(&conn, 1, "Bad Luck");
        insert_test_hindrance(&conn, 2, "Heroic");

        insert_test_character_hindrance(&conn, 1, 1, 1, "chosen");
        insert_test_character_hindrance(&conn, 2, 1, 2, "ancestry");

        let hindrances = CharacterHindranceRepository::get_by_character_id(&conn, 1).unwrap();

        assert_eq!(hindrances.len(), 2);
        assert_eq!(hindrances[0].source, "chosen");
        assert_eq!(hindrances[1].source, "ancestry");
    }

    #[test]
    fn test_insert() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_hindrance(&conn, 1, "Bad Luck");

        let character_hindrance = CharacterHindrance {
            id: 0, // Will be ignored
            character_id: 1,
            hindrance_id: 1,
            source: "chosen".to_string(),
            created_at: "2024-01-01 00:00:00".to_string(),
            updated_at: "2024-01-01 00:00:00".to_string(),
        };

        let id = CharacterHindranceRepository::insert(&conn, &character_hindrance).unwrap();

        assert!(id > 0);

        let retrieved = CharacterHindranceRepository::get_by_id(&conn, id).unwrap();
        assert!(retrieved.is_some());
        let retrieved = retrieved.unwrap();
        assert_eq!(retrieved.character_id, 1);
        assert_eq!(retrieved.hindrance_id, 1);
        assert_eq!(retrieved.source, "chosen");
    }

    #[test]
    fn test_update() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_hindrance(&conn, 1, "Bad Luck");
        insert_test_hindrance(&conn, 2, "Heroic");
        insert_test_character_hindrance(&conn, 1, 1, 1, "chosen");

        let mut character_hindrance = CharacterHindranceRepository::get_by_id(&conn, 1)
            .unwrap()
            .unwrap();

        character_hindrance.hindrance_id = 2;
        character_hindrance.source = "ancestry".to_string();
        character_hindrance.updated_at = "2024-12-25 10:30:00".to_string();

        CharacterHindranceRepository::update(&conn, &character_hindrance).unwrap();

        let retrieved = CharacterHindranceRepository::get_by_id(&conn, 1)
            .unwrap()
            .unwrap();
        assert_eq!(retrieved.hindrance_id, 2);
        assert_eq!(retrieved.source, "ancestry");
        assert_eq!(retrieved.updated_at, "2024-12-25 10:30:00");
    }

    #[test]
    fn test_delete() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_hindrance(&conn, 1, "Bad Luck");
        insert_test_character_hindrance(&conn, 1, 1, 1, "chosen");

        let exists_before = CharacterHindranceRepository::get_by_id(&conn, 1).unwrap();
        assert!(exists_before.is_some());

        CharacterHindranceRepository::delete(&conn, 1).unwrap();

        let exists_after = CharacterHindranceRepository::get_by_id(&conn, 1).unwrap();
        assert!(exists_after.is_none());
    }

    #[test]
    fn test_delete_nonexistent() {
        let conn = setup_test_db();

        // Deleting a non-existent record should not error
        let result = CharacterHindranceRepository::delete(&conn, 999);
        assert!(result.is_ok());
    }

    #[test]
    fn test_row_to_model_all_fields() {
        let conn = setup_test_db();
        insert_test_character(&conn, 42, "Test Character");
        insert_test_hindrance(&conn, 99, "Test Hindrance");

        conn.execute(
            "INSERT INTO character_hindrances (id, character_id, hindrance_id, source,
                                              created_at, updated_at)
             VALUES (100, 42, 99, 'ancestry', '2024-12-25 10:30:00', '2024-12-26 15:45:00')",
            [],
        )
        .unwrap();

        let character_hindrance = CharacterHindranceRepository::get_by_id(&conn, 100)
            .unwrap()
            .unwrap();

        assert_eq!(character_hindrance.id, 100);
        assert_eq!(character_hindrance.character_id, 42);
        assert_eq!(character_hindrance.hindrance_id, 99);
        assert_eq!(character_hindrance.source, "ancestry");
        assert_eq!(character_hindrance.created_at, "2024-12-25 10:30:00");
        assert_eq!(character_hindrance.updated_at, "2024-12-26 15:45:00");
    }

    #[test]
    fn test_unique_constraint_character_hindrance_pair() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_hindrance(&conn, 1, "Bad Luck");
        insert_test_character_hindrance(&conn, 1, 1, 1, "chosen");

        // Attempting to insert duplicate character_id/hindrance_id pair should fail
        let result = conn.execute(
            "INSERT INTO character_hindrances (character_id, hindrance_id, source,
                                              created_at, updated_at)
             VALUES (1, 1, 'chosen', '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            [],
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_insert_with_ancestry_source() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_hindrance(&conn, 1, "Bad Luck");

        let character_hindrance = CharacterHindrance {
            id: 0,
            character_id: 1,
            hindrance_id: 1,
            source: "ancestry".to_string(),
            created_at: "2024-01-01 00:00:00".to_string(),
            updated_at: "2024-01-01 00:00:00".to_string(),
        };

        let id = CharacterHindranceRepository::insert(&conn, &character_hindrance).unwrap();
        let retrieved = CharacterHindranceRepository::get_by_id(&conn, id)
            .unwrap()
            .unwrap();

        assert_eq!(retrieved.source, "ancestry");
    }
}
