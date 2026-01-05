use crate::error::Result;
use crate::models::CharacterModifier;
use rusqlite::{Connection, Row, params};

pub struct CharacterModifierRepository;

impl CharacterModifierRepository {
    /// Get a single character modifier by ID
    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<CharacterModifier>> {
        let mut stmt = conn.prepare(
            "SELECT id, character_id, modifier_id, advance_taken, created_at, updated_at
             FROM character_modifiers WHERE id = ?",
        )?;

        let mut rows = stmt.query(params![id])?;

        match rows.next()? {
            Some(row) => Ok(Some(Self::row_to_model(row)?)),
            None => Ok(None),
        }
    }

    /// Get all character modifiers for a specific character
    pub fn get_by_character_id(
        conn: &Connection,
        character_id: i64,
    ) -> Result<Vec<CharacterModifier>> {
        let mut stmt = conn.prepare(
            "SELECT id, character_id, modifier_id, advance_taken, created_at, updated_at
             FROM character_modifiers WHERE character_id = ?",
        )?;

        let rows = stmt.query_map(params![character_id], Self::row_to_model)?;

        Ok(rows.filter_map(|r| r.ok()).collect())
    }

    /// Insert a new character modifier, return generated ID
    pub fn insert(conn: &Connection, model: &CharacterModifier) -> Result<i64> {
        conn.execute(
            "INSERT INTO character_modifiers (character_id, modifier_id, advance_taken, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?)",
            params![
                model.character_id,
                model.modifier_id,
                model.advance_taken,
                model.created_at,
                model.updated_at,
            ],
        )?;

        Ok(conn.last_insert_rowid())
    }

    /// Update an existing character modifier
    pub fn update(conn: &Connection, model: &CharacterModifier) -> Result<()> {
        conn.execute(
            "UPDATE character_modifiers
             SET character_id = ?, modifier_id = ?, advance_taken = ?, updated_at = ?
             WHERE id = ?",
            params![
                model.character_id,
                model.modifier_id,
                model.advance_taken,
                model.updated_at,
                model.id,
            ],
        )?;

        Ok(())
    }

    /// Delete a character modifier by ID
    pub fn delete(conn: &Connection, id: i64) -> Result<()> {
        conn.execute("DELETE FROM character_modifiers WHERE id = ?", params![id])?;

        Ok(())
    }

    /// Convert a database row to a CharacterModifier model
    fn row_to_model(row: &Row) -> rusqlite::Result<CharacterModifier> {
        Ok(CharacterModifier {
            id: row.get(0)?,
            character_id: row.get(1)?,
            modifier_id: row.get(2)?,
            advance_taken: row.get(3)?,
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

        // Disable foreign key constraints for tests to avoid needing seed data
        conn.execute_batch("PRAGMA foreign_keys = OFF;").unwrap();

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

    fn insert_test_modifier(conn: &Connection, id: i64, description: &str) {
        conn.execute(
            "INSERT INTO modifiers (id, target_type, target_identifier, value_type,
                                   value, description, created_at, updated_at)
             VALUES (?, 'attribute', 'agility', 'die_increment',
                     1, ?, '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            params![id, description],
        )
        .unwrap();
    }

    fn insert_test_character_modifier(
        conn: &Connection,
        id: i64,
        character_id: i64,
        modifier_id: i64,
        advance_taken: Option<i64>,
    ) {
        conn.execute(
            "INSERT INTO character_modifiers (id, character_id, modifier_id, advance_taken, created_at, updated_at)
             VALUES (?, ?, ?, ?, '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            params![id, character_id, modifier_id, advance_taken],
        ).unwrap();
    }

    #[test]
    fn test_get_by_id_found() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_modifier(&conn, 1, "Agility Bonus");
        insert_test_character_modifier(&conn, 1, 1, 1, None);

        let result = CharacterModifierRepository::get_by_id(&conn, 1).unwrap();

        assert!(result.is_some());
        let character_modifier = result.unwrap();
        assert_eq!(character_modifier.id, 1);
        assert_eq!(character_modifier.character_id, 1);
        assert_eq!(character_modifier.modifier_id, 1);
        assert_eq!(character_modifier.advance_taken, None);
        assert_eq!(character_modifier.created_at, "2024-01-01 00:00:00");
        assert_eq!(character_modifier.updated_at, "2024-01-01 00:00:00");
    }

    #[test]
    fn test_get_by_id_not_found() {
        let conn = setup_test_db();

        let result = CharacterModifierRepository::get_by_id(&conn, 999).unwrap();

        assert!(result.is_none());
    }

    #[test]
    fn test_get_by_character_id_multiple_modifiers() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_modifier(&conn, 1, "Agility Bonus");
        insert_test_modifier(&conn, 2, "Strength Bonus");

        insert_test_character_modifier(&conn, 1, 1, 1, None);
        insert_test_character_modifier(&conn, 2, 1, 2, Some(1));

        let modifiers = CharacterModifierRepository::get_by_character_id(&conn, 1).unwrap();

        assert_eq!(modifiers.len(), 2);
        assert_eq!(modifiers[0].character_id, 1);
        assert_eq!(modifiers[0].modifier_id, 1);
        assert_eq!(modifiers[0].advance_taken, None);
        assert_eq!(modifiers[1].character_id, 1);
        assert_eq!(modifiers[1].modifier_id, 2);
        assert_eq!(modifiers[1].advance_taken, Some(1));
    }

    #[test]
    fn test_get_by_character_id_empty() {
        let conn = setup_test_db();

        let modifiers = CharacterModifierRepository::get_by_character_id(&conn, 999).unwrap();

        assert_eq!(modifiers.len(), 0);
    }

    #[test]
    fn test_get_by_character_id_filters_correctly() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Character One");
        insert_test_character(&conn, 2, "Character Two");
        insert_test_modifier(&conn, 1, "Test Modifier");

        insert_test_character_modifier(&conn, 1, 1, 1, None);
        insert_test_character_modifier(&conn, 2, 2, 1, None);

        let modifiers = CharacterModifierRepository::get_by_character_id(&conn, 1).unwrap();

        assert_eq!(modifiers.len(), 1);
        assert_eq!(modifiers[0].character_id, 1);
    }

    #[test]
    fn test_insert_without_advance() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_modifier(&conn, 1, "Test Modifier");

        let new_modifier = CharacterModifier {
            id: 0, // Will be auto-generated
            character_id: 1,
            modifier_id: 1,
            advance_taken: None,
            created_at: "2024-01-01 00:00:00".to_string(),
            updated_at: "2024-01-01 00:00:00".to_string(),
        };

        let id = CharacterModifierRepository::insert(&conn, &new_modifier).unwrap();

        assert!(id > 0);

        let retrieved = CharacterModifierRepository::get_by_id(&conn, id)
            .unwrap()
            .unwrap();
        assert_eq!(retrieved.character_id, 1);
        assert_eq!(retrieved.modifier_id, 1);
        assert_eq!(retrieved.advance_taken, None);
    }

    #[test]
    fn test_insert_with_advance() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_modifier(&conn, 1, "Test Modifier");

        let new_modifier = CharacterModifier {
            id: 0,
            character_id: 1,
            modifier_id: 1,
            advance_taken: Some(3),
            created_at: "2024-01-01 00:00:00".to_string(),
            updated_at: "2024-01-01 00:00:00".to_string(),
        };

        let id = CharacterModifierRepository::insert(&conn, &new_modifier).unwrap();

        let retrieved = CharacterModifierRepository::get_by_id(&conn, id)
            .unwrap()
            .unwrap();
        assert_eq!(retrieved.advance_taken, Some(3));
    }

    #[test]
    fn test_update() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_modifier(&conn, 1, "Original Modifier");
        insert_test_modifier(&conn, 2, "Updated Modifier");
        insert_test_character_modifier(&conn, 1, 1, 1, None);

        let mut updated_modifier = CharacterModifierRepository::get_by_id(&conn, 1)
            .unwrap()
            .unwrap();
        updated_modifier.modifier_id = 2;
        updated_modifier.advance_taken = Some(5);
        updated_modifier.updated_at = "2024-12-01 00:00:00".to_string();

        CharacterModifierRepository::update(&conn, &updated_modifier).unwrap();

        let retrieved = CharacterModifierRepository::get_by_id(&conn, 1)
            .unwrap()
            .unwrap();
        assert_eq!(retrieved.modifier_id, 2);
        assert_eq!(retrieved.advance_taken, Some(5));
        assert_eq!(retrieved.updated_at, "2024-12-01 00:00:00");
    }

    #[test]
    fn test_delete() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_modifier(&conn, 1, "Test Modifier");
        insert_test_character_modifier(&conn, 1, 1, 1, None);

        CharacterModifierRepository::delete(&conn, 1).unwrap();

        let result = CharacterModifierRepository::get_by_id(&conn, 1).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_row_to_model_all_fields() {
        let conn = setup_test_db();
        insert_test_character(&conn, 42, "Test Character");
        insert_test_modifier(&conn, 99, "Test Modifier");

        conn.execute(
            "INSERT INTO character_modifiers (id, character_id, modifier_id, advance_taken, created_at, updated_at)
             VALUES (100, 42, 99, 7, '2024-12-25 10:30:00', '2024-12-26 15:45:00')",
            [],
        ).unwrap();

        let character_modifier = CharacterModifierRepository::get_by_id(&conn, 100)
            .unwrap()
            .unwrap();

        assert_eq!(character_modifier.id, 100);
        assert_eq!(character_modifier.character_id, 42);
        assert_eq!(character_modifier.modifier_id, 99);
        assert_eq!(character_modifier.advance_taken, Some(7));
        assert_eq!(character_modifier.created_at, "2024-12-25 10:30:00");
        assert_eq!(character_modifier.updated_at, "2024-12-26 15:45:00");
    }

    #[test]
    fn test_unique_constraint_character_modifier_pair() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_modifier(&conn, 1, "Test Modifier");
        insert_test_character_modifier(&conn, 1, 1, 1, None);

        // Attempting to insert duplicate character_id/modifier_id pair should fail
        let result = conn.execute(
            "INSERT INTO character_modifiers (character_id, modifier_id, created_at, updated_at)
             VALUES (1, 1, '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            [],
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_get_by_character_id_with_single_modifier() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_modifier(&conn, 1, "Test Modifier");
        insert_test_character_modifier(&conn, 1, 1, 1, Some(2));

        let modifiers = CharacterModifierRepository::get_by_character_id(&conn, 1).unwrap();

        assert_eq!(modifiers.len(), 1);
        assert_eq!(modifiers[0].character_id, 1);
        assert_eq!(modifiers[0].modifier_id, 1);
        assert_eq!(modifiers[0].advance_taken, Some(2));
    }

    #[test]
    fn test_delete_nonexistent() {
        let conn = setup_test_db();

        // Deleting a non-existent record should succeed without error
        let result = CharacterModifierRepository::delete(&conn, 999);

        assert!(result.is_ok());
    }

    #[test]
    fn test_update_changes_multiple_fields() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_character(&conn, 2, "Another Character");
        insert_test_modifier(&conn, 1, "Modifier 1");
        insert_test_modifier(&conn, 2, "Modifier 2");
        insert_test_character_modifier(&conn, 1, 1, 1, None);

        let mut updated = CharacterModifierRepository::get_by_id(&conn, 1)
            .unwrap()
            .unwrap();
        updated.character_id = 2;
        updated.modifier_id = 2;
        updated.advance_taken = Some(10);
        updated.updated_at = "2025-01-01 00:00:00".to_string();

        CharacterModifierRepository::update(&conn, &updated).unwrap();

        let retrieved = CharacterModifierRepository::get_by_id(&conn, 1)
            .unwrap()
            .unwrap();
        assert_eq!(retrieved.character_id, 2);
        assert_eq!(retrieved.modifier_id, 2);
        assert_eq!(retrieved.advance_taken, Some(10));
        assert_eq!(retrieved.updated_at, "2025-01-01 00:00:00");
    }

    #[test]
    fn test_insert_multiple_modifiers_same_character() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_modifier(&conn, 1, "Modifier 1");
        insert_test_modifier(&conn, 2, "Modifier 2");
        insert_test_modifier(&conn, 3, "Modifier 3");

        let modifier1 = CharacterModifier {
            id: 0,
            character_id: 1,
            modifier_id: 1,
            advance_taken: None,
            created_at: "2024-01-01 00:00:00".to_string(),
            updated_at: "2024-01-01 00:00:00".to_string(),
        };

        let modifier2 = CharacterModifier {
            id: 0,
            character_id: 1,
            modifier_id: 2,
            advance_taken: Some(1),
            created_at: "2024-01-01 00:00:00".to_string(),
            updated_at: "2024-01-01 00:00:00".to_string(),
        };

        let modifier3 = CharacterModifier {
            id: 0,
            character_id: 1,
            modifier_id: 3,
            advance_taken: Some(2),
            created_at: "2024-01-01 00:00:00".to_string(),
            updated_at: "2024-01-01 00:00:00".to_string(),
        };

        CharacterModifierRepository::insert(&conn, &modifier1).unwrap();
        CharacterModifierRepository::insert(&conn, &modifier2).unwrap();
        CharacterModifierRepository::insert(&conn, &modifier3).unwrap();

        let modifiers = CharacterModifierRepository::get_by_character_id(&conn, 1).unwrap();

        assert_eq!(modifiers.len(), 3);
    }
}
