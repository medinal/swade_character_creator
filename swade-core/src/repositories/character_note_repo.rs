use rusqlite::{Connection, Row, params};

use crate::error::Result;
use crate::models::CharacterNote;

pub struct CharacterNoteRepository;

impl CharacterNoteRepository {
    /// Get a single note by ID
    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<CharacterNote>> {
        let mut stmt = conn.prepare(
            "SELECT id, character_id, title, body, created_at, updated_at
             FROM character_notes WHERE id = ?",
        )?;

        let mut rows = stmt.query(params![id])?;

        match rows.next()? {
            Some(row) => Ok(Some(Self::row_to_model(row)?)),
            None => Ok(None),
        }
    }

    /// Get all notes for a specific character (newest first)
    pub fn get_by_character_id(
        conn: &Connection,
        character_id: i64,
    ) -> Result<Vec<CharacterNote>> {
        let mut stmt = conn.prepare(
            "SELECT id, character_id, title, body, created_at, updated_at
             FROM character_notes WHERE character_id = ?
             ORDER BY created_at DESC",
        )?;

        let rows = stmt.query_map(params![character_id], Self::row_to_model)?;

        Ok(rows.filter_map(|r| r.ok()).collect())
    }

    /// Insert a new note, return generated ID
    pub fn insert(conn: &Connection, note: &CharacterNote) -> Result<i64> {
        conn.execute(
            "INSERT INTO character_notes (character_id, title, body)
             VALUES (?1, ?2, ?3)",
            params![note.character_id, note.title, note.body],
        )?;

        Ok(conn.last_insert_rowid())
    }

    /// Update an existing note
    pub fn update(conn: &Connection, note: &CharacterNote) -> Result<()> {
        conn.execute(
            "UPDATE character_notes SET
                title = ?1,
                body = ?2,
                updated_at = CURRENT_TIMESTAMP
             WHERE id = ?3",
            params![note.title, note.body, note.id],
        )?;

        Ok(())
    }

    /// Delete a note by ID
    pub fn delete(conn: &Connection, id: i64) -> Result<()> {
        conn.execute("DELETE FROM character_notes WHERE id = ?", params![id])?;
        Ok(())
    }

    /// Delete all notes for a character
    pub fn delete_by_character_id(conn: &Connection, character_id: i64) -> Result<()> {
        conn.execute(
            "DELETE FROM character_notes WHERE character_id = ?",
            params![character_id],
        )?;
        Ok(())
    }

    /// Convert a database row to a CharacterNote model
    fn row_to_model(row: &Row) -> rusqlite::Result<CharacterNote> {
        Ok(CharacterNote {
            id: row.get(0)?,
            character_id: row.get(1)?,
            title: row.get(2)?,
            body: row.get(3)?,
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

        // Insert required rank for foreign key constraint
        conn.execute(
            "INSERT INTO ranks (id, name, min_advances, max_advances, description, created_at, updated_at)
             VALUES (1, 'Novice', 0, 3, 'Starting rank', '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            [],
        ).unwrap();

        conn
    }

    fn insert_test_character(conn: &Connection, id: i64, name: &str) {
        conn.execute(
            "INSERT INTO characters (id, is_wild_card, name, ancestry_id,
                                    attribute_points_spent, attribute_points_earned,
                                    skill_points_spent, skill_points_earned,
                                    hindrance_points_spent, hindrance_points_earned,
                                    hindrance_points_to_edges, hindrance_points_to_attributes,
                                    hindrance_points_to_skills, hindrance_points_to_wealth,
                                    power_points, power_points_used, wounds, fatigue, wealth, background, description,
                                    created_at, updated_at)
             VALUES (?, 1, ?, NULL, 0, 5, 0, 12, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 500,
                     'Test background', 'Test description', '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            params![id, name],
        ).unwrap();
    }

    fn insert_test_note(conn: &Connection, id: i64, character_id: i64, title: &str, body: &str) {
        conn.execute(
            "INSERT INTO character_notes (id, character_id, title, body, created_at, updated_at)
             VALUES (?, ?, ?, ?, '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            params![id, character_id, title, body],
        ).unwrap();
    }

    #[test]
    fn test_get_by_id_found() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_note(&conn, 1, 1, "Session 1 Notes", "We fought goblins.");

        let result = CharacterNoteRepository::get_by_id(&conn, 1).unwrap();

        assert!(result.is_some());
        let note = result.unwrap();
        assert_eq!(note.id, 1);
        assert_eq!(note.character_id, 1);
        assert_eq!(note.title, "Session 1 Notes");
        assert_eq!(note.body, "We fought goblins.");
    }

    #[test]
    fn test_get_by_id_not_found() {
        let conn = setup_test_db();

        let result = CharacterNoteRepository::get_by_id(&conn, 999).unwrap();

        assert!(result.is_none());
    }

    #[test]
    fn test_get_by_character_id_empty() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");

        let notes = CharacterNoteRepository::get_by_character_id(&conn, 1).unwrap();

        assert_eq!(notes.len(), 0);
    }

    #[test]
    fn test_get_by_character_id_multiple() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");

        // Insert with different created_at times to test ordering
        conn.execute(
            "INSERT INTO character_notes (id, character_id, title, body, created_at, updated_at)
             VALUES (1, 1, 'First Note', 'Body 1', '2024-01-01 10:00:00', '2024-01-01 10:00:00')",
            [],
        ).unwrap();
        conn.execute(
            "INSERT INTO character_notes (id, character_id, title, body, created_at, updated_at)
             VALUES (2, 1, 'Second Note', 'Body 2', '2024-01-02 10:00:00', '2024-01-02 10:00:00')",
            [],
        ).unwrap();
        conn.execute(
            "INSERT INTO character_notes (id, character_id, title, body, created_at, updated_at)
             VALUES (3, 1, 'Third Note', 'Body 3', '2024-01-03 10:00:00', '2024-01-03 10:00:00')",
            [],
        ).unwrap();

        let notes = CharacterNoteRepository::get_by_character_id(&conn, 1).unwrap();

        assert_eq!(notes.len(), 3);
        // Should be ordered newest first
        assert_eq!(notes[0].title, "Third Note");
        assert_eq!(notes[1].title, "Second Note");
        assert_eq!(notes[2].title, "First Note");
    }

    #[test]
    fn test_get_by_character_id_filters_correctly() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Character One");
        insert_test_character(&conn, 2, "Character Two");
        insert_test_note(&conn, 1, 1, "Note for Char 1", "Body");
        insert_test_note(&conn, 2, 2, "Note for Char 2", "Body");

        let notes = CharacterNoteRepository::get_by_character_id(&conn, 1).unwrap();

        assert_eq!(notes.len(), 1);
        assert_eq!(notes[0].character_id, 1);
    }

    #[test]
    fn test_insert() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");

        let note = CharacterNote {
            id: 0,
            character_id: 1,
            title: "New Note".to_string(),
            body: "Some content here.".to_string(),
            created_at: String::new(),
            updated_at: String::new(),
        };

        let id = CharacterNoteRepository::insert(&conn, &note).unwrap();

        assert!(id > 0);

        let result = CharacterNoteRepository::get_by_id(&conn, id).unwrap();
        assert!(result.is_some());
        let inserted = result.unwrap();
        assert_eq!(inserted.title, "New Note");
        assert_eq!(inserted.body, "Some content here.");
    }

    #[test]
    fn test_update() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_note(&conn, 1, 1, "Original Title", "Original body");

        let mut note = CharacterNoteRepository::get_by_id(&conn, 1).unwrap().unwrap();
        note.title = "Updated Title".to_string();
        note.body = "Updated body content.".to_string();

        CharacterNoteRepository::update(&conn, &note).unwrap();

        let result = CharacterNoteRepository::get_by_id(&conn, 1).unwrap().unwrap();
        assert_eq!(result.title, "Updated Title");
        assert_eq!(result.body, "Updated body content.");
    }

    #[test]
    fn test_delete() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_note(&conn, 1, 1, "To Delete", "Body");

        // Verify it exists
        assert!(CharacterNoteRepository::get_by_id(&conn, 1).unwrap().is_some());

        // Delete it
        CharacterNoteRepository::delete(&conn, 1).unwrap();

        // Verify it's gone
        assert!(CharacterNoteRepository::get_by_id(&conn, 1).unwrap().is_none());
    }

    #[test]
    fn test_delete_by_character_id() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_note(&conn, 1, 1, "Note 1", "Body 1");
        insert_test_note(&conn, 2, 1, "Note 2", "Body 2");
        insert_test_note(&conn, 3, 1, "Note 3", "Body 3");

        // Verify they exist
        assert_eq!(CharacterNoteRepository::get_by_character_id(&conn, 1).unwrap().len(), 3);

        // Delete all notes for character
        CharacterNoteRepository::delete_by_character_id(&conn, 1).unwrap();

        // Verify they're gone
        assert_eq!(CharacterNoteRepository::get_by_character_id(&conn, 1).unwrap().len(), 0);
    }

    #[test]
    fn test_cascade_delete_on_character_deletion() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_note(&conn, 1, 1, "Note 1", "Body");

        // Verify note exists
        assert!(CharacterNoteRepository::get_by_id(&conn, 1).unwrap().is_some());

        // Delete the character
        conn.execute("DELETE FROM characters WHERE id = 1", []).unwrap();

        // Note should be cascade deleted
        assert!(CharacterNoteRepository::get_by_id(&conn, 1).unwrap().is_none());
    }
}
