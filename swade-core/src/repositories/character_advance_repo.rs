use crate::error::Result;
use crate::models::CharacterAdvance;
use rusqlite::{Connection, Row, params};

pub struct CharacterAdvanceRepository;

impl CharacterAdvanceRepository {
    /// Get a single character advance by ID
    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<CharacterAdvance>> {
        let mut stmt = conn.prepare(
            "SELECT id, character_id, advance_number, advance_type,
                    edge_id, attribute_id, skill_id_1, skill_id_2,
                    hindrance_id, hindrance_action, notes,
                    created_at, updated_at
             FROM character_advances WHERE id = ?",
        )?;

        let mut rows = stmt.query(params![id])?;

        match rows.next()? {
            Some(row) => Ok(Some(Self::row_to_model(row)?)),
            None => Ok(None),
        }
    }

    /// Get all advances for a specific character, ordered by advance number
    pub fn get_by_character_id(conn: &Connection, character_id: i64) -> Result<Vec<CharacterAdvance>> {
        let mut stmt = conn.prepare(
            "SELECT id, character_id, advance_number, advance_type,
                    edge_id, attribute_id, skill_id_1, skill_id_2,
                    hindrance_id, hindrance_action, notes,
                    created_at, updated_at
             FROM character_advances WHERE character_id = ? ORDER BY advance_number",
        )?;

        let rows = stmt.query_map(params![character_id], Self::row_to_model)?;

        Ok(rows.filter_map(|r| r.ok()).collect())
    }

    /// Get a specific advance by character and advance number
    pub fn get_by_advance_number(
        conn: &Connection,
        character_id: i64,
        advance_number: i64,
    ) -> Result<Option<CharacterAdvance>> {
        let mut stmt = conn.prepare(
            "SELECT id, character_id, advance_number, advance_type,
                    edge_id, attribute_id, skill_id_1, skill_id_2,
                    hindrance_id, hindrance_action, notes,
                    created_at, updated_at
             FROM character_advances WHERE character_id = ? AND advance_number = ?",
        )?;

        let mut rows = stmt.query(params![character_id, advance_number])?;

        match rows.next()? {
            Some(row) => Ok(Some(Self::row_to_model(row)?)),
            None => Ok(None),
        }
    }

    /// Count total advances for a character
    pub fn count_by_character_id(conn: &Connection, character_id: i64) -> Result<i64> {
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM character_advances WHERE character_id = ?",
            params![character_id],
            |row| row.get(0),
        )?;
        Ok(count)
    }

    /// Count attribute advances within a rank's advance range
    pub fn count_attribute_advances_in_range(
        conn: &Connection,
        character_id: i64,
        min_advance: i64,
        max_advance: i64,
    ) -> Result<i64> {
        let count: i64 = conn.query_row(
            "SELECT COUNT(*) FROM character_advances
             WHERE character_id = ? AND advance_type = 'attribute'
             AND advance_number >= ? AND advance_number <= ?",
            params![character_id, min_advance, max_advance],
            |row| row.get(0),
        )?;
        Ok(count)
    }

    /// Find any banked hindrance removal (single remove_major_half without completion)
    pub fn get_banked_hindrance_removal(
        conn: &Connection,
        character_id: i64,
        hindrance_id: i64,
    ) -> Result<Option<CharacterAdvance>> {
        // A banked hindrance removal is a remove_major_half that doesn't have a matching second one
        let mut stmt = conn.prepare(
            "SELECT id, character_id, advance_number, advance_type,
                    edge_id, attribute_id, skill_id_1, skill_id_2,
                    hindrance_id, hindrance_action, notes,
                    created_at, updated_at
             FROM character_advances
             WHERE character_id = ? AND hindrance_id = ? AND hindrance_action = 'remove_major_half'
             ORDER BY advance_number DESC
             LIMIT 1",
        )?;

        let mut rows = stmt.query(params![character_id, hindrance_id])?;

        match rows.next()? {
            Some(row) => {
                let advance = Self::row_to_model(row)?;
                // Check if there's a second remove_major_half for this hindrance
                let count: i64 = conn.query_row(
                    "SELECT COUNT(*) FROM character_advances
                     WHERE character_id = ? AND hindrance_id = ? AND hindrance_action = 'remove_major_half'",
                    params![character_id, hindrance_id],
                    |row| row.get(0),
                )?;
                // If count is odd (1), there's a banked advance
                if count % 2 == 1 {
                    Ok(Some(advance))
                } else {
                    Ok(None)
                }
            }
            None => Ok(None),
        }
    }

    /// Insert a new character advance, returns the generated ID
    pub fn insert(conn: &Connection, model: &CharacterAdvance) -> Result<i64> {
        conn.execute(
            "INSERT INTO character_advances (
                character_id, advance_number, advance_type,
                edge_id, attribute_id, skill_id_1, skill_id_2,
                hindrance_id, hindrance_action, notes
             ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
            params![
                model.character_id,
                model.advance_number,
                model.advance_type,
                model.edge_id,
                model.attribute_id,
                model.skill_id_1,
                model.skill_id_2,
                model.hindrance_id,
                model.hindrance_action,
                model.notes,
            ],
        )?;

        Ok(conn.last_insert_rowid())
    }

    /// Delete a character advance by ID (for undo functionality)
    pub fn delete(conn: &Connection, id: i64) -> Result<()> {
        conn.execute("DELETE FROM character_advances WHERE id = ?", params![id])?;
        Ok(())
    }

    /// Delete the most recent advance for a character (for undo functionality)
    pub fn delete_latest(conn: &Connection, character_id: i64) -> Result<bool> {
        let affected = conn.execute(
            "DELETE FROM character_advances
             WHERE character_id = ? AND advance_number = (
                 SELECT MAX(advance_number) FROM character_advances WHERE character_id = ?
             )",
            params![character_id, character_id],
        )?;
        Ok(affected > 0)
    }

    /// Convert a database row to a CharacterAdvance model
    fn row_to_model(row: &Row) -> rusqlite::Result<CharacterAdvance> {
        Ok(CharacterAdvance {
            id: row.get(0)?,
            character_id: row.get(1)?,
            advance_number: row.get(2)?,
            advance_type: row.get(3)?,
            edge_id: row.get(4)?,
            attribute_id: row.get(5)?,
            skill_id_1: row.get(6)?,
            skill_id_2: row.get(7)?,
            hindrance_id: row.get(8)?,
            hindrance_action: row.get(9)?,
            notes: row.get(10)?,
            created_at: row.get(11)?,
            updated_at: row.get(12)?,
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

        // Disable foreign key constraints for tests
        conn.execute_batch("PRAGMA foreign_keys = OFF;").unwrap();

        conn
    }

    fn insert_test_character(conn: &Connection, id: i64, name: &str) {
        conn.execute(
            "INSERT INTO characters (id, name, is_wild_card, created_at, updated_at)
             VALUES (?, ?, 1, '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            params![id, name],
        )
        .unwrap();
    }

    fn insert_test_advance(
        conn: &Connection,
        id: i64,
        character_id: i64,
        advance_number: i64,
        advance_type: &str,
    ) -> i64 {
        conn.execute(
            "INSERT INTO character_advances (id, character_id, advance_number, advance_type,
                                            created_at, updated_at)
             VALUES (?, ?, ?, ?, '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            params![id, character_id, advance_number, advance_type],
        )
        .unwrap();
        id
    }

    #[test]
    fn test_get_by_id_found() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_advance(&conn, 1, 1, 1, "attribute");

        let result = CharacterAdvanceRepository::get_by_id(&conn, 1).unwrap();

        assert!(result.is_some());
        let advance = result.unwrap();
        assert_eq!(advance.id, 1);
        assert_eq!(advance.character_id, 1);
        assert_eq!(advance.advance_number, 1);
        assert_eq!(advance.advance_type, "attribute");
    }

    #[test]
    fn test_get_by_id_not_found() {
        let conn = setup_test_db();

        let result = CharacterAdvanceRepository::get_by_id(&conn, 999).unwrap();

        assert!(result.is_none());
    }

    #[test]
    fn test_get_by_character_id_empty() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");

        let advances = CharacterAdvanceRepository::get_by_character_id(&conn, 1).unwrap();

        assert_eq!(advances.len(), 0);
    }

    #[test]
    fn test_get_by_character_id_multiple() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_advance(&conn, 1, 1, 1, "attribute");
        insert_test_advance(&conn, 2, 1, 2, "edge");
        insert_test_advance(&conn, 3, 1, 3, "skill_cheap");

        let advances = CharacterAdvanceRepository::get_by_character_id(&conn, 1).unwrap();

        assert_eq!(advances.len(), 3);
        assert_eq!(advances[0].advance_number, 1);
        assert_eq!(advances[1].advance_number, 2);
        assert_eq!(advances[2].advance_number, 3);
    }

    #[test]
    fn test_get_by_advance_number_found() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_advance(&conn, 1, 1, 1, "attribute");
        insert_test_advance(&conn, 2, 1, 2, "edge");

        let result = CharacterAdvanceRepository::get_by_advance_number(&conn, 1, 2).unwrap();

        assert!(result.is_some());
        let advance = result.unwrap();
        assert_eq!(advance.advance_number, 2);
        assert_eq!(advance.advance_type, "edge");
    }

    #[test]
    fn test_get_by_advance_number_not_found() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_advance(&conn, 1, 1, 1, "attribute");

        let result = CharacterAdvanceRepository::get_by_advance_number(&conn, 1, 99).unwrap();

        assert!(result.is_none());
    }

    #[test]
    fn test_count_by_character_id() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_advance(&conn, 1, 1, 1, "attribute");
        insert_test_advance(&conn, 2, 1, 2, "edge");
        insert_test_advance(&conn, 3, 1, 3, "skill_cheap");

        let count = CharacterAdvanceRepository::count_by_character_id(&conn, 1).unwrap();

        assert_eq!(count, 3);
    }

    #[test]
    fn test_count_by_character_id_empty() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");

        let count = CharacterAdvanceRepository::count_by_character_id(&conn, 1).unwrap();

        assert_eq!(count, 0);
    }

    #[test]
    fn test_count_attribute_advances_in_range() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        // Novice (1-4)
        insert_test_advance(&conn, 1, 1, 1, "attribute");
        insert_test_advance(&conn, 2, 1, 2, "edge");
        insert_test_advance(&conn, 3, 1, 3, "skill_cheap");
        insert_test_advance(&conn, 4, 1, 4, "edge");
        // Seasoned (5-8)
        insert_test_advance(&conn, 5, 1, 5, "attribute");
        insert_test_advance(&conn, 6, 1, 6, "edge");

        // Count attribute advances in Novice range
        let count = CharacterAdvanceRepository::count_attribute_advances_in_range(&conn, 1, 1, 4).unwrap();
        assert_eq!(count, 1);

        // Count attribute advances in Seasoned range
        let count = CharacterAdvanceRepository::count_attribute_advances_in_range(&conn, 1, 5, 8).unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_insert() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");

        let advance = CharacterAdvance {
            id: 0,
            character_id: 1,
            advance_number: 1,
            advance_type: "edge".to_string(),
            edge_id: Some(5),
            attribute_id: None,
            skill_id_1: None,
            skill_id_2: None,
            hindrance_id: None,
            hindrance_action: None,
            notes: Some("Took Alertness".to_string()),
            created_at: String::new(),
            updated_at: String::new(),
        };

        let id = CharacterAdvanceRepository::insert(&conn, &advance).unwrap();

        assert!(id > 0);
        let retrieved = CharacterAdvanceRepository::get_by_id(&conn, id).unwrap().unwrap();
        assert_eq!(retrieved.character_id, 1);
        assert_eq!(retrieved.advance_number, 1);
        assert_eq!(retrieved.advance_type, "edge");
        assert_eq!(retrieved.edge_id, Some(5));
        assert_eq!(retrieved.notes, Some("Took Alertness".to_string()));
    }

    #[test]
    fn test_delete() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_advance(&conn, 1, 1, 1, "attribute");

        CharacterAdvanceRepository::delete(&conn, 1).unwrap();

        let result = CharacterAdvanceRepository::get_by_id(&conn, 1).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_delete_latest() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_advance(&conn, 1, 1, 1, "attribute");
        insert_test_advance(&conn, 2, 1, 2, "edge");
        insert_test_advance(&conn, 3, 1, 3, "skill_cheap");

        let deleted = CharacterAdvanceRepository::delete_latest(&conn, 1).unwrap();

        assert!(deleted);
        let advances = CharacterAdvanceRepository::get_by_character_id(&conn, 1).unwrap();
        assert_eq!(advances.len(), 2);
        assert_eq!(advances[0].advance_number, 1);
        assert_eq!(advances[1].advance_number, 2);
    }

    #[test]
    fn test_delete_latest_empty() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");

        let deleted = CharacterAdvanceRepository::delete_latest(&conn, 1).unwrap();

        assert!(!deleted);
    }

    #[test]
    fn test_banked_hindrance_removal_none() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");

        let result = CharacterAdvanceRepository::get_banked_hindrance_removal(&conn, 1, 5).unwrap();

        assert!(result.is_none());
    }

    #[test]
    fn test_banked_hindrance_removal_single() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        conn.execute(
            "INSERT INTO character_advances (id, character_id, advance_number, advance_type,
                                            hindrance_id, hindrance_action, created_at, updated_at)
             VALUES (1, 1, 1, 'hindrance', 5, 'remove_major_half', '2024-01-01', '2024-01-01')",
            [],
        ).unwrap();

        let result = CharacterAdvanceRepository::get_banked_hindrance_removal(&conn, 1, 5).unwrap();

        assert!(result.is_some());
        let advance = result.unwrap();
        assert_eq!(advance.hindrance_id, Some(5));
        assert_eq!(advance.hindrance_action, Some("remove_major_half".to_string()));
    }

    #[test]
    fn test_banked_hindrance_removal_completed() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        // Two remove_major_half entries means completed, not banked
        conn.execute(
            "INSERT INTO character_advances (id, character_id, advance_number, advance_type,
                                            hindrance_id, hindrance_action, created_at, updated_at)
             VALUES (1, 1, 1, 'hindrance', 5, 'remove_major_half', '2024-01-01', '2024-01-01')",
            [],
        ).unwrap();
        conn.execute(
            "INSERT INTO character_advances (id, character_id, advance_number, advance_type,
                                            hindrance_id, hindrance_action, created_at, updated_at)
             VALUES (2, 1, 2, 'hindrance', 5, 'remove_major_half', '2024-01-01', '2024-01-01')",
            [],
        ).unwrap();

        let result = CharacterAdvanceRepository::get_banked_hindrance_removal(&conn, 1, 5).unwrap();

        assert!(result.is_none());
    }
}
