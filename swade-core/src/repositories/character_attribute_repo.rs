use crate::error::Result;
use crate::models::CharacterAttribute;
use rusqlite::{Connection, Row, params};

pub struct CharacterAttributeRepository;

impl CharacterAttributeRepository {
    /// Get a single character attribute by ID
    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<CharacterAttribute>> {
        let mut stmt = conn.prepare(
            "SELECT id, character_id, attribute_id, steps_incremented,
                    created_at, updated_at
             FROM character_attributes WHERE id = ?",
        )?;

        let mut rows = stmt.query(params![id])?;

        match rows.next()? {
            Some(row) => Ok(Some(Self::row_to_model(row)?)),
            None => Ok(None),
        }
    }

    /// Get all character attributes for a specific character
    pub fn get_by_character_id(
        conn: &Connection,
        character_id: i64,
    ) -> Result<Vec<CharacterAttribute>> {
        let mut stmt = conn.prepare(
            "SELECT id, character_id, attribute_id, steps_incremented,
                    created_at, updated_at
             FROM character_attributes WHERE character_id = ?",
        )?;

        let rows = stmt.query_map(params![character_id], Self::row_to_model)?;

        Ok(rows.filter_map(|r| r.ok()).collect())
    }

    /// Insert a new character attribute, return generated ID
    pub fn insert(conn: &Connection, model: &CharacterAttribute) -> Result<i64> {
        conn.execute(
            "INSERT INTO character_attributes (character_id, attribute_id, steps_incremented,
                                              created_at, updated_at)
             VALUES (?, ?, ?, ?, ?)",
            params![
                model.character_id,
                model.attribute_id,
                model.steps_incremented,
                model.created_at,
                model.updated_at
            ],
        )?;

        Ok(conn.last_insert_rowid())
    }

    /// Update an existing character attribute
    pub fn update(conn: &Connection, model: &CharacterAttribute) -> Result<()> {
        conn.execute(
            "UPDATE character_attributes
             SET character_id = ?, attribute_id = ?, steps_incremented = ?, updated_at = ?
             WHERE id = ?",
            params![
                model.character_id,
                model.attribute_id,
                model.steps_incremented,
                model.updated_at,
                model.id
            ],
        )?;

        Ok(())
    }

    /// Delete a character attribute by ID
    pub fn delete(conn: &Connection, id: i64) -> Result<()> {
        conn.execute("DELETE FROM character_attributes WHERE id = ?", params![id])?;

        Ok(())
    }

    /// Convert a database row to a CharacterAttribute model
    fn row_to_model(row: &Row) -> rusqlite::Result<CharacterAttribute> {
        Ok(CharacterAttribute {
            id: row.get(0)?,
            character_id: row.get(1)?,
            attribute_id: row.get(2)?,
            steps_incremented: row.get(3)?,
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
        // Enable foreign key constraints
        conn.execute_batch("PRAGMA foreign_keys = ON").unwrap();
        let schema = include_str!("../../../db/schema.sql");
        conn.execute_batch(schema).unwrap();
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

    fn insert_test_attribute(conn: &Connection, id: i64, name: &str) {
        conn.execute(
            "INSERT INTO attributes (id, name, description, base_value,
                                   created_at, updated_at)
             VALUES (?, ?, 'Test attribute', 4,
                     '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            params![id, name],
        )
        .unwrap();
    }

    fn insert_test_character_attribute(
        conn: &Connection,
        id: i64,
        character_id: i64,
        attribute_id: i64,
        steps_incremented: i64,
    ) {
        conn.execute(
            "INSERT INTO character_attributes (id, character_id, attribute_id, steps_incremented,
                                              created_at, updated_at)
             VALUES (?, ?, ?, ?, '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            params![id, character_id, attribute_id, steps_incremented],
        )
        .unwrap();
    }

    #[test]
    fn test_get_by_id_found() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_attribute(&conn, 1, "Agility");
        insert_test_character_attribute(&conn, 1, 1, 1, 2);

        let result = CharacterAttributeRepository::get_by_id(&conn, 1).unwrap();

        assert!(result.is_some());
        let char_attr = result.unwrap();
        assert_eq!(char_attr.id, 1);
        assert_eq!(char_attr.character_id, 1);
        assert_eq!(char_attr.attribute_id, 1);
        assert_eq!(char_attr.steps_incremented, 2);
        assert_eq!(char_attr.created_at, "2024-01-01 00:00:00");
        assert_eq!(char_attr.updated_at, "2024-01-01 00:00:00");
    }

    #[test]
    fn test_get_by_id_not_found() {
        let conn = setup_test_db();

        let result = CharacterAttributeRepository::get_by_id(&conn, 999).unwrap();

        assert!(result.is_none());
    }

    #[test]
    fn test_get_by_character_id_multiple_attributes() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_attribute(&conn, 1, "Agility");
        insert_test_attribute(&conn, 2, "Strength");
        insert_test_attribute(&conn, 3, "Smarts");

        insert_test_character_attribute(&conn, 1, 1, 1, 1);
        insert_test_character_attribute(&conn, 2, 1, 2, 2);
        insert_test_character_attribute(&conn, 3, 1, 3, 0);

        let attributes = CharacterAttributeRepository::get_by_character_id(&conn, 1).unwrap();

        assert_eq!(attributes.len(), 3);
        assert_eq!(attributes[0].character_id, 1);
        assert_eq!(attributes[0].attribute_id, 1);
        assert_eq!(attributes[0].steps_incremented, 1);
        assert_eq!(attributes[1].character_id, 1);
        assert_eq!(attributes[1].attribute_id, 2);
        assert_eq!(attributes[1].steps_incremented, 2);
        assert_eq!(attributes[2].character_id, 1);
        assert_eq!(attributes[2].attribute_id, 3);
        assert_eq!(attributes[2].steps_incremented, 0);
    }

    #[test]
    fn test_get_by_character_id_empty() {
        let conn = setup_test_db();

        let attributes = CharacterAttributeRepository::get_by_character_id(&conn, 999).unwrap();

        assert_eq!(attributes.len(), 0);
    }

    #[test]
    fn test_get_by_character_id_filters_correctly() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Character One");
        insert_test_character(&conn, 2, "Character Two");
        insert_test_attribute(&conn, 1, "Agility");

        insert_test_character_attribute(&conn, 1, 1, 1, 1);
        insert_test_character_attribute(&conn, 2, 2, 1, 2);

        let attributes = CharacterAttributeRepository::get_by_character_id(&conn, 1).unwrap();

        assert_eq!(attributes.len(), 1);
        assert_eq!(attributes[0].character_id, 1);
        assert_eq!(attributes[0].steps_incremented, 1);
    }

    #[test]
    fn test_insert_success() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_attribute(&conn, 1, "Agility");

        let char_attr = CharacterAttribute {
            id: 0, // Will be ignored, ID is auto-generated
            character_id: 1,
            attribute_id: 1,
            steps_incremented: 3,
            created_at: "2024-06-15 10:00:00".to_string(),
            updated_at: "2024-06-15 10:00:00".to_string(),
        };

        let id = CharacterAttributeRepository::insert(&conn, &char_attr).unwrap();

        assert!(id > 0);

        // Verify the inserted record
        let result = CharacterAttributeRepository::get_by_id(&conn, id).unwrap();
        assert!(result.is_some());
        let inserted = result.unwrap();
        assert_eq!(inserted.character_id, 1);
        assert_eq!(inserted.attribute_id, 1);
        assert_eq!(inserted.steps_incremented, 3);
        assert_eq!(inserted.created_at, "2024-06-15 10:00:00");
        assert_eq!(inserted.updated_at, "2024-06-15 10:00:00");
    }

    #[test]
    fn test_insert_with_zero_steps() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_attribute(&conn, 1, "Vigor");

        let char_attr = CharacterAttribute {
            id: 0,
            character_id: 1,
            attribute_id: 1,
            steps_incremented: 0,
            created_at: "2024-01-01 00:00:00".to_string(),
            updated_at: "2024-01-01 00:00:00".to_string(),
        };

        let id = CharacterAttributeRepository::insert(&conn, &char_attr).unwrap();

        let result = CharacterAttributeRepository::get_by_id(&conn, id)
            .unwrap()
            .unwrap();
        assert_eq!(result.steps_incremented, 0);
    }

    #[test]
    fn test_update_success() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_attribute(&conn, 1, "Agility");
        insert_test_attribute(&conn, 2, "Strength");
        insert_test_character_attribute(&conn, 1, 1, 1, 2);

        let mut char_attr = CharacterAttributeRepository::get_by_id(&conn, 1)
            .unwrap()
            .unwrap();
        char_attr.attribute_id = 2;
        char_attr.steps_incremented = 4;
        char_attr.updated_at = "2024-12-01 15:30:00".to_string();

        CharacterAttributeRepository::update(&conn, &char_attr).unwrap();

        // Verify the update
        let result = CharacterAttributeRepository::get_by_id(&conn, 1)
            .unwrap()
            .unwrap();
        assert_eq!(result.attribute_id, 2);
        assert_eq!(result.steps_incremented, 4);
        assert_eq!(result.updated_at, "2024-12-01 15:30:00");
        // created_at should remain unchanged
        assert_eq!(result.created_at, "2024-01-01 00:00:00");
    }

    #[test]
    fn test_update_nonexistent_record() {
        let conn = setup_test_db();

        let char_attr = CharacterAttribute {
            id: 999,
            character_id: 1,
            attribute_id: 1,
            steps_incremented: 2,
            created_at: "2024-01-01 00:00:00".to_string(),
            updated_at: "2024-01-01 00:00:00".to_string(),
        };

        // Should succeed but not update any rows
        let result = CharacterAttributeRepository::update(&conn, &char_attr);
        assert!(result.is_ok());
    }

    #[test]
    fn test_delete_success() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_attribute(&conn, 1, "Agility");
        insert_test_character_attribute(&conn, 1, 1, 1, 2);

        // Verify it exists
        assert!(
            CharacterAttributeRepository::get_by_id(&conn, 1)
                .unwrap()
                .is_some()
        );

        CharacterAttributeRepository::delete(&conn, 1).unwrap();

        // Verify it's deleted
        assert!(
            CharacterAttributeRepository::get_by_id(&conn, 1)
                .unwrap()
                .is_none()
        );
    }

    #[test]
    fn test_delete_nonexistent_record() {
        let conn = setup_test_db();

        // Should succeed even if record doesn't exist
        let result = CharacterAttributeRepository::delete(&conn, 999);
        assert!(result.is_ok());
    }

    #[test]
    fn test_unique_constraint_character_attribute_pair() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_attribute(&conn, 1, "Agility");
        insert_test_character_attribute(&conn, 1, 1, 1, 2);

        // Attempting to insert duplicate character_id/attribute_id pair should fail
        let result = conn.execute(
            "INSERT INTO character_attributes (character_id, attribute_id, steps_incremented,
                                              created_at, updated_at)
             VALUES (1, 1, 3, '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            [],
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_foreign_key_character_id() {
        let conn = setup_test_db();
        insert_test_attribute(&conn, 1, "Agility");

        // Attempting to insert with non-existent character_id should fail
        let result = conn.execute(
            "INSERT INTO character_attributes (character_id, attribute_id, steps_incremented,
                                              created_at, updated_at)
             VALUES (999, 1, 2, '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            [],
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_foreign_key_attribute_id() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");

        // Attempting to insert with non-existent attribute_id should fail
        let result = conn.execute(
            "INSERT INTO character_attributes (character_id, attribute_id, steps_incremented,
                                              created_at, updated_at)
             VALUES (1, 999, 2, '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            [],
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_row_to_model_all_fields() {
        let conn = setup_test_db();
        insert_test_character(&conn, 42, "Special Character");
        insert_test_attribute(&conn, 99, "Special Attribute");

        conn.execute(
            "INSERT INTO character_attributes (id, character_id, attribute_id, steps_incremented,
                                              created_at, updated_at)
             VALUES (100, 42, 99, 5, '2024-12-25 10:30:00', '2024-12-26 15:45:00')",
            [],
        )
        .unwrap();

        let char_attr = CharacterAttributeRepository::get_by_id(&conn, 100)
            .unwrap()
            .unwrap();

        assert_eq!(char_attr.id, 100);
        assert_eq!(char_attr.character_id, 42);
        assert_eq!(char_attr.attribute_id, 99);
        assert_eq!(char_attr.steps_incremented, 5);
        assert_eq!(char_attr.created_at, "2024-12-25 10:30:00");
        assert_eq!(char_attr.updated_at, "2024-12-26 15:45:00");
    }

    #[test]
    fn test_delete_character_requires_deleting_attributes_first() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_attribute(&conn, 1, "Agility");
        insert_test_character_attribute(&conn, 1, 1, 1, 2);

        // Verify character attribute exists
        assert!(
            CharacterAttributeRepository::get_by_id(&conn, 1)
                .unwrap()
                .is_some()
        );

        // Attempting to delete the character should fail due to foreign key constraint
        let result = conn.execute("DELETE FROM characters WHERE id = 1", []);
        assert!(result.is_err());

        // Must delete character attributes first
        CharacterAttributeRepository::delete(&conn, 1).unwrap();

        // Now deleting the character should succeed
        conn.execute("DELETE FROM characters WHERE id = 1", [])
            .unwrap();
    }

    #[test]
    fn test_insert_multiple_attributes_for_same_character() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_attribute(&conn, 1, "Agility");
        insert_test_attribute(&conn, 2, "Strength");
        insert_test_attribute(&conn, 3, "Smarts");

        let attr1 = CharacterAttribute {
            id: 0,
            character_id: 1,
            attribute_id: 1,
            steps_incremented: 1,
            created_at: "2024-01-01 00:00:00".to_string(),
            updated_at: "2024-01-01 00:00:00".to_string(),
        };

        let attr2 = CharacterAttribute {
            id: 0,
            character_id: 1,
            attribute_id: 2,
            steps_incremented: 2,
            created_at: "2024-01-01 00:00:00".to_string(),
            updated_at: "2024-01-01 00:00:00".to_string(),
        };

        let attr3 = CharacterAttribute {
            id: 0,
            character_id: 1,
            attribute_id: 3,
            steps_incremented: 0,
            created_at: "2024-01-01 00:00:00".to_string(),
            updated_at: "2024-01-01 00:00:00".to_string(),
        };

        CharacterAttributeRepository::insert(&conn, &attr1).unwrap();
        CharacterAttributeRepository::insert(&conn, &attr2).unwrap();
        CharacterAttributeRepository::insert(&conn, &attr3).unwrap();

        let attributes = CharacterAttributeRepository::get_by_character_id(&conn, 1).unwrap();
        assert_eq!(attributes.len(), 3);
    }
}
