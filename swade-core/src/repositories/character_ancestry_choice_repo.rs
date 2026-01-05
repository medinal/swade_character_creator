use crate::error::Result;
use crate::models::CharacterAncestryChoice;
use rusqlite::{Connection, Row, params};

pub struct CharacterAncestryChoiceRepository;

impl CharacterAncestryChoiceRepository {
    /// Get a single character ancestry choice by ID
    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<CharacterAncestryChoice>> {
        let mut stmt = conn.prepare(
            "SELECT id, character_id, choice_id, selected_option_id,
                    created_at, updated_at
             FROM character_ancestry_choices WHERE id = ?",
        )?;

        let mut rows = stmt.query(params![id])?;

        match rows.next()? {
            Some(row) => Ok(Some(Self::row_to_model(row)?)),
            None => Ok(None),
        }
    }

    /// Get all character ancestry choices for a specific character
    pub fn get_by_character_id(
        conn: &Connection,
        character_id: i64,
    ) -> Result<Vec<CharacterAncestryChoice>> {
        let mut stmt = conn.prepare(
            "SELECT id, character_id, choice_id, selected_option_id,
                    created_at, updated_at
             FROM character_ancestry_choices WHERE character_id = ? ORDER BY id",
        )?;

        let rows = stmt.query_map(params![character_id], Self::row_to_model)?;

        Ok(rows.filter_map(|r| r.ok()).collect())
    }

    /// Insert a new character ancestry choice, returns the generated ID
    pub fn insert(conn: &Connection, model: &CharacterAncestryChoice) -> Result<i64> {
        conn.execute(
            "INSERT INTO character_ancestry_choices (character_id, choice_id, selected_option_id,
                                                     created_at, updated_at)
             VALUES (?, ?, ?, ?, ?)",
            params![
                model.character_id,
                model.choice_id,
                model.selected_option_id,
                model.created_at,
                model.updated_at
            ],
        )?;

        Ok(conn.last_insert_rowid())
    }

    /// Update an existing character ancestry choice
    pub fn update(conn: &Connection, model: &CharacterAncestryChoice) -> Result<()> {
        conn.execute(
            "UPDATE character_ancestry_choices
             SET character_id = ?, choice_id = ?, selected_option_id = ?,
                 updated_at = ?
             WHERE id = ?",
            params![
                model.character_id,
                model.choice_id,
                model.selected_option_id,
                model.updated_at,
                model.id
            ],
        )?;

        Ok(())
    }

    /// Delete a character ancestry choice by ID
    pub fn delete(conn: &Connection, id: i64) -> Result<()> {
        conn.execute(
            "DELETE FROM character_ancestry_choices WHERE id = ?",
            params![id],
        )?;
        Ok(())
    }

    /// Delete all ancestry choices for a character
    pub fn delete_by_character_id(conn: &Connection, character_id: i64) -> Result<()> {
        conn.execute(
            "DELETE FROM character_ancestry_choices WHERE character_id = ?",
            params![character_id],
        )?;
        Ok(())
    }

    /// Convert a database row to a CharacterAncestryChoice model
    fn row_to_model(row: &Row) -> rusqlite::Result<CharacterAncestryChoice> {
        Ok(CharacterAncestryChoice {
            id: row.get(0)?,
            character_id: row.get(1)?,
            choice_id: row.get(2)?,
            selected_option_id: row.get(3)?,
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
            "INSERT INTO characters (id, name, is_wild_card, created_at, updated_at)
             VALUES (?, ?, 1, '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            params![id, name],
        )
        .unwrap();
    }

    fn insert_test_ancestry(conn: &Connection, id: i64, name: &str) {
        conn.execute(
            "INSERT INTO ancestries (id, name, source, description, created_at, updated_at)
             VALUES (?, ?, 'core', 'Test ancestry description', '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            params![id, name],
        ).unwrap();
    }

    fn insert_test_ancestry_choice(
        conn: &Connection,
        id: i64,
        ancestry_id: i64,
        choice_type: &str,
    ) {
        conn.execute(
            "INSERT INTO ancestry_choices (id, ancestry_id, choice_type, min_selections, max_selections,
                                          description, created_at, updated_at)
             VALUES (?, ?, ?, 1, 1, 'Test choice description',
                     '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            params![id, ancestry_id, choice_type],
        ).unwrap();
    }

    fn insert_test_ancestry_choice_option(
        conn: &Connection,
        id: i64,
        choice_id: i64,
        option_type: &str,
    ) {
        conn.execute(
            "INSERT INTO ancestry_choice_options (id, choice_id, option_type, option_description,
                                                  created_at, updated_at)
             VALUES (?, ?, ?, 'Test option description',
                     '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            params![id, choice_id, option_type],
        )
        .unwrap();
    }

    fn insert_test_character_ancestry_choice(
        conn: &Connection,
        id: i64,
        character_id: i64,
        choice_id: i64,
        selected_option_id: i64,
    ) {
        conn.execute(
            "INSERT INTO character_ancestry_choices (id, character_id, choice_id, selected_option_id,
                                                     created_at, updated_at)
             VALUES (?, ?, ?, ?, '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            params![id, character_id, choice_id, selected_option_id],
        ).unwrap();
    }

    #[test]
    fn test_get_by_id_found() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_ancestry(&conn, 1, "Human");
        insert_test_ancestry_choice(&conn, 1, 1, "free_edge");
        insert_test_ancestry_choice_option(&conn, 1, 1, "edge");
        insert_test_character_ancestry_choice(&conn, 1, 1, 1, 1);

        let result = CharacterAncestryChoiceRepository::get_by_id(&conn, 1).unwrap();

        assert!(result.is_some());
        let choice = result.unwrap();
        assert_eq!(choice.id, 1);
        assert_eq!(choice.character_id, 1);
        assert_eq!(choice.choice_id, 1);
        assert_eq!(choice.selected_option_id, 1);
        assert_eq!(choice.created_at, "2024-01-01 00:00:00");
        assert_eq!(choice.updated_at, "2024-01-01 00:00:00");
    }

    #[test]
    fn test_get_by_id_not_found() {
        let conn = setup_test_db();

        let result = CharacterAncestryChoiceRepository::get_by_id(&conn, 999).unwrap();

        assert!(result.is_none());
    }

    #[test]
    fn test_get_by_character_id_empty() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");

        let choices = CharacterAncestryChoiceRepository::get_by_character_id(&conn, 1).unwrap();

        assert_eq!(choices.len(), 0);
    }

    #[test]
    fn test_get_by_character_id_single_choice() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_ancestry(&conn, 1, "Human");
        insert_test_ancestry_choice(&conn, 1, 1, "free_edge");
        insert_test_ancestry_choice_option(&conn, 1, 1, "edge");
        insert_test_character_ancestry_choice(&conn, 1, 1, 1, 1);

        let choices = CharacterAncestryChoiceRepository::get_by_character_id(&conn, 1).unwrap();

        assert_eq!(choices.len(), 1);
        assert_eq!(choices[0].character_id, 1);
        assert_eq!(choices[0].choice_id, 1);
        assert_eq!(choices[0].selected_option_id, 1);
    }

    #[test]
    fn test_get_by_character_id_multiple_choices() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_ancestry(&conn, 1, "Human");
        insert_test_ancestry_choice(&conn, 1, 1, "free_edge");
        insert_test_ancestry_choice(&conn, 2, 1, "heritage_trait");
        insert_test_ancestry_choice_option(&conn, 1, 1, "edge");
        insert_test_ancestry_choice_option(&conn, 2, 2, "modifier");
        insert_test_character_ancestry_choice(&conn, 1, 1, 1, 1);
        insert_test_character_ancestry_choice(&conn, 2, 1, 2, 2);

        let choices = CharacterAncestryChoiceRepository::get_by_character_id(&conn, 1).unwrap();

        assert_eq!(choices.len(), 2);
        assert_eq!(choices[0].choice_id, 1);
        assert_eq!(choices[1].choice_id, 2);
    }

    #[test]
    fn test_get_by_character_id_does_not_return_other_characters() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Character One");
        insert_test_character(&conn, 2, "Character Two");
        insert_test_ancestry(&conn, 1, "Human");
        insert_test_ancestry_choice(&conn, 1, 1, "free_edge");
        insert_test_ancestry_choice_option(&conn, 1, 1, "edge");
        insert_test_character_ancestry_choice(&conn, 1, 1, 1, 1);
        insert_test_character_ancestry_choice(&conn, 2, 2, 1, 1);

        let choices = CharacterAncestryChoiceRepository::get_by_character_id(&conn, 1).unwrap();

        assert_eq!(choices.len(), 1);
        assert_eq!(choices[0].character_id, 1);
    }

    #[test]
    fn test_insert_basic() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_ancestry(&conn, 1, "Human");
        insert_test_ancestry_choice(&conn, 1, 1, "free_edge");
        insert_test_ancestry_choice_option(&conn, 1, 1, "edge");

        let choice = CharacterAncestryChoice {
            id: 0, // Will be set by database
            character_id: 1,
            choice_id: 1,
            selected_option_id: 1,
            created_at: "2024-01-01 00:00:00".to_string(),
            updated_at: "2024-01-01 00:00:00".to_string(),
        };

        let id = CharacterAncestryChoiceRepository::insert(&conn, &choice).unwrap();

        assert!(id > 0);
        let retrieved = CharacterAncestryChoiceRepository::get_by_id(&conn, id).unwrap();
        assert!(retrieved.is_some());
        let retrieved_choice = retrieved.unwrap();
        assert_eq!(retrieved_choice.character_id, 1);
        assert_eq!(retrieved_choice.choice_id, 1);
        assert_eq!(retrieved_choice.selected_option_id, 1);
    }

    #[test]
    fn test_insert_multiple_choices_for_character() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_ancestry(&conn, 1, "Human");
        insert_test_ancestry_choice(&conn, 1, 1, "free_edge");
        insert_test_ancestry_choice(&conn, 2, 1, "heritage_trait");
        insert_test_ancestry_choice_option(&conn, 1, 1, "edge");
        insert_test_ancestry_choice_option(&conn, 2, 2, "modifier");

        let choice1 = CharacterAncestryChoice {
            id: 0,
            character_id: 1,
            choice_id: 1,
            selected_option_id: 1,
            created_at: "2024-01-01 00:00:00".to_string(),
            updated_at: "2024-01-01 00:00:00".to_string(),
        };

        let choice2 = CharacterAncestryChoice {
            id: 0,
            character_id: 1,
            choice_id: 2,
            selected_option_id: 2,
            created_at: "2024-01-01 00:00:00".to_string(),
            updated_at: "2024-01-01 00:00:00".to_string(),
        };

        CharacterAncestryChoiceRepository::insert(&conn, &choice1).unwrap();
        CharacterAncestryChoiceRepository::insert(&conn, &choice2).unwrap();

        let choices = CharacterAncestryChoiceRepository::get_by_character_id(&conn, 1).unwrap();
        assert_eq!(choices.len(), 2);
    }

    #[test]
    fn test_update_selected_option() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_ancestry(&conn, 1, "Human");
        insert_test_ancestry_choice(&conn, 1, 1, "free_edge");
        insert_test_ancestry_choice_option(&conn, 1, 1, "edge");
        insert_test_ancestry_choice_option(&conn, 2, 1, "edge");
        insert_test_character_ancestry_choice(&conn, 1, 1, 1, 1);

        let mut choice = CharacterAncestryChoiceRepository::get_by_id(&conn, 1)
            .unwrap()
            .unwrap();
        choice.selected_option_id = 2;
        choice.updated_at = "2024-01-02 00:00:00".to_string();

        CharacterAncestryChoiceRepository::update(&conn, &choice).unwrap();

        let updated = CharacterAncestryChoiceRepository::get_by_id(&conn, 1)
            .unwrap()
            .unwrap();
        assert_eq!(updated.selected_option_id, 2);
        assert_eq!(updated.updated_at, "2024-01-02 00:00:00");
    }

    #[test]
    fn test_update_choice_id() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_ancestry(&conn, 1, "Human");
        insert_test_ancestry_choice(&conn, 1, 1, "free_edge");
        insert_test_ancestry_choice(&conn, 2, 1, "heritage_trait");
        insert_test_ancestry_choice_option(&conn, 1, 1, "edge");
        insert_test_ancestry_choice_option(&conn, 2, 2, "modifier");
        insert_test_character_ancestry_choice(&conn, 1, 1, 1, 1);

        let mut choice = CharacterAncestryChoiceRepository::get_by_id(&conn, 1)
            .unwrap()
            .unwrap();
        choice.choice_id = 2;
        choice.selected_option_id = 2;
        choice.updated_at = "2024-01-02 00:00:00".to_string();

        CharacterAncestryChoiceRepository::update(&conn, &choice).unwrap();

        let updated = CharacterAncestryChoiceRepository::get_by_id(&conn, 1)
            .unwrap()
            .unwrap();
        assert_eq!(updated.choice_id, 2);
        assert_eq!(updated.selected_option_id, 2);
    }

    #[test]
    fn test_delete_basic() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_ancestry(&conn, 1, "Human");
        insert_test_ancestry_choice(&conn, 1, 1, "free_edge");
        insert_test_ancestry_choice_option(&conn, 1, 1, "edge");
        insert_test_character_ancestry_choice(&conn, 1, 1, 1, 1);

        CharacterAncestryChoiceRepository::delete(&conn, 1).unwrap();

        let result = CharacterAncestryChoiceRepository::get_by_id(&conn, 1).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_delete_nonexistent() {
        let conn = setup_test_db();

        // Should not error when deleting nonexistent record
        let result = CharacterAncestryChoiceRepository::delete(&conn, 999);
        assert!(result.is_ok());
    }

    #[test]
    fn test_delete_does_not_affect_other_records() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_ancestry(&conn, 1, "Human");
        insert_test_ancestry_choice(&conn, 1, 1, "free_edge");
        insert_test_ancestry_choice(&conn, 2, 1, "heritage_trait");
        insert_test_ancestry_choice_option(&conn, 1, 1, "edge");
        insert_test_ancestry_choice_option(&conn, 2, 2, "modifier");
        insert_test_character_ancestry_choice(&conn, 1, 1, 1, 1);
        insert_test_character_ancestry_choice(&conn, 2, 1, 2, 2);

        CharacterAncestryChoiceRepository::delete(&conn, 1).unwrap();

        let choices = CharacterAncestryChoiceRepository::get_by_character_id(&conn, 1).unwrap();
        assert_eq!(choices.len(), 1);
        assert_eq!(choices[0].id, 2);
    }

    #[test]
    fn test_row_to_model_all_fields() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_ancestry(&conn, 1, "Human");
        insert_test_ancestry_choice(&conn, 1, 1, "free_edge");
        insert_test_ancestry_choice_option(&conn, 1, 1, "edge");
        conn.execute(
            "INSERT INTO character_ancestry_choices (id, character_id, choice_id, selected_option_id,
                                                     created_at, updated_at)
             VALUES (42, 1, 1, 1, '2024-12-25 10:30:00', '2024-12-26 15:45:00')",
            [],
        ).unwrap();

        let choice = CharacterAncestryChoiceRepository::get_by_id(&conn, 42)
            .unwrap()
            .unwrap();

        assert_eq!(choice.id, 42);
        assert_eq!(choice.character_id, 1);
        assert_eq!(choice.choice_id, 1);
        assert_eq!(choice.selected_option_id, 1);
        assert_eq!(choice.created_at, "2024-12-25 10:30:00");
        assert_eq!(choice.updated_at, "2024-12-26 15:45:00");
    }

    #[test]
    fn test_cascade_delete_when_character_deleted() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_ancestry(&conn, 1, "Human");
        insert_test_ancestry_choice(&conn, 1, 1, "free_edge");
        insert_test_ancestry_choice_option(&conn, 1, 1, "edge");
        insert_test_character_ancestry_choice(&conn, 1, 1, 1, 1);

        // Verify choice exists
        let result = CharacterAncestryChoiceRepository::get_by_id(&conn, 1).unwrap();
        assert!(result.is_some());

        // Delete the character
        conn.execute("DELETE FROM characters WHERE id = 1", [])
            .unwrap();

        // Verify choice was cascade deleted
        let result = CharacterAncestryChoiceRepository::get_by_id(&conn, 1).unwrap();
        assert!(result.is_none());
    }
}
