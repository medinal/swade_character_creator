use crate::error::Result;
use crate::models::CharacterArcaneBackgroundChoice;
use rusqlite::{Connection, Row, params};

pub struct CharacterArcaneBackgroundChoiceRepository;

impl CharacterArcaneBackgroundChoiceRepository {
    /// Get a single character arcane background choice by ID
    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<CharacterArcaneBackgroundChoice>> {
        let mut stmt = conn.prepare(
            "SELECT id, character_id, choice_id, selected_option_id,
                    created_at, updated_at
             FROM character_arcane_background_choices WHERE id = ?",
        )?;

        let mut rows = stmt.query(params![id])?;

        match rows.next()? {
            Some(row) => Ok(Some(Self::row_to_model(row)?)),
            None => Ok(None),
        }
    }

    /// Get all character arcane background choices for a specific character
    pub fn get_by_character_id(
        conn: &Connection,
        character_id: i64,
    ) -> Result<Vec<CharacterArcaneBackgroundChoice>> {
        let mut stmt = conn.prepare(
            "SELECT id, character_id, choice_id, selected_option_id,
                    created_at, updated_at
             FROM character_arcane_background_choices WHERE character_id = ? ORDER BY id",
        )?;

        let rows = stmt.query_map(params![character_id], Self::row_to_model)?;

        Ok(rows.filter_map(|r| r.ok()).collect())
    }

    /// Insert a new character arcane background choice, returns the generated ID
    pub fn insert(conn: &Connection, model: &CharacterArcaneBackgroundChoice) -> Result<i64> {
        conn.execute(
            "INSERT INTO character_arcane_background_choices (character_id, choice_id, selected_option_id,
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

    /// Update an existing character arcane background choice
    pub fn update(conn: &Connection, model: &CharacterArcaneBackgroundChoice) -> Result<()> {
        conn.execute(
            "UPDATE character_arcane_background_choices
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

    /// Delete a character arcane background choice by ID
    pub fn delete(conn: &Connection, id: i64) -> Result<()> {
        conn.execute(
            "DELETE FROM character_arcane_background_choices WHERE id = ?",
            params![id],
        )?;
        Ok(())
    }

    /// Delete all arcane background choices for a character
    pub fn delete_by_character_id(conn: &Connection, character_id: i64) -> Result<()> {
        conn.execute(
            "DELETE FROM character_arcane_background_choices WHERE character_id = ?",
            params![character_id],
        )?;
        Ok(())
    }

    /// Convert a database row to a CharacterArcaneBackgroundChoice model
    fn row_to_model(row: &Row) -> rusqlite::Result<CharacterArcaneBackgroundChoice> {
        Ok(CharacterArcaneBackgroundChoice {
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

        // Insert attribute for skill foreign key
        conn.execute(
            "INSERT INTO attributes (id, name, description) VALUES (1, 'Smarts', 'Mental aptitude')",
            [],
        ).unwrap();

        // Insert skill for arcane background foreign key
        conn.execute(
            "INSERT INTO skills (id, name, linked_attribute_id, description)
             VALUES (28, 'Spellcasting', 1, 'Cast spells')",
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

    fn insert_test_arcane_background(conn: &Connection, id: i64, name: &str) {
        conn.execute(
            "INSERT INTO arcane_backgrounds (id, name, arcane_skill_id, starting_powers, starting_power_points, has_power_list, source, description)
             VALUES (?, ?, 28, 6, 15, 1, 'fantasy_companion', 'Test AB')",
            params![id, name],
        ).unwrap();
    }

    fn insert_test_arcane_background_choice(
        conn: &Connection,
        id: i64,
        arcane_background_id: i64,
        choice_type: &str,
    ) {
        conn.execute(
            "INSERT INTO arcane_background_choices (id, arcane_background_id, choice_type, min_selections, max_selections,
                                          description, position, created_at, updated_at)
             VALUES (?, ?, ?, 1, 1, 'Test choice description', 0,
                     '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            params![id, arcane_background_id, choice_type],
        ).unwrap();
    }

    fn insert_test_arcane_background_choice_option(
        conn: &Connection,
        id: i64,
        choice_id: i64,
        option_type: &str,
    ) {
        conn.execute(
            "INSERT INTO arcane_background_choice_options (id, choice_id, option_type, option_description, position,
                                                  created_at, updated_at)
             VALUES (?, ?, ?, 'Test option description', 0,
                     '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            params![id, choice_id, option_type],
        )
        .unwrap();
    }

    fn insert_test_character_arcane_background_choice(
        conn: &Connection,
        id: i64,
        character_id: i64,
        choice_id: i64,
        selected_option_id: i64,
    ) {
        conn.execute(
            "INSERT INTO character_arcane_background_choices (id, character_id, choice_id, selected_option_id,
                                                     created_at, updated_at)
             VALUES (?, ?, ?, ?, '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            params![id, character_id, choice_id, selected_option_id],
        ).unwrap();
    }

    #[test]
    fn test_get_by_id_found() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_arcane_background(&conn, 1, "Wizard");
        insert_test_arcane_background_choice(&conn, 1, 1, "built_in_hindrance");
        insert_test_arcane_background_choice_option(&conn, 1, 1, "hindrance");
        insert_test_character_arcane_background_choice(&conn, 1, 1, 1, 1);

        let result = CharacterArcaneBackgroundChoiceRepository::get_by_id(&conn, 1).unwrap();

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

        let result = CharacterArcaneBackgroundChoiceRepository::get_by_id(&conn, 999).unwrap();

        assert!(result.is_none());
    }

    #[test]
    fn test_get_by_character_id_empty() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");

        let choices = CharacterArcaneBackgroundChoiceRepository::get_by_character_id(&conn, 1).unwrap();

        assert_eq!(choices.len(), 0);
    }

    #[test]
    fn test_get_by_character_id_single_choice() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_arcane_background(&conn, 1, "Wizard");
        insert_test_arcane_background_choice(&conn, 1, 1, "built_in_hindrance");
        insert_test_arcane_background_choice_option(&conn, 1, 1, "hindrance");
        insert_test_character_arcane_background_choice(&conn, 1, 1, 1, 1);

        let choices = CharacterArcaneBackgroundChoiceRepository::get_by_character_id(&conn, 1).unwrap();

        assert_eq!(choices.len(), 1);
        assert_eq!(choices[0].character_id, 1);
        assert_eq!(choices[0].choice_id, 1);
        assert_eq!(choices[0].selected_option_id, 1);
    }

    #[test]
    fn test_get_by_character_id_multiple_choices() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_arcane_background(&conn, 1, "Wizard");
        insert_test_arcane_background_choice(&conn, 1, 1, "built_in_hindrance");
        insert_test_arcane_background_choice(&conn, 2, 1, "special_ability");
        insert_test_arcane_background_choice_option(&conn, 1, 1, "hindrance");
        insert_test_arcane_background_choice_option(&conn, 2, 2, "ability");
        insert_test_character_arcane_background_choice(&conn, 1, 1, 1, 1);
        insert_test_character_arcane_background_choice(&conn, 2, 1, 2, 2);

        let choices = CharacterArcaneBackgroundChoiceRepository::get_by_character_id(&conn, 1).unwrap();

        assert_eq!(choices.len(), 2);
        assert_eq!(choices[0].choice_id, 1);
        assert_eq!(choices[1].choice_id, 2);
    }

    #[test]
    fn test_insert_basic() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_arcane_background(&conn, 1, "Wizard");
        insert_test_arcane_background_choice(&conn, 1, 1, "built_in_hindrance");
        insert_test_arcane_background_choice_option(&conn, 1, 1, "hindrance");

        let choice = CharacterArcaneBackgroundChoice {
            id: 0, // Will be set by database
            character_id: 1,
            choice_id: 1,
            selected_option_id: 1,
            created_at: "2024-01-01 00:00:00".to_string(),
            updated_at: "2024-01-01 00:00:00".to_string(),
        };

        let id = CharacterArcaneBackgroundChoiceRepository::insert(&conn, &choice).unwrap();

        assert!(id > 0);
        let retrieved = CharacterArcaneBackgroundChoiceRepository::get_by_id(&conn, id).unwrap();
        assert!(retrieved.is_some());
        let retrieved_choice = retrieved.unwrap();
        assert_eq!(retrieved_choice.character_id, 1);
        assert_eq!(retrieved_choice.choice_id, 1);
        assert_eq!(retrieved_choice.selected_option_id, 1);
    }

    #[test]
    fn test_update_selected_option() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_arcane_background(&conn, 1, "Wizard");
        insert_test_arcane_background_choice(&conn, 1, 1, "choosable_starting_power");
        insert_test_arcane_background_choice_option(&conn, 1, 1, "power");
        insert_test_arcane_background_choice_option(&conn, 2, 1, "power");
        insert_test_character_arcane_background_choice(&conn, 1, 1, 1, 1);

        let mut choice = CharacterArcaneBackgroundChoiceRepository::get_by_id(&conn, 1)
            .unwrap()
            .unwrap();
        choice.selected_option_id = 2;
        choice.updated_at = "2024-01-02 00:00:00".to_string();

        CharacterArcaneBackgroundChoiceRepository::update(&conn, &choice).unwrap();

        let updated = CharacterArcaneBackgroundChoiceRepository::get_by_id(&conn, 1)
            .unwrap()
            .unwrap();
        assert_eq!(updated.selected_option_id, 2);
        assert_eq!(updated.updated_at, "2024-01-02 00:00:00");
    }

    #[test]
    fn test_delete_basic() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_arcane_background(&conn, 1, "Wizard");
        insert_test_arcane_background_choice(&conn, 1, 1, "built_in_hindrance");
        insert_test_arcane_background_choice_option(&conn, 1, 1, "hindrance");
        insert_test_character_arcane_background_choice(&conn, 1, 1, 1, 1);

        CharacterArcaneBackgroundChoiceRepository::delete(&conn, 1).unwrap();

        let result = CharacterArcaneBackgroundChoiceRepository::get_by_id(&conn, 1).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_delete_by_character_id() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_arcane_background(&conn, 1, "Wizard");
        insert_test_arcane_background_choice(&conn, 1, 1, "built_in_hindrance");
        insert_test_arcane_background_choice(&conn, 2, 1, "special_ability");
        insert_test_arcane_background_choice_option(&conn, 1, 1, "hindrance");
        insert_test_arcane_background_choice_option(&conn, 2, 2, "ability");
        insert_test_character_arcane_background_choice(&conn, 1, 1, 1, 1);
        insert_test_character_arcane_background_choice(&conn, 2, 1, 2, 2);

        CharacterArcaneBackgroundChoiceRepository::delete_by_character_id(&conn, 1).unwrap();

        let choices = CharacterArcaneBackgroundChoiceRepository::get_by_character_id(&conn, 1).unwrap();
        assert_eq!(choices.len(), 0);
    }

    #[test]
    fn test_cascade_delete_when_character_deleted() {
        let conn = setup_test_db();
        insert_test_character(&conn, 1, "Test Character");
        insert_test_arcane_background(&conn, 1, "Wizard");
        insert_test_arcane_background_choice(&conn, 1, 1, "built_in_hindrance");
        insert_test_arcane_background_choice_option(&conn, 1, 1, "hindrance");
        insert_test_character_arcane_background_choice(&conn, 1, 1, 1, 1);

        // Verify choice exists
        let result = CharacterArcaneBackgroundChoiceRepository::get_by_id(&conn, 1).unwrap();
        assert!(result.is_some());

        // Delete the character
        conn.execute("DELETE FROM characters WHERE id = 1", [])
            .unwrap();

        // Verify choice was cascade deleted
        let result = CharacterArcaneBackgroundChoiceRepository::get_by_id(&conn, 1).unwrap();
        assert!(result.is_none());
    }
}
