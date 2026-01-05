use rusqlite::{Connection, Row, params};

use crate::error::Result;
use crate::models::Character;

pub struct CharacterRepository;

impl CharacterRepository {
    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<Character>> {
        let mut stmt = conn.prepare(
            "SELECT id, is_wild_card, name, ancestry_id,
                    attribute_points_spent, attribute_points_earned,
                    skill_points_spent, skill_points_earned,
                    hindrance_points_spent, hindrance_points_earned,
                    hindrance_points_to_edges, hindrance_points_to_attributes,
                    hindrance_points_to_skills, hindrance_points_to_wealth,
                    power_points, power_points_used, wounds, fatigue,
                    wealth, background, description,
                    portrait, portrait_mime_type,
                    created_at, updated_at
             FROM characters WHERE id = ?",
        )?;

        let mut rows = stmt.query(params![id])?;

        match rows.next()? {
            Some(row) => Ok(Some(Self::row_to_model(row)?)),
            None => Ok(None),
        }
    }

    pub fn get_all(conn: &Connection) -> Result<Vec<Character>> {
        let mut stmt = conn.prepare(
            "SELECT id, is_wild_card, name, ancestry_id,
                    attribute_points_spent, attribute_points_earned,
                    skill_points_spent, skill_points_earned,
                    hindrance_points_spent, hindrance_points_earned,
                    hindrance_points_to_edges, hindrance_points_to_attributes,
                    hindrance_points_to_skills, hindrance_points_to_wealth,
                    power_points, power_points_used, wounds, fatigue,
                    wealth, background, description,
                    portrait, portrait_mime_type,
                    created_at, updated_at
             FROM characters ORDER BY name",
        )?;

        let rows = stmt.query_map([], Self::row_to_model)?;

        Ok(rows.filter_map(|r| r.ok()).collect())
    }

    pub fn insert(conn: &Connection, character: &Character) -> Result<i64> {
        conn.execute(
            "INSERT INTO characters (
                is_wild_card, name, ancestry_id,
                attribute_points_spent, attribute_points_earned,
                skill_points_spent, skill_points_earned,
                hindrance_points_spent, hindrance_points_earned,
                hindrance_points_to_edges, hindrance_points_to_attributes,
                hindrance_points_to_skills, hindrance_points_to_wealth,
                power_points, power_points_used, wounds, fatigue,
                wealth, background, description,
                portrait, portrait_mime_type
             ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13, ?14, ?15, ?16, ?17, ?18, ?19, ?20, ?21, ?22)",
            params![
                character.is_wild_card,
                character.name,
                character.ancestry_id,
                character.attribute_points_spent,
                character.attribute_points_earned,
                character.skill_points_spent,
                character.skill_points_earned,
                character.hindrance_points_spent,
                character.hindrance_points_earned,
                character.hindrance_points_to_edges,
                character.hindrance_points_to_attributes,
                character.hindrance_points_to_skills,
                character.hindrance_points_to_wealth,
                character.power_points,
                character.power_points_used,
                character.wounds,
                character.fatigue,
                character.wealth,
                character.background,
                character.description,
                character.portrait,
                character.portrait_mime_type,
            ],
        )?;

        Ok(conn.last_insert_rowid())
    }

    pub fn update(conn: &Connection, character: &Character) -> Result<()> {
        conn.execute(
            "UPDATE characters SET
                is_wild_card = ?1,
                name = ?2,
                ancestry_id = ?3,
                attribute_points_spent = ?4,
                attribute_points_earned = ?5,
                skill_points_spent = ?6,
                skill_points_earned = ?7,
                hindrance_points_spent = ?8,
                hindrance_points_earned = ?9,
                hindrance_points_to_edges = ?10,
                hindrance_points_to_attributes = ?11,
                hindrance_points_to_skills = ?12,
                hindrance_points_to_wealth = ?13,
                power_points = ?14,
                power_points_used = ?15,
                wounds = ?16,
                fatigue = ?17,
                wealth = ?18,
                background = ?19,
                description = ?20,
                updated_at = CURRENT_TIMESTAMP
             WHERE id = ?21",
            params![
                character.is_wild_card,
                character.name,
                character.ancestry_id,
                character.attribute_points_spent,
                character.attribute_points_earned,
                character.skill_points_spent,
                character.skill_points_earned,
                character.hindrance_points_spent,
                character.hindrance_points_earned,
                character.hindrance_points_to_edges,
                character.hindrance_points_to_attributes,
                character.hindrance_points_to_skills,
                character.hindrance_points_to_wealth,
                character.power_points,
                character.power_points_used,
                character.wounds,
                character.fatigue,
                character.wealth,
                character.background,
                character.description,
                character.id,
            ],
        )?;

        Ok(())
    }

    pub fn delete(conn: &Connection, id: i64) -> Result<()> {
        conn.execute("DELETE FROM characters WHERE id = ?", params![id])?;
        Ok(())
    }

    /// Update only the portrait fields for a character.
    /// Pass None for both to clear the portrait.
    pub fn update_portrait(
        conn: &Connection,
        id: i64,
        portrait: Option<&[u8]>,
        mime_type: Option<&str>,
    ) -> Result<()> {
        conn.execute(
            "UPDATE characters SET portrait = ?1, portrait_mime_type = ?2, updated_at = CURRENT_TIMESTAMP WHERE id = ?3",
            params![portrait, mime_type, id],
        )?;
        Ok(())
    }

    fn row_to_model(row: &Row) -> rusqlite::Result<Character> {
        Ok(Character {
            id: row.get(0)?,
            is_wild_card: row.get(1)?,
            name: row.get(2)?,
            ancestry_id: row.get(3)?,
            attribute_points_spent: row.get(4)?,
            attribute_points_earned: row.get(5)?,
            skill_points_spent: row.get(6)?,
            skill_points_earned: row.get(7)?,
            hindrance_points_spent: row.get(8)?,
            hindrance_points_earned: row.get(9)?,
            hindrance_points_to_edges: row.get(10)?,
            hindrance_points_to_attributes: row.get(11)?,
            hindrance_points_to_skills: row.get(12)?,
            hindrance_points_to_wealth: row.get(13)?,
            power_points: row.get(14)?,
            power_points_used: row.get(15)?,
            wounds: row.get(16)?,
            fatigue: row.get(17)?,
            wealth: row.get(18)?,
            background: row.get(19)?,
            description: row.get(20)?,
            portrait: row.get(21)?,
            portrait_mime_type: row.get(22)?,
            created_at: row.get(23)?,
            updated_at: row.get(24)?,
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

    fn create_test_character() -> Character {
        Character {
            id: 0, // Will be set by database
            is_wild_card: true,
            name: "Test Hero".to_string(),
            ancestry_id: Some(1),
            attribute_points_spent: 3,
            attribute_points_earned: 5,
            skill_points_spent: 8,
            skill_points_earned: 12,
            hindrance_points_spent: 0,
            hindrance_points_earned: 4,
            hindrance_points_to_edges: 2,
            hindrance_points_to_attributes: 0,
            hindrance_points_to_skills: 2,
            hindrance_points_to_wealth: 0,
            power_points: 10,
            power_points_used: 0,
            wounds: 0,
            fatigue: 0,
            wealth: 500,
            background: Some("A brave adventurer".to_string()),
            description: Some("Tall and strong".to_string()),
            portrait: None,
            portrait_mime_type: None,
            created_at: "2024-01-01 00:00:00".to_string(),
            updated_at: "2024-01-01 00:00:00".to_string(),
        }
    }

    fn insert_test_character(conn: &Connection) -> i64 {
        let character = create_test_character();
        CharacterRepository::insert(conn, &character).unwrap()
    }

    #[test]
    fn test_insert() {
        let conn = setup_test_db();
        let character = create_test_character();

        let id = CharacterRepository::insert(&conn, &character).unwrap();

        assert!(id > 0);

        // Verify the character was inserted
        let result = CharacterRepository::get_by_id(&conn, id).unwrap();
        assert!(result.is_some());
        let saved = result.unwrap();
        assert_eq!(saved.name, "Test Hero");
        assert!(saved.is_wild_card);
        assert_eq!(saved.ancestry_id, Some(1));
    }

    #[test]
    fn test_insert_with_null_fields() {
        let conn = setup_test_db();
        let mut character = create_test_character();
        character.ancestry_id = None;
        character.background = None;
        character.description = None;

        let id = CharacterRepository::insert(&conn, &character).unwrap();

        let result = CharacterRepository::get_by_id(&conn, id).unwrap();
        assert!(result.is_some());
        let saved = result.unwrap();
        assert_eq!(saved.ancestry_id, None);
        assert_eq!(saved.background, None);
        assert_eq!(saved.description, None);
    }

    #[test]
    fn test_get_by_id_found() {
        let conn = setup_test_db();
        let id = insert_test_character(&conn);

        let result = CharacterRepository::get_by_id(&conn, id).unwrap();

        assert!(result.is_some());
        let character = result.unwrap();
        assert_eq!(character.id, id);
        assert_eq!(character.name, "Test Hero");
        assert!(character.is_wild_card);
        assert_eq!(character.ancestry_id, Some(1));
        assert_eq!(character.attribute_points_spent, 3);
        assert_eq!(character.attribute_points_earned, 5);
        assert_eq!(character.skill_points_spent, 8);
        assert_eq!(character.skill_points_earned, 12);
        assert_eq!(character.hindrance_points_spent, 0);
        assert_eq!(character.hindrance_points_earned, 4);
        assert_eq!(character.hindrance_points_to_edges, 2);
        assert_eq!(character.hindrance_points_to_attributes, 0);
        assert_eq!(character.hindrance_points_to_skills, 2);
        assert_eq!(character.hindrance_points_to_wealth, 0);
        assert_eq!(character.power_points, 10);
        assert_eq!(character.wealth, 500);
        assert_eq!(character.background, Some("A brave adventurer".to_string()));
        assert_eq!(character.description, Some("Tall and strong".to_string()));
    }

    #[test]
    fn test_get_by_id_not_found() {
        let conn = setup_test_db();

        let result = CharacterRepository::get_by_id(&conn, 999).unwrap();

        assert!(result.is_none());
    }

    #[test]
    fn test_get_all_empty() {
        let conn = setup_test_db();

        let characters = CharacterRepository::get_all(&conn).unwrap();

        assert_eq!(characters.len(), 0);
    }

    #[test]
    fn test_get_all_multiple() {
        let conn = setup_test_db();

        // Insert multiple characters
        let mut char1 = create_test_character();
        char1.name = "Alice".to_string();
        CharacterRepository::insert(&conn, &char1).unwrap();

        let mut char2 = create_test_character();
        char2.name = "Bob".to_string();
        CharacterRepository::insert(&conn, &char2).unwrap();

        let mut char3 = create_test_character();
        char3.name = "Charlie".to_string();
        CharacterRepository::insert(&conn, &char3).unwrap();

        let characters = CharacterRepository::get_all(&conn).unwrap();

        assert_eq!(characters.len(), 3);
        // Verify ordering by name
        assert_eq!(characters[0].name, "Alice");
        assert_eq!(characters[1].name, "Bob");
        assert_eq!(characters[2].name, "Charlie");
    }

    #[test]
    fn test_update() {
        let conn = setup_test_db();
        let id = insert_test_character(&conn);

        // Fetch the character
        let mut character = CharacterRepository::get_by_id(&conn, id).unwrap().unwrap();

        // Modify fields
        character.name = "Updated Hero".to_string();
        character.is_wild_card = false;
        character.power_points = 15;
        character.wealth = 1000;
        character.background = Some("Updated background".to_string());

        // Update
        CharacterRepository::update(&conn, &character).unwrap();

        // Verify the update
        let result = CharacterRepository::get_by_id(&conn, id).unwrap().unwrap();
        assert_eq!(result.name, "Updated Hero");
        assert!(!result.is_wild_card);
        assert_eq!(result.power_points, 15);
        assert_eq!(result.wealth, 1000);
        assert_eq!(result.background, Some("Updated background".to_string()));
    }

    #[test]
    fn test_update_to_null() {
        let conn = setup_test_db();
        let id = insert_test_character(&conn);

        // Fetch the character
        let mut character = CharacterRepository::get_by_id(&conn, id).unwrap().unwrap();

        // Set fields to None
        character.ancestry_id = None;
        character.background = None;
        character.description = None;

        // Update
        CharacterRepository::update(&conn, &character).unwrap();

        // Verify the update
        let result = CharacterRepository::get_by_id(&conn, id).unwrap().unwrap();
        assert_eq!(result.ancestry_id, None);
        assert_eq!(result.background, None);
        assert_eq!(result.description, None);
    }

    #[test]
    fn test_delete() {
        let conn = setup_test_db();
        let id = insert_test_character(&conn);

        // Verify character exists
        assert!(CharacterRepository::get_by_id(&conn, id).unwrap().is_some());

        // Delete
        CharacterRepository::delete(&conn, id).unwrap();

        // Verify character no longer exists
        assert!(CharacterRepository::get_by_id(&conn, id).unwrap().is_none());
    }

    #[test]
    fn test_delete_nonexistent() {
        let conn = setup_test_db();

        // Deleting a nonexistent character should not error
        let result = CharacterRepository::delete(&conn, 999);
        assert!(result.is_ok());
    }
}
