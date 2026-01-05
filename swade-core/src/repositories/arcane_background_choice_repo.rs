use crate::error::Result;
use crate::models::ArcaneBackgroundChoice;
use crate::models::ArcaneBackgroundChoiceOption;
use super::base_repository::{query_one_by_id, query_all_ordered, query_by_fk_id};
use rusqlite::{Connection, Row};

pub struct ArcaneBackgroundChoiceRepository;

impl ArcaneBackgroundChoiceRepository {
    const CHOICE_TABLE: &'static str = "arcane_background_choices";
    const CHOICE_COLUMNS: &'static str = "id, arcane_background_id, choice_type, choice_category,
                                          min_selections, max_selections, description, position,
                                          created_at, updated_at";

    const OPTION_TABLE: &'static str = "arcane_background_choice_options";
    const OPTION_COLUMNS: &'static str = "id, choice_id, option_type, option_id,
                                          option_description, position, created_at, updated_at";

    /// Get a single arcane background choice by ID
    pub fn get_choice_by_id(conn: &Connection, id: i64) -> Result<Option<ArcaneBackgroundChoice>> {
        query_one_by_id(conn, Self::CHOICE_TABLE, Self::CHOICE_COLUMNS, id, Self::row_to_choice)
    }

    /// Get all arcane background choices ordered by arcane_background_id
    pub fn get_all_choices(conn: &Connection) -> Result<Vec<ArcaneBackgroundChoice>> {
        query_all_ordered(conn, Self::CHOICE_TABLE, Self::CHOICE_COLUMNS, "arcane_background_id, position", Self::row_to_choice)
    }

    /// Get all choices for a specific arcane background
    pub fn get_choices_by_arcane_background_id(
        conn: &Connection,
        arcane_background_id: i64,
    ) -> Result<Vec<ArcaneBackgroundChoice>> {
        query_by_fk_id(conn, Self::CHOICE_TABLE, Self::CHOICE_COLUMNS, "arcane_background_id", arcane_background_id, "position", Self::row_to_choice)
    }

    /// Get a single arcane background choice option by ID
    pub fn get_option_by_id(conn: &Connection, id: i64) -> Result<Option<ArcaneBackgroundChoiceOption>> {
        query_one_by_id(conn, Self::OPTION_TABLE, Self::OPTION_COLUMNS, id, Self::row_to_option)
    }

    /// Get all options for a specific arcane background choice
    pub fn get_options_by_choice_id(
        conn: &Connection,
        choice_id: i64,
    ) -> Result<Vec<ArcaneBackgroundChoiceOption>> {
        query_by_fk_id(conn, Self::OPTION_TABLE, Self::OPTION_COLUMNS, "choice_id", choice_id, "position", Self::row_to_option)
    }

    /// Convert a database row to an ArcaneBackgroundChoice model
    fn row_to_choice(row: &Row) -> rusqlite::Result<ArcaneBackgroundChoice> {
        Ok(ArcaneBackgroundChoice {
            id: row.get(0)?,
            arcane_background_id: row.get(1)?,
            choice_type: row.get(2)?,
            choice_category: row.get(3)?,
            min_selections: row.get(4)?,
            max_selections: row.get(5)?,
            description: row.get(6)?,
            position: row.get(7)?,
            created_at: row.get(8)?,
            updated_at: row.get(9)?,
        })
    }

    /// Convert a database row to an ArcaneBackgroundChoiceOption model
    fn row_to_option(row: &Row) -> rusqlite::Result<ArcaneBackgroundChoiceOption> {
        Ok(ArcaneBackgroundChoiceOption {
            id: row.get(0)?,
            choice_id: row.get(1)?,
            option_type: row.get(2)?,
            option_id: row.get(3)?,
            option_description: row.get(4)?,
            position: row.get(5)?,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::{Connection, params};

    fn setup_test_db() -> Connection {
        let conn = Connection::open_in_memory().unwrap();
        let schema = include_str!("../../../db/schema.sql");
        conn.execute_batch(schema).unwrap();
        conn
    }

    fn insert_test_skill(conn: &Connection, id: i64, name: &str) {
        // Insert attribute first for foreign key
        conn.execute(
            "INSERT OR IGNORE INTO attributes (id, name, description) VALUES (1, 'Smarts', 'Mental aptitude')",
            [],
        ).unwrap();
        conn.execute(
            "INSERT INTO skills (id, name, linked_attribute_id, description)
             VALUES (?, ?, 1, 'Test skill')",
            params![id, name],
        ).unwrap();
    }

    fn insert_test_arcane_background(conn: &Connection, id: i64, name: &str) {
        insert_test_skill(&conn, 28, "Spellcasting");
        conn.execute(
            "INSERT INTO arcane_backgrounds (id, name, arcane_skill_id, starting_powers, starting_power_points, has_power_list, source, description, created_at, updated_at)
             VALUES (?, ?, 28, 3, 10, 1, 'fantasy_companion', 'Test arcane background', '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            params![id, name],
        )
        .unwrap();
    }

    fn insert_test_choice(
        conn: &Connection,
        id: i64,
        arcane_background_id: i64,
        choice_type: &str,
        choice_category: Option<&str>,
        min_selections: i64,
        max_selections: i64,
        description: &str,
        position: i64,
    ) {
        conn.execute(
            "INSERT INTO arcane_background_choices (id, arcane_background_id, choice_type, choice_category,
                                          min_selections, max_selections, description, position,
                                          created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            params![
                id,
                arcane_background_id,
                choice_type,
                choice_category,
                min_selections,
                max_selections,
                description,
                position
            ],
        )
        .unwrap();
    }

    fn insert_test_option(
        conn: &Connection,
        id: i64,
        choice_id: i64,
        option_type: &str,
        option_id: Option<i64>,
        option_description: Option<&str>,
        position: i64,
    ) {
        conn.execute(
            "INSERT INTO arcane_background_choice_options (id, choice_id, option_type, option_id,
                                                  option_description, position, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            params![id, choice_id, option_type, option_id, option_description, position],
        )
        .unwrap();
    }

    // ArcaneBackgroundChoice tests
    #[test]
    fn test_get_choice_by_id_found() {
        let conn = setup_test_db();
        insert_test_arcane_background(&conn, 1, "Wizard");
        insert_test_choice(
            &conn,
            1,
            1,
            "available_power",
            None,
            0,
            55,
            "Powers available to the Wizard",
            0,
        );

        let result = ArcaneBackgroundChoiceRepository::get_choice_by_id(&conn, 1).unwrap();

        assert!(result.is_some());
        let choice = result.unwrap();
        assert_eq!(choice.id, 1);
        assert_eq!(choice.arcane_background_id, 1);
        assert_eq!(choice.choice_type, "available_power");
        assert_eq!(choice.choice_category, None);
        assert_eq!(choice.min_selections, 0);
        assert_eq!(choice.max_selections, 55);
        assert_eq!(choice.description, "Powers available to the Wizard");
        assert_eq!(choice.position, 0);
    }

    #[test]
    fn test_get_choice_by_id_not_found() {
        let conn = setup_test_db();

        let result = ArcaneBackgroundChoiceRepository::get_choice_by_id(&conn, 999).unwrap();

        assert!(result.is_none());
    }

    #[test]
    fn test_get_choices_by_arcane_background_id() {
        let conn = setup_test_db();
        insert_test_arcane_background(&conn, 1, "Wizard");
        insert_test_choice(&conn, 1, 1, "available_power", None, 0, 55, "Available powers", 0);
        insert_test_choice(&conn, 2, 1, "required_starting_power", None, 3, 3, "Required powers", 1);
        insert_test_choice(&conn, 3, 1, "built_in_hindrance", None, 1, 1, "Armor Interference", 2);

        let choices = ArcaneBackgroundChoiceRepository::get_choices_by_arcane_background_id(&conn, 1).unwrap();

        assert_eq!(choices.len(), 3);
        // Should be ordered by position
        assert_eq!(choices[0].position, 0);
        assert_eq!(choices[1].position, 1);
        assert_eq!(choices[2].position, 2);
    }

    #[test]
    fn test_get_choices_by_arcane_background_id_empty() {
        let conn = setup_test_db();
        insert_test_arcane_background(&conn, 1, "Wizard");

        let choices = ArcaneBackgroundChoiceRepository::get_choices_by_arcane_background_id(&conn, 1).unwrap();

        assert_eq!(choices.len(), 0);
    }

    // ArcaneBackgroundChoiceOption tests
    #[test]
    fn test_get_option_by_id_found() {
        let conn = setup_test_db();
        insert_test_arcane_background(&conn, 1, "Wizard");
        insert_test_choice(&conn, 1, 1, "built_in_hindrance", None, 1, 1, "Test choice", 0);
        insert_test_option(&conn, 1, 1, "hindrance", Some(85), Some("Armor Interference (Major)"), 0);

        let result = ArcaneBackgroundChoiceRepository::get_option_by_id(&conn, 1).unwrap();

        assert!(result.is_some());
        let option = result.unwrap();
        assert_eq!(option.id, 1);
        assert_eq!(option.choice_id, 1);
        assert_eq!(option.option_type, "hindrance");
        assert_eq!(option.option_id, Some(85));
        assert_eq!(option.option_description, Some("Armor Interference (Major)".to_string()));
        assert_eq!(option.position, 0);
    }

    #[test]
    fn test_get_option_by_id_not_found() {
        let conn = setup_test_db();

        let result = ArcaneBackgroundChoiceRepository::get_option_by_id(&conn, 999).unwrap();

        assert!(result.is_none());
    }

    #[test]
    fn test_get_options_by_choice_id() {
        let conn = setup_test_db();
        insert_test_arcane_background(&conn, 1, "Wizard");
        insert_test_choice(&conn, 1, 1, "available_power", None, 0, 55, "Available powers", 0);
        insert_test_option(&conn, 1, 1, "power", Some(1), None, 0);
        insert_test_option(&conn, 2, 1, "power", Some(2), None, 1);
        insert_test_option(&conn, 3, 1, "power", Some(3), None, 2);

        let options = ArcaneBackgroundChoiceRepository::get_options_by_choice_id(&conn, 1).unwrap();

        assert_eq!(options.len(), 3);
        // Should be ordered by position
        assert_eq!(options[0].position, 0);
        assert_eq!(options[1].position, 1);
        assert_eq!(options[2].position, 2);
    }

    #[test]
    fn test_get_options_by_choice_id_empty() {
        let conn = setup_test_db();
        insert_test_arcane_background(&conn, 1, "Wizard");
        insert_test_choice(&conn, 1, 1, "available_power", None, 0, 55, "Available powers", 0);

        let options = ArcaneBackgroundChoiceRepository::get_options_by_choice_id(&conn, 1).unwrap();

        assert_eq!(options.len(), 0);
    }

    #[test]
    fn test_option_with_null_values() {
        let conn = setup_test_db();
        insert_test_arcane_background(&conn, 1, "Wizard");
        insert_test_choice(&conn, 1, 1, "special_ability", None, 0, 0, "Test ability", 0);
        insert_test_option(&conn, 1, 1, "ability", None, Some("Can cast Magic edges"), 0);

        let result = ArcaneBackgroundChoiceRepository::get_option_by_id(&conn, 1).unwrap();

        assert!(result.is_some());
        let option = result.unwrap();
        assert_eq!(option.option_id, None);
        assert_eq!(option.option_description, Some("Can cast Magic edges".to_string()));
    }
}
