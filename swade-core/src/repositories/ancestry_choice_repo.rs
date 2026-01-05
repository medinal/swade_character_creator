use crate::error::Result;
use crate::models::AncestryChoice;
use crate::models::AncestryChoiceOption;
use super::base_repository::{query_one_by_id, query_all_ordered, query_by_fk_id};
use rusqlite::{Connection, Row};

pub struct AncestryChoiceRepository;

impl AncestryChoiceRepository {
    const CHOICE_TABLE: &'static str = "ancestry_choices";
    const CHOICE_COLUMNS: &'static str = "id, ancestry_id, choice_type, choice_category,
                                          min_selections, max_selections, description,
                                          created_at, updated_at";

    const OPTION_TABLE: &'static str = "ancestry_choice_options";
    const OPTION_COLUMNS: &'static str = "id, choice_id, option_type, option_id,
                                          option_description, created_at, updated_at";

    /// Get a single ancestry choice by ID
    pub fn get_choice_by_id(conn: &Connection, id: i64) -> Result<Option<AncestryChoice>> {
        query_one_by_id(conn, Self::CHOICE_TABLE, Self::CHOICE_COLUMNS, id, Self::row_to_choice)
    }

    /// Get all ancestry choices ordered by ancestry_id
    pub fn get_all_choices(conn: &Connection) -> Result<Vec<AncestryChoice>> {
        query_all_ordered(conn, Self::CHOICE_TABLE, Self::CHOICE_COLUMNS, "ancestry_id", Self::row_to_choice)
    }

    /// Get all choices for a specific ancestry
    pub fn get_choices_by_ancestry_id(
        conn: &Connection,
        ancestry_id: i64,
    ) -> Result<Vec<AncestryChoice>> {
        query_by_fk_id(conn, Self::CHOICE_TABLE, Self::CHOICE_COLUMNS, "ancestry_id", ancestry_id, "id", Self::row_to_choice)
    }

    /// Get a single ancestry choice option by ID
    pub fn get_option_by_id(conn: &Connection, id: i64) -> Result<Option<AncestryChoiceOption>> {
        query_one_by_id(conn, Self::OPTION_TABLE, Self::OPTION_COLUMNS, id, Self::row_to_option)
    }

    /// Get all options for a specific ancestry choice
    pub fn get_options_by_choice_id(
        conn: &Connection,
        choice_id: i64,
    ) -> Result<Vec<AncestryChoiceOption>> {
        query_by_fk_id(conn, Self::OPTION_TABLE, Self::OPTION_COLUMNS, "choice_id", choice_id, "id", Self::row_to_option)
    }

    /// Convert a database row to an AncestryChoice model
    fn row_to_choice(row: &Row) -> rusqlite::Result<AncestryChoice> {
        Ok(AncestryChoice {
            id: row.get(0)?,
            ancestry_id: row.get(1)?,
            choice_type: row.get(2)?,
            choice_category: row.get(3)?,
            min_selections: row.get(4)?,
            max_selections: row.get(5)?,
            description: row.get(6)?,
            created_at: row.get(7)?,
            updated_at: row.get(8)?,
        })
    }

    /// Convert a database row to an AncestryChoiceOption model
    fn row_to_option(row: &Row) -> rusqlite::Result<AncestryChoiceOption> {
        Ok(AncestryChoiceOption {
            id: row.get(0)?,
            choice_id: row.get(1)?,
            option_type: row.get(2)?,
            option_id: row.get(3)?,
            option_description: row.get(4)?,
            created_at: row.get(5)?,
            updated_at: row.get(6)?,
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

    fn insert_test_ancestry(conn: &Connection, id: i64, name: &str) {
        conn.execute(
            "INSERT INTO ancestries (id, name, source, description, created_at, updated_at)
             VALUES (?, ?, 'core', 'Test ancestry', '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            params![id, name],
        )
        .unwrap();
    }

    fn insert_test_choice(
        conn: &Connection,
        id: i64,
        ancestry_id: i64,
        choice_type: &str,
        choice_category: Option<&str>,
        min_selections: i64,
        max_selections: i64,
        description: &str,
    ) {
        conn.execute(
            "INSERT INTO ancestry_choices (id, ancestry_id, choice_type, choice_category,
                                          min_selections, max_selections, description,
                                          created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            params![
                id,
                ancestry_id,
                choice_type,
                choice_category,
                min_selections,
                max_selections,
                description
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
    ) {
        conn.execute(
            "INSERT INTO ancestry_choice_options (id, choice_id, option_type, option_id,
                                                  option_description, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            params![id, choice_id, option_type, option_id, option_description],
        )
        .unwrap();
    }

    // AncestryChoice tests
    #[test]
    fn test_get_choice_by_id_found() {
        let conn = setup_test_db();
        insert_test_ancestry(&conn, 1, "Human");
        insert_test_choice(
            &conn,
            1,
            1,
            "free_edge",
            Some("background_edge"),
            1,
            1,
            "Choose one Background Edge",
        );

        let result = AncestryChoiceRepository::get_choice_by_id(&conn, 1).unwrap();

        assert!(result.is_some());
        let choice = result.unwrap();
        assert_eq!(choice.id, 1);
        assert_eq!(choice.ancestry_id, 1);
        assert_eq!(choice.choice_type, "free_edge");
        assert_eq!(choice.choice_category, Some("background_edge".to_string()));
        assert_eq!(choice.min_selections, 1);
        assert_eq!(choice.max_selections, 1);
        assert_eq!(choice.description, "Choose one Background Edge");
        assert_eq!(choice.created_at, "2024-01-01 00:00:00");
        assert_eq!(choice.updated_at, "2024-01-01 00:00:00");
    }

    #[test]
    fn test_get_choice_by_id_not_found() {
        let conn = setup_test_db();

        let result = AncestryChoiceRepository::get_choice_by_id(&conn, 999).unwrap();

        assert!(result.is_none());
    }

    #[test]
    fn test_get_choice_with_null_category() {
        let conn = setup_test_db();
        insert_test_ancestry(&conn, 1, "Dwarf");
        insert_test_choice(
            &conn,
            1,
            1,
            "heritage_trait",
            None,
            1,
            1,
            "Choose a heritage trait",
        );

        let result = AncestryChoiceRepository::get_choice_by_id(&conn, 1).unwrap();

        assert!(result.is_some());
        let choice = result.unwrap();
        assert_eq!(choice.choice_category, None);
    }

    #[test]
    fn test_get_all_choices_empty() {
        let conn = setup_test_db();

        let choices = AncestryChoiceRepository::get_all_choices(&conn).unwrap();

        assert_eq!(choices.len(), 0);
    }

    #[test]
    fn test_get_all_choices_single() {
        let conn = setup_test_db();
        insert_test_ancestry(&conn, 1, "Elf");
        insert_test_choice(
            &conn,
            1,
            1,
            "free_edge",
            Some("background_edge"),
            1,
            1,
            "Test choice",
        );

        let choices = AncestryChoiceRepository::get_all_choices(&conn).unwrap();

        assert_eq!(choices.len(), 1);
        assert_eq!(choices[0].choice_type, "free_edge");
    }

    #[test]
    fn test_get_all_choices_multiple_ordered_by_ancestry_id() {
        let conn = setup_test_db();
        insert_test_ancestry(&conn, 1, "Human");
        insert_test_ancestry(&conn, 2, "Dwarf");
        insert_test_ancestry(&conn, 3, "Elf");

        insert_test_choice(&conn, 3, 3, "heritage_trait", None, 1, 1, "Elf choice");
        insert_test_choice(
            &conn,
            1,
            1,
            "free_edge",
            Some("background_edge"),
            1,
            1,
            "Human choice",
        );
        insert_test_choice(
            &conn,
            2,
            2,
            "mandatory_hindrance",
            None,
            1,
            1,
            "Dwarf choice",
        );

        let choices = AncestryChoiceRepository::get_all_choices(&conn).unwrap();

        assert_eq!(choices.len(), 3);
        // Should be ordered by ancestry_id
        assert_eq!(choices[0].ancestry_id, 1);
        assert_eq!(choices[1].ancestry_id, 2);
        assert_eq!(choices[2].ancestry_id, 3);
    }

    #[test]
    fn test_get_all_choices_multiple_per_ancestry() {
        let conn = setup_test_db();
        insert_test_ancestry(&conn, 1, "Half-Elf");

        insert_test_choice(
            &conn,
            1,
            1,
            "free_edge",
            Some("background_edge"),
            1,
            1,
            "First choice",
        );
        insert_test_choice(&conn, 2, 1, "heritage_trait", None, 1, 1, "Second choice");

        let choices = AncestryChoiceRepository::get_all_choices(&conn).unwrap();

        assert_eq!(choices.len(), 2);
        assert_eq!(choices[0].ancestry_id, 1);
        assert_eq!(choices[1].ancestry_id, 1);
    }

    #[test]
    fn test_get_choice_with_min_max_selections() {
        let conn = setup_test_db();
        insert_test_ancestry(&conn, 1, "Test Ancestry");
        insert_test_choice(&conn, 1, 1, "skill_choice", None, 2, 4, "Choose 2-4 skills");

        let result = AncestryChoiceRepository::get_choice_by_id(&conn, 1).unwrap();

        assert!(result.is_some());
        let choice = result.unwrap();
        assert_eq!(choice.min_selections, 2);
        assert_eq!(choice.max_selections, 4);
    }

    #[test]
    fn test_get_choices_by_ancestry_id_empty() {
        let conn = setup_test_db();
        insert_test_ancestry(&conn, 1, "Human");

        let choices = AncestryChoiceRepository::get_choices_by_ancestry_id(&conn, 1).unwrap();

        assert_eq!(choices.len(), 0);
    }

    #[test]
    fn test_get_choices_by_ancestry_id_single() {
        let conn = setup_test_db();
        insert_test_ancestry(&conn, 1, "Human");
        insert_test_choice(
            &conn,
            1,
            1,
            "free_edge",
            Some("background_edge"),
            1,
            1,
            "Test choice",
        );

        let choices = AncestryChoiceRepository::get_choices_by_ancestry_id(&conn, 1).unwrap();

        assert_eq!(choices.len(), 1);
        assert_eq!(choices[0].ancestry_id, 1);
        assert_eq!(choices[0].choice_type, "free_edge");
    }

    #[test]
    fn test_get_choices_by_ancestry_id_multiple() {
        let conn = setup_test_db();
        insert_test_ancestry(&conn, 1, "Human");

        insert_test_choice(&conn, 3, 1, "heritage_trait", None, 1, 1, "Third choice");
        insert_test_choice(
            &conn,
            1,
            1,
            "free_edge",
            Some("background_edge"),
            1,
            1,
            "First choice",
        );
        insert_test_choice(
            &conn,
            2,
            1,
            "mandatory_hindrance",
            None,
            1,
            1,
            "Second choice",
        );

        let choices = AncestryChoiceRepository::get_choices_by_ancestry_id(&conn, 1).unwrap();

        assert_eq!(choices.len(), 3);
        // Should be ordered by id
        assert_eq!(choices[0].id, 1);
        assert_eq!(choices[1].id, 2);
        assert_eq!(choices[2].id, 3);
    }

    #[test]
    fn test_get_choices_by_ancestry_id_filters_correctly() {
        let conn = setup_test_db();
        insert_test_ancestry(&conn, 1, "Human");
        insert_test_ancestry(&conn, 2, "Dwarf");

        insert_test_choice(&conn, 1, 1, "free_edge", None, 1, 1, "Human choice 1");
        insert_test_choice(&conn, 2, 2, "heritage_trait", None, 1, 1, "Dwarf choice");
        insert_test_choice(
            &conn,
            3,
            1,
            "mandatory_hindrance",
            None,
            1,
            1,
            "Human choice 2",
        );

        let choices = AncestryChoiceRepository::get_choices_by_ancestry_id(&conn, 1).unwrap();

        assert_eq!(choices.len(), 2);
        assert_eq!(choices[0].ancestry_id, 1);
        assert_eq!(choices[1].ancestry_id, 1);
    }

    #[test]
    fn test_get_choices_by_ancestry_id_nonexistent_ancestry() {
        let conn = setup_test_db();

        let choices = AncestryChoiceRepository::get_choices_by_ancestry_id(&conn, 999).unwrap();

        assert_eq!(choices.len(), 0);
    }

    // AncestryChoiceOption tests
    #[test]
    fn test_get_option_by_id_found() {
        let conn = setup_test_db();
        insert_test_ancestry(&conn, 1, "Human");
        insert_test_choice(
            &conn,
            1,
            1,
            "free_edge",
            Some("background_edge"),
            1,
            1,
            "Test choice",
        );
        insert_test_option(&conn, 1, 1, "edge", Some(42), Some("Specific edge option"));

        let result = AncestryChoiceRepository::get_option_by_id(&conn, 1).unwrap();

        assert!(result.is_some());
        let option = result.unwrap();
        assert_eq!(option.id, 1);
        assert_eq!(option.choice_id, 1);
        assert_eq!(option.option_type, "edge");
        assert_eq!(option.option_id, Some(42));
        assert_eq!(
            option.option_description,
            Some("Specific edge option".to_string())
        );
        assert_eq!(option.created_at, "2024-01-01 00:00:00");
        assert_eq!(option.updated_at, "2024-01-01 00:00:00");
    }

    #[test]
    fn test_get_option_by_id_not_found() {
        let conn = setup_test_db();

        let result = AncestryChoiceRepository::get_option_by_id(&conn, 999).unwrap();

        assert!(result.is_none());
    }

    #[test]
    fn test_get_option_with_null_values() {
        let conn = setup_test_db();
        insert_test_ancestry(&conn, 1, "Human");
        insert_test_choice(&conn, 1, 1, "free_edge", None, 1, 1, "Test choice");
        insert_test_option(&conn, 1, 1, "edge", None, None);

        let result = AncestryChoiceRepository::get_option_by_id(&conn, 1).unwrap();

        assert!(result.is_some());
        let option = result.unwrap();
        assert_eq!(option.option_id, None);
        assert_eq!(option.option_description, None);
    }

    #[test]
    fn test_get_options_by_choice_id_empty() {
        let conn = setup_test_db();
        insert_test_ancestry(&conn, 1, "Human");
        insert_test_choice(&conn, 1, 1, "free_edge", None, 1, 1, "Test choice");

        let options = AncestryChoiceRepository::get_options_by_choice_id(&conn, 1).unwrap();

        assert_eq!(options.len(), 0);
    }

    #[test]
    fn test_get_options_by_choice_id_single() {
        let conn = setup_test_db();
        insert_test_ancestry(&conn, 1, "Human");
        insert_test_choice(&conn, 1, 1, "free_edge", None, 1, 1, "Test choice");
        insert_test_option(&conn, 1, 1, "edge", Some(10), Some("Test option"));

        let options = AncestryChoiceRepository::get_options_by_choice_id(&conn, 1).unwrap();

        assert_eq!(options.len(), 1);
        assert_eq!(options[0].option_type, "edge");
    }

    #[test]
    fn test_get_options_by_choice_id_multiple() {
        let conn = setup_test_db();
        insert_test_ancestry(&conn, 1, "Human");
        insert_test_choice(&conn, 1, 1, "free_edge", None, 1, 3, "Test choice");

        insert_test_option(&conn, 3, 1, "modifier", Some(30), Some("Third option"));
        insert_test_option(&conn, 1, 1, "edge", Some(10), Some("First option"));
        insert_test_option(&conn, 2, 1, "hindrance", Some(20), Some("Second option"));

        let options = AncestryChoiceRepository::get_options_by_choice_id(&conn, 1).unwrap();

        assert_eq!(options.len(), 3);
        // Should be ordered by id
        assert_eq!(options[0].id, 1);
        assert_eq!(options[1].id, 2);
        assert_eq!(options[2].id, 3);
        assert_eq!(options[0].option_type, "edge");
        assert_eq!(options[1].option_type, "hindrance");
        assert_eq!(options[2].option_type, "modifier");
    }

    #[test]
    fn test_get_options_by_choice_id_filters_correctly() {
        let conn = setup_test_db();
        insert_test_ancestry(&conn, 1, "Human");
        insert_test_choice(&conn, 1, 1, "free_edge", None, 1, 1, "First choice");
        insert_test_choice(&conn, 2, 1, "heritage_trait", None, 1, 1, "Second choice");

        insert_test_option(&conn, 1, 1, "edge", Some(10), None);
        insert_test_option(&conn, 2, 2, "modifier", Some(20), None);
        insert_test_option(&conn, 3, 1, "edge", Some(11), None);

        let options = AncestryChoiceRepository::get_options_by_choice_id(&conn, 1).unwrap();

        assert_eq!(options.len(), 2);
        assert_eq!(options[0].choice_id, 1);
        assert_eq!(options[1].choice_id, 1);
    }

    #[test]
    fn test_get_options_by_choice_id_nonexistent_choice() {
        let conn = setup_test_db();

        let options = AncestryChoiceRepository::get_options_by_choice_id(&conn, 999).unwrap();

        assert_eq!(options.len(), 0);
    }

    #[test]
    fn test_option_types() {
        let conn = setup_test_db();
        insert_test_ancestry(&conn, 1, "Test");
        insert_test_choice(&conn, 1, 1, "free_choice", None, 1, 3, "Test");

        insert_test_option(&conn, 1, 1, "edge", Some(1), None);
        insert_test_option(&conn, 2, 1, "hindrance", Some(2), None);
        insert_test_option(&conn, 3, 1, "modifier", Some(3), None);

        let options = AncestryChoiceRepository::get_options_by_choice_id(&conn, 1).unwrap();

        assert_eq!(options.len(), 3);
        let option_types: Vec<String> = options.iter().map(|o| o.option_type.clone()).collect();
        assert!(option_types.contains(&"edge".to_string()));
        assert!(option_types.contains(&"hindrance".to_string()));
        assert!(option_types.contains(&"modifier".to_string()));
    }
}
