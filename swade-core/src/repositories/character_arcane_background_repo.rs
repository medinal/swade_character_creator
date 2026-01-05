use crate::error::Result;
use crate::models::CharacterArcaneBackground;
use rusqlite::{Connection, Row, params};

pub struct CharacterArcaneBackgroundRepository;

impl CharacterArcaneBackgroundRepository {
    /// Get a single character arcane background by ID
    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<CharacterArcaneBackground>> {
        let mut stmt = conn.prepare(
            "SELECT id, character_id, arcane_background_id, advance_taken,
                    created_at, updated_at
             FROM character_arcane_backgrounds WHERE id = ?",
        )?;

        let mut rows = stmt.query(params![id])?;

        match rows.next()? {
            Some(row) => Ok(Some(Self::row_to_model(row)?)),
            None => Ok(None),
        }
    }

    /// Get all character arcane backgrounds for a specific character
    pub fn get_by_character_id(
        conn: &Connection,
        character_id: i64,
    ) -> Result<Vec<CharacterArcaneBackground>> {
        let mut stmt = conn.prepare(
            "SELECT id, character_id, arcane_background_id, advance_taken,
                    created_at, updated_at
             FROM character_arcane_backgrounds WHERE character_id = ?",
        )?;

        let rows = stmt.query_map(params![character_id], Self::row_to_model)?;

        Ok(rows.filter_map(|r| r.ok()).collect())
    }

    /// Insert a new character arcane background, return generated ID
    pub fn insert(conn: &Connection, model: &CharacterArcaneBackground) -> Result<i64> {
        conn.execute(
            "INSERT INTO character_arcane_backgrounds (character_id, arcane_background_id, advance_taken,
                                                       created_at, updated_at)
             VALUES (?, ?, ?, ?, ?)",
            params![
                model.character_id,
                model.arcane_background_id,
                model.advance_taken,
                model.created_at,
                model.updated_at
            ],
        )?;

        Ok(conn.last_insert_rowid())
    }

    /// Update an existing character arcane background
    pub fn update(conn: &Connection, model: &CharacterArcaneBackground) -> Result<()> {
        conn.execute(
            "UPDATE character_arcane_backgrounds
             SET character_id = ?, arcane_background_id = ?, advance_taken = ?, updated_at = ?
             WHERE id = ?",
            params![
                model.character_id,
                model.arcane_background_id,
                model.advance_taken,
                model.updated_at,
                model.id
            ],
        )?;

        Ok(())
    }

    /// Delete a character arcane background by ID
    pub fn delete(conn: &Connection, id: i64) -> Result<()> {
        conn.execute(
            "DELETE FROM character_arcane_backgrounds WHERE id = ?",
            params![id],
        )?;

        Ok(())
    }

    /// Delete all arcane backgrounds for a character
    pub fn delete_by_character_id(conn: &Connection, character_id: i64) -> Result<()> {
        conn.execute(
            "DELETE FROM character_arcane_backgrounds WHERE character_id = ?",
            params![character_id],
        )?;

        Ok(())
    }

    /// Convert a database row to a CharacterArcaneBackground model
    fn row_to_model(row: &Row) -> rusqlite::Result<CharacterArcaneBackground> {
        Ok(CharacterArcaneBackground {
            id: row.get(0)?,
            character_id: row.get(1)?,
            arcane_background_id: row.get(2)?,
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

    fn insert_test_skill(conn: &Connection, id: i64, name: &str, attribute_id: i64) {
        conn.execute(
            "INSERT INTO skills (id, name, description, linked_attribute_id,
                               created_at, updated_at)
             VALUES (?, ?, 'Test skill', ?,
                     '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            params![id, name, attribute_id],
        )
        .unwrap();
    }

    fn insert_test_arcane_background(conn: &Connection, id: i64, name: &str, arcane_skill_id: i64) {
        conn.execute(
            "INSERT INTO arcane_backgrounds (id, name, arcane_skill_id, starting_powers,
                                            starting_power_points, description,
                                            created_at, updated_at)
             VALUES (?, ?, ?, 3, 10, 'Test arcane background',
                     '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            params![id, name, arcane_skill_id],
        )
        .unwrap();
    }

    fn insert_test_character_arcane_background(
        conn: &Connection,
        id: i64,
        character_id: i64,
        arcane_background_id: i64,
        advance_taken: Option<i64>,
    ) {
        conn.execute(
            "INSERT INTO character_arcane_backgrounds (id, character_id, arcane_background_id,
                                                      advance_taken, created_at, updated_at)
             VALUES (?, ?, ?, ?, '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            params![id, character_id, arcane_background_id, advance_taken],
        )
        .unwrap();
    }

    #[test]
    fn test_get_by_id_found() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_attribute(&conn, 1, "Smarts");
        insert_test_skill(&conn, 1, "Spellcasting", 1);
        insert_test_arcane_background(&conn, 1, "Magic", 1);
        insert_test_character_arcane_background(&conn, 1, 1, 1, Some(0));

        let result = CharacterArcaneBackgroundRepository::get_by_id(&conn, 1).unwrap();

        assert!(result.is_some());
        let char_ab = result.unwrap();
        assert_eq!(char_ab.id, 1);
        assert_eq!(char_ab.character_id, 1);
        assert_eq!(char_ab.arcane_background_id, 1);
        assert_eq!(char_ab.advance_taken, Some(0));
        assert_eq!(char_ab.created_at, "2024-01-01 00:00:00");
        assert_eq!(char_ab.updated_at, "2024-01-01 00:00:00");
    }

    #[test]
    fn test_get_by_id_not_found() {
        let conn = setup_test_db();

        let result = CharacterArcaneBackgroundRepository::get_by_id(&conn, 999).unwrap();

        assert!(result.is_none());
    }

    #[test]
    fn test_get_by_id_with_null_advance_taken() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_attribute(&conn, 1, "Smarts");
        insert_test_skill(&conn, 1, "Spellcasting", 1);
        insert_test_arcane_background(&conn, 1, "Magic", 1);
        insert_test_character_arcane_background(&conn, 1, 1, 1, None);

        let result = CharacterArcaneBackgroundRepository::get_by_id(&conn, 1).unwrap();

        assert!(result.is_some());
        let char_ab = result.unwrap();
        assert_eq!(char_ab.advance_taken, None);
    }

    #[test]
    fn test_get_by_character_id_multiple_backgrounds() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_attribute(&conn, 1, "Smarts");
        insert_test_skill(&conn, 1, "Spellcasting", 1);
        insert_test_skill(&conn, 2, "Faith", 1);
        insert_test_arcane_background(&conn, 1, "Magic", 1);
        insert_test_arcane_background(&conn, 2, "Miracles", 2);

        insert_test_character_arcane_background(&conn, 1, 1, 1, Some(0));
        insert_test_character_arcane_background(&conn, 2, 1, 2, Some(4));

        let backgrounds =
            CharacterArcaneBackgroundRepository::get_by_character_id(&conn, 1).unwrap();

        assert_eq!(backgrounds.len(), 2);
        assert_eq!(backgrounds[0].character_id, 1);
        assert_eq!(backgrounds[0].arcane_background_id, 1);
        assert_eq!(backgrounds[0].advance_taken, Some(0));
        assert_eq!(backgrounds[1].character_id, 1);
        assert_eq!(backgrounds[1].arcane_background_id, 2);
        assert_eq!(backgrounds[1].advance_taken, Some(4));
    }

    #[test]
    fn test_get_by_character_id_empty() {
        let conn = setup_test_db();

        let backgrounds =
            CharacterArcaneBackgroundRepository::get_by_character_id(&conn, 999).unwrap();

        assert_eq!(backgrounds.len(), 0);
    }

    #[test]
    fn test_get_by_character_id_filters_correctly() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Character One");
        insert_test_character(&conn, 2, "Character Two");
        insert_test_attribute(&conn, 1, "Smarts");
        insert_test_skill(&conn, 1, "Spellcasting", 1);
        insert_test_arcane_background(&conn, 1, "Magic", 1);

        insert_test_character_arcane_background(&conn, 1, 1, 1, Some(0));
        insert_test_character_arcane_background(&conn, 2, 2, 1, Some(0));

        let backgrounds =
            CharacterArcaneBackgroundRepository::get_by_character_id(&conn, 1).unwrap();

        assert_eq!(backgrounds.len(), 1);
        assert_eq!(backgrounds[0].character_id, 1);
    }

    #[test]
    fn test_insert_success() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_attribute(&conn, 1, "Smarts");
        insert_test_skill(&conn, 1, "Spellcasting", 1);
        insert_test_arcane_background(&conn, 1, "Magic", 1);

        let char_ab = CharacterArcaneBackground {
            id: 0, // Will be ignored, ID is auto-generated
            character_id: 1,
            arcane_background_id: 1,
            advance_taken: Some(0),
            created_at: "2024-06-15 10:00:00".to_string(),
            updated_at: "2024-06-15 10:00:00".to_string(),
        };

        let id = CharacterArcaneBackgroundRepository::insert(&conn, &char_ab).unwrap();

        assert!(id > 0);

        // Verify the inserted record
        let result = CharacterArcaneBackgroundRepository::get_by_id(&conn, id).unwrap();
        assert!(result.is_some());
        let inserted = result.unwrap();
        assert_eq!(inserted.character_id, 1);
        assert_eq!(inserted.arcane_background_id, 1);
        assert_eq!(inserted.advance_taken, Some(0));
        assert_eq!(inserted.created_at, "2024-06-15 10:00:00");
        assert_eq!(inserted.updated_at, "2024-06-15 10:00:00");
    }

    #[test]
    fn test_insert_with_null_advance_taken() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_attribute(&conn, 1, "Smarts");
        insert_test_skill(&conn, 1, "Spellcasting", 1);
        insert_test_arcane_background(&conn, 1, "Magic", 1);

        let char_ab = CharacterArcaneBackground {
            id: 0,
            character_id: 1,
            arcane_background_id: 1,
            advance_taken: None,
            created_at: "2024-01-01 00:00:00".to_string(),
            updated_at: "2024-01-01 00:00:00".to_string(),
        };

        let id = CharacterArcaneBackgroundRepository::insert(&conn, &char_ab).unwrap();

        let result = CharacterArcaneBackgroundRepository::get_by_id(&conn, id)
            .unwrap()
            .unwrap();
        assert_eq!(result.advance_taken, None);
    }

    #[test]
    fn test_update_success() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_attribute(&conn, 1, "Smarts");
        insert_test_skill(&conn, 1, "Spellcasting", 1);
        insert_test_skill(&conn, 2, "Faith", 1);
        insert_test_arcane_background(&conn, 1, "Magic", 1);
        insert_test_arcane_background(&conn, 2, "Miracles", 2);
        insert_test_character_arcane_background(&conn, 1, 1, 1, Some(0));

        let mut char_ab = CharacterArcaneBackgroundRepository::get_by_id(&conn, 1)
            .unwrap()
            .unwrap();
        char_ab.arcane_background_id = 2;
        char_ab.advance_taken = Some(5);
        char_ab.updated_at = "2024-12-01 15:30:00".to_string();

        CharacterArcaneBackgroundRepository::update(&conn, &char_ab).unwrap();

        // Verify the update
        let result = CharacterArcaneBackgroundRepository::get_by_id(&conn, 1)
            .unwrap()
            .unwrap();
        assert_eq!(result.arcane_background_id, 2);
        assert_eq!(result.advance_taken, Some(5));
        assert_eq!(result.updated_at, "2024-12-01 15:30:00");
        // created_at should remain unchanged
        assert_eq!(result.created_at, "2024-01-01 00:00:00");
    }

    #[test]
    fn test_update_advance_taken_to_null() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_attribute(&conn, 1, "Smarts");
        insert_test_skill(&conn, 1, "Spellcasting", 1);
        insert_test_arcane_background(&conn, 1, "Magic", 1);
        insert_test_character_arcane_background(&conn, 1, 1, 1, Some(0));

        let mut char_ab = CharacterArcaneBackgroundRepository::get_by_id(&conn, 1)
            .unwrap()
            .unwrap();
        char_ab.advance_taken = None;
        char_ab.updated_at = "2024-12-01 15:30:00".to_string();

        CharacterArcaneBackgroundRepository::update(&conn, &char_ab).unwrap();

        // Verify the update
        let result = CharacterArcaneBackgroundRepository::get_by_id(&conn, 1)
            .unwrap()
            .unwrap();
        assert_eq!(result.advance_taken, None);
    }

    #[test]
    fn test_update_nonexistent_record() {
        let conn = setup_test_db();

        let char_ab = CharacterArcaneBackground {
            id: 999,
            character_id: 1,
            arcane_background_id: 1,
            advance_taken: Some(0),
            created_at: "2024-01-01 00:00:00".to_string(),
            updated_at: "2024-01-01 00:00:00".to_string(),
        };

        // Should succeed but not update any rows
        let result = CharacterArcaneBackgroundRepository::update(&conn, &char_ab);
        assert!(result.is_ok());
    }

    #[test]
    fn test_delete_success() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_attribute(&conn, 1, "Smarts");
        insert_test_skill(&conn, 1, "Spellcasting", 1);
        insert_test_arcane_background(&conn, 1, "Magic", 1);
        insert_test_character_arcane_background(&conn, 1, 1, 1, Some(0));

        // Verify it exists
        assert!(
            CharacterArcaneBackgroundRepository::get_by_id(&conn, 1)
                .unwrap()
                .is_some()
        );

        CharacterArcaneBackgroundRepository::delete(&conn, 1).unwrap();

        // Verify it's deleted
        assert!(
            CharacterArcaneBackgroundRepository::get_by_id(&conn, 1)
                .unwrap()
                .is_none()
        );
    }

    #[test]
    fn test_delete_nonexistent_record() {
        let conn = setup_test_db();

        // Should succeed even if record doesn't exist
        let result = CharacterArcaneBackgroundRepository::delete(&conn, 999);
        assert!(result.is_ok());
    }

    #[test]
    fn test_unique_constraint_character_arcane_background_pair() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_attribute(&conn, 1, "Smarts");
        insert_test_skill(&conn, 1, "Spellcasting", 1);
        insert_test_arcane_background(&conn, 1, "Magic", 1);
        insert_test_character_arcane_background(&conn, 1, 1, 1, Some(0));

        // Attempting to insert duplicate character_id/arcane_background_id pair should fail
        let result = conn.execute(
            "INSERT INTO character_arcane_backgrounds (character_id, arcane_background_id,
                                                      advance_taken, created_at, updated_at)
             VALUES (1, 1, 5, '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            [],
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_foreign_key_character_id() {
        let conn = setup_test_db();
        insert_test_attribute(&conn, 1, "Smarts");
        insert_test_skill(&conn, 1, "Spellcasting", 1);
        insert_test_arcane_background(&conn, 1, "Magic", 1);

        // Attempting to insert with non-existent character_id should fail
        let result = conn.execute(
            "INSERT INTO character_arcane_backgrounds (character_id, arcane_background_id,
                                                      advance_taken, created_at, updated_at)
             VALUES (999, 1, 0, '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            [],
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_foreign_key_arcane_background_id() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");

        // Attempting to insert with non-existent arcane_background_id should fail
        let result = conn.execute(
            "INSERT INTO character_arcane_backgrounds (character_id, arcane_background_id,
                                                      advance_taken, created_at, updated_at)
             VALUES (1, 999, 0, '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            [],
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_row_to_model_all_fields() {
        let conn = setup_test_db();
        insert_test_character(&conn, 42, "Special Character");
        insert_test_attribute(&conn, 1, "Smarts");
        insert_test_skill(&conn, 99, "Special Skill", 1);
        insert_test_arcane_background(&conn, 50, "Special Background", 99);

        conn.execute(
            "INSERT INTO character_arcane_backgrounds (id, character_id, arcane_background_id,
                                                      advance_taken, created_at, updated_at)
             VALUES (100, 42, 50, 7, '2024-12-25 10:30:00', '2024-12-26 15:45:00')",
            [],
        )
        .unwrap();

        let char_ab = CharacterArcaneBackgroundRepository::get_by_id(&conn, 100)
            .unwrap()
            .unwrap();

        assert_eq!(char_ab.id, 100);
        assert_eq!(char_ab.character_id, 42);
        assert_eq!(char_ab.arcane_background_id, 50);
        assert_eq!(char_ab.advance_taken, Some(7));
        assert_eq!(char_ab.created_at, "2024-12-25 10:30:00");
        assert_eq!(char_ab.updated_at, "2024-12-26 15:45:00");
    }

    #[test]
    fn test_insert_multiple_backgrounds_for_same_character() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_attribute(&conn, 1, "Smarts");
        insert_test_skill(&conn, 1, "Spellcasting", 1);
        insert_test_skill(&conn, 2, "Faith", 1);
        insert_test_skill(&conn, 3, "Psionics", 1);
        insert_test_arcane_background(&conn, 1, "Magic", 1);
        insert_test_arcane_background(&conn, 2, "Miracles", 2);
        insert_test_arcane_background(&conn, 3, "Psionics", 3);

        let ab1 = CharacterArcaneBackground {
            id: 0,
            character_id: 1,
            arcane_background_id: 1,
            advance_taken: Some(0),
            created_at: "2024-01-01 00:00:00".to_string(),
            updated_at: "2024-01-01 00:00:00".to_string(),
        };

        let ab2 = CharacterArcaneBackground {
            id: 0,
            character_id: 1,
            arcane_background_id: 2,
            advance_taken: Some(4),
            created_at: "2024-01-01 00:00:00".to_string(),
            updated_at: "2024-01-01 00:00:00".to_string(),
        };

        let ab3 = CharacterArcaneBackground {
            id: 0,
            character_id: 1,
            arcane_background_id: 3,
            advance_taken: Some(8),
            created_at: "2024-01-01 00:00:00".to_string(),
            updated_at: "2024-01-01 00:00:00".to_string(),
        };

        CharacterArcaneBackgroundRepository::insert(&conn, &ab1).unwrap();
        CharacterArcaneBackgroundRepository::insert(&conn, &ab2).unwrap();
        CharacterArcaneBackgroundRepository::insert(&conn, &ab3).unwrap();

        let backgrounds =
            CharacterArcaneBackgroundRepository::get_by_character_id(&conn, 1).unwrap();
        assert_eq!(backgrounds.len(), 3);
    }
}
