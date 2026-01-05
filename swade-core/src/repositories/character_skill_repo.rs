use crate::error::Result;
use crate::models::CharacterSkill;
use rusqlite::{Connection, Row, params};

pub struct CharacterSkillRepository;

impl CharacterSkillRepository {
    /// Get a single character skill by ID
    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<CharacterSkill>> {
        let mut stmt = conn.prepare(
            "SELECT id, character_id, skill_id, current_die_size, current_die_modifier,
                    created_at, updated_at
             FROM character_skills WHERE id = ?",
        )?;

        let mut rows = stmt.query(params![id])?;

        match rows.next()? {
            Some(row) => Ok(Some(Self::row_to_model(row)?)),
            None => Ok(None),
        }
    }

    /// Get all skills for a specific character
    pub fn get_by_character_id(
        conn: &Connection,
        character_id: i64,
    ) -> Result<Vec<CharacterSkill>> {
        let mut stmt = conn.prepare(
            "SELECT id, character_id, skill_id, current_die_size, current_die_modifier,
                    created_at, updated_at
             FROM character_skills WHERE character_id = ?",
        )?;

        let rows = stmt.query_map(params![character_id], Self::row_to_model)?;

        Ok(rows.filter_map(|r| r.ok()).collect())
    }

    /// Insert a new character skill record and return the generated ID
    pub fn insert(conn: &Connection, model: &CharacterSkill) -> Result<i64> {
        conn.execute(
            "INSERT INTO character_skills (character_id, skill_id, current_die_size,
                                          current_die_modifier, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?)",
            params![
                model.character_id,
                model.skill_id,
                model.current_die_size,
                model.current_die_modifier,
                model.created_at,
                model.updated_at
            ],
        )?;

        Ok(conn.last_insert_rowid())
    }

    /// Update an existing character skill record
    pub fn update(conn: &Connection, model: &CharacterSkill) -> Result<()> {
        conn.execute(
            "UPDATE character_skills
             SET character_id = ?, skill_id = ?, current_die_size = ?,
                 current_die_modifier = ?, created_at = ?, updated_at = ?
             WHERE id = ?",
            params![
                model.character_id,
                model.skill_id,
                model.current_die_size,
                model.current_die_modifier,
                model.created_at,
                model.updated_at,
                model.id
            ],
        )?;

        Ok(())
    }

    /// Delete a character skill record
    pub fn delete(conn: &Connection, id: i64) -> Result<()> {
        conn.execute("DELETE FROM character_skills WHERE id = ?", params![id])?;
        Ok(())
    }

    /// Convert a database row to a CharacterSkill model
    fn row_to_model(row: &Row) -> rusqlite::Result<CharacterSkill> {
        Ok(CharacterSkill {
            id: row.get(0)?,
            character_id: row.get(1)?,
            skill_id: row.get(2)?,
            current_die_size: row.get(3)?,
            current_die_modifier: row.get(4)?,
            created_at: row.get(5)?,
            updated_at: row.get(6)?,
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
        conn.execute("PRAGMA foreign_keys = ON", []).unwrap();
        let schema = include_str!("../../../db/schema.sql");
        conn.execute_batch(schema).unwrap();
        conn
    }

    fn insert_test_rank(conn: &Connection, id: i64, name: &str) {
        conn.execute(
            "INSERT INTO ranks (id, name, min_advances, description, created_at, updated_at)
             VALUES (?, ?, 0, 'Test rank', '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            params![id, name],
        )
        .unwrap();
    }

    fn insert_test_character(conn: &Connection, id: i64, name: &str) {
        conn.execute(
            "INSERT INTO characters (id, name, is_wild_card, created_at, updated_at)
             VALUES (?, ?, 1, '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            params![id, name],
        ).unwrap();
    }

    fn insert_test_attribute(conn: &Connection, id: i64, name: &str) {
        conn.execute(
            "INSERT INTO attributes (id, name, description, base_value, created_at, updated_at)
             VALUES (?, ?, 'Test attribute', 4, '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            params![id, name],
        )
        .unwrap();
    }

    fn insert_test_skill(conn: &Connection, id: i64, name: &str, attribute_id: i64, is_core: bool) {
        conn.execute(
            "INSERT INTO skills (id, name, description, linked_attribute_id, is_core_skill,
                               default_die_size, created_at, updated_at)
             VALUES (?, ?, 'Test skill', ?, ?, ?, '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            params![
                id,
                name,
                attribute_id,
                is_core,
                if is_core { Some(4) } else { None::<i64> }
            ],
        )
        .unwrap();
    }

    fn insert_test_character_skill(
        conn: &Connection,
        id: i64,
        character_id: i64,
        skill_id: i64,
        die_size: Option<i64>,
        die_modifier: i64,
    ) {
        conn.execute(
            "INSERT INTO character_skills (id, character_id, skill_id, current_die_size,
                                          current_die_modifier, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            params![id, character_id, skill_id, die_size, die_modifier],
        )
        .unwrap();
    }

    #[test]
    fn test_get_by_id_found() {
        let conn = setup_test_db();
        insert_test_rank(&conn, 1, "Novice");
        insert_test_character(&conn, 1, "Test Character");
        insert_test_attribute(&conn, 1, "Smarts");
        insert_test_skill(&conn, 1, "Notice", 1, true);
        insert_test_character_skill(&conn, 1, 1, 1, Some(6), 0);

        let result = CharacterSkillRepository::get_by_id(&conn, 1).unwrap();

        assert!(result.is_some());
        let character_skill = result.unwrap();
        assert_eq!(character_skill.id, 1);
        assert_eq!(character_skill.character_id, 1);
        assert_eq!(character_skill.skill_id, 1);
        assert_eq!(character_skill.current_die_size, Some(6));
        assert_eq!(character_skill.current_die_modifier, 0);
        assert_eq!(character_skill.created_at, "2024-01-01 00:00:00");
        assert_eq!(character_skill.updated_at, "2024-01-01 00:00:00");
    }

    #[test]
    fn test_get_by_id_not_found() {
        let conn = setup_test_db();

        let result = CharacterSkillRepository::get_by_id(&conn, 999).unwrap();

        assert!(result.is_none());
    }

    #[test]
    fn test_get_by_id_with_null_die_size() {
        let conn = setup_test_db();
        insert_test_rank(&conn, 1, "Novice");
        insert_test_character(&conn, 1, "Test Character");
        insert_test_attribute(&conn, 1, "Agility");
        insert_test_skill(&conn, 1, "Fighting", 1, false);
        insert_test_character_skill(&conn, 1, 1, 1, None, 0);

        let result = CharacterSkillRepository::get_by_id(&conn, 1).unwrap();

        assert!(result.is_some());
        let character_skill = result.unwrap();
        assert_eq!(character_skill.current_die_size, None);
    }

    #[test]
    fn test_get_by_character_id_multiple_skills() {
        let conn = setup_test_db();
        insert_test_rank(&conn, 1, "Novice");
        insert_test_character(&conn, 1, "Test Character");
        insert_test_attribute(&conn, 1, "Smarts");
        insert_test_attribute(&conn, 2, "Agility");
        insert_test_skill(&conn, 1, "Notice", 1, true);
        insert_test_skill(&conn, 2, "Fighting", 2, false);
        insert_test_skill(&conn, 3, "Shooting", 2, false);

        insert_test_character_skill(&conn, 1, 1, 1, Some(6), 0);
        insert_test_character_skill(&conn, 2, 1, 2, Some(8), 1);
        insert_test_character_skill(&conn, 3, 1, 3, Some(4), 0);

        let skills = CharacterSkillRepository::get_by_character_id(&conn, 1).unwrap();

        assert_eq!(skills.len(), 3);
        assert_eq!(skills[0].skill_id, 1);
        assert_eq!(skills[1].skill_id, 2);
        assert_eq!(skills[2].skill_id, 3);
    }

    #[test]
    fn test_get_by_character_id_empty() {
        let conn = setup_test_db();

        let skills = CharacterSkillRepository::get_by_character_id(&conn, 999).unwrap();

        assert_eq!(skills.len(), 0);
    }

    #[test]
    fn test_get_by_character_id_filters_correctly() {
        let conn = setup_test_db();
        insert_test_rank(&conn, 1, "Novice");
        insert_test_character(&conn, 1, "Character 1");
        insert_test_character(&conn, 2, "Character 2");
        insert_test_attribute(&conn, 1, "Smarts");
        insert_test_skill(&conn, 1, "Notice", 1, true);

        insert_test_character_skill(&conn, 1, 1, 1, Some(6), 0);
        insert_test_character_skill(&conn, 2, 2, 1, Some(8), 0);

        let skills = CharacterSkillRepository::get_by_character_id(&conn, 1).unwrap();

        assert_eq!(skills.len(), 1);
        assert_eq!(skills[0].character_id, 1);
    }

    #[test]
    fn test_insert_new_character_skill() {
        let conn = setup_test_db();
        insert_test_rank(&conn, 1, "Novice");
        insert_test_character(&conn, 1, "Test Character");
        insert_test_attribute(&conn, 1, "Agility");
        insert_test_skill(&conn, 1, "Fighting", 1, false);

        let character_skill = CharacterSkill {
            id: 0, // Will be set by database
            character_id: 1,
            skill_id: 1,
            current_die_size: Some(6),
            current_die_modifier: 0,
            created_at: "2024-02-01 00:00:00".to_string(),
            updated_at: "2024-02-01 00:00:00".to_string(),
        };

        let id = CharacterSkillRepository::insert(&conn, &character_skill).unwrap();

        assert!(id > 0);

        let inserted = CharacterSkillRepository::get_by_id(&conn, id)
            .unwrap()
            .unwrap();
        assert_eq!(inserted.character_id, 1);
        assert_eq!(inserted.skill_id, 1);
        assert_eq!(inserted.current_die_size, Some(6));
        assert_eq!(inserted.current_die_modifier, 0);
    }

    #[test]
    fn test_insert_with_null_die_size() {
        let conn = setup_test_db();
        insert_test_rank(&conn, 1, "Novice");
        insert_test_character(&conn, 1, "Test Character");
        insert_test_attribute(&conn, 1, "Smarts");
        insert_test_skill(&conn, 1, "Research", 1, false);

        let character_skill = CharacterSkill {
            id: 0,
            character_id: 1,
            skill_id: 1,
            current_die_size: None,
            current_die_modifier: 0,
            created_at: "2024-02-01 00:00:00".to_string(),
            updated_at: "2024-02-01 00:00:00".to_string(),
        };

        let id = CharacterSkillRepository::insert(&conn, &character_skill).unwrap();

        let inserted = CharacterSkillRepository::get_by_id(&conn, id)
            .unwrap()
            .unwrap();
        assert_eq!(inserted.current_die_size, None);
    }

    #[test]
    fn test_insert_with_die_modifier() {
        let conn = setup_test_db();
        insert_test_rank(&conn, 1, "Novice");
        insert_test_character(&conn, 1, "Test Character");
        insert_test_attribute(&conn, 1, "Agility");
        insert_test_skill(&conn, 1, "Fighting", 1, false);

        let character_skill = CharacterSkill {
            id: 0,
            character_id: 1,
            skill_id: 1,
            current_die_size: Some(12),
            current_die_modifier: 2,
            created_at: "2024-02-01 00:00:00".to_string(),
            updated_at: "2024-02-01 00:00:00".to_string(),
        };

        let id = CharacterSkillRepository::insert(&conn, &character_skill).unwrap();

        let inserted = CharacterSkillRepository::get_by_id(&conn, id)
            .unwrap()
            .unwrap();
        assert_eq!(inserted.current_die_modifier, 2);
    }

    #[test]
    fn test_update_character_skill() {
        let conn = setup_test_db();
        insert_test_rank(&conn, 1, "Novice");
        insert_test_character(&conn, 1, "Test Character");
        insert_test_attribute(&conn, 1, "Agility");
        insert_test_skill(&conn, 1, "Fighting", 1, false);
        insert_test_character_skill(&conn, 1, 1, 1, Some(6), 0);

        let mut character_skill = CharacterSkillRepository::get_by_id(&conn, 1)
            .unwrap()
            .unwrap();
        character_skill.current_die_size = Some(8);
        character_skill.current_die_modifier = 1;
        character_skill.updated_at = "2024-03-01 00:00:00".to_string();

        CharacterSkillRepository::update(&conn, &character_skill).unwrap();

        let updated = CharacterSkillRepository::get_by_id(&conn, 1)
            .unwrap()
            .unwrap();
        assert_eq!(updated.current_die_size, Some(8));
        assert_eq!(updated.current_die_modifier, 1);
        assert_eq!(updated.updated_at, "2024-03-01 00:00:00");
    }

    #[test]
    fn test_update_to_null_die_size() {
        let conn = setup_test_db();
        insert_test_rank(&conn, 1, "Novice");
        insert_test_character(&conn, 1, "Test Character");
        insert_test_attribute(&conn, 1, "Agility");
        insert_test_skill(&conn, 1, "Fighting", 1, false);
        insert_test_character_skill(&conn, 1, 1, 1, Some(6), 0);

        let mut character_skill = CharacterSkillRepository::get_by_id(&conn, 1)
            .unwrap()
            .unwrap();
        character_skill.current_die_size = None;

        CharacterSkillRepository::update(&conn, &character_skill).unwrap();

        let updated = CharacterSkillRepository::get_by_id(&conn, 1)
            .unwrap()
            .unwrap();
        assert_eq!(updated.current_die_size, None);
    }

    #[test]
    fn test_delete_character_skill() {
        let conn = setup_test_db();
        insert_test_rank(&conn, 1, "Novice");
        insert_test_character(&conn, 1, "Test Character");
        insert_test_attribute(&conn, 1, "Agility");
        insert_test_skill(&conn, 1, "Fighting", 1, false);
        insert_test_character_skill(&conn, 1, 1, 1, Some(6), 0);

        CharacterSkillRepository::delete(&conn, 1).unwrap();

        let result = CharacterSkillRepository::get_by_id(&conn, 1).unwrap();
        assert!(result.is_none());
    }

    #[test]
    fn test_delete_nonexistent_skill() {
        let conn = setup_test_db();

        // Should not error when deleting non-existent record
        let result = CharacterSkillRepository::delete(&conn, 999);
        assert!(result.is_ok());
    }

    #[test]
    fn test_unique_constraint_character_skill_pair() {
        let conn = setup_test_db();
        insert_test_rank(&conn, 1, "Novice");
        insert_test_character(&conn, 1, "Test Character");
        insert_test_attribute(&conn, 1, "Agility");
        insert_test_skill(&conn, 1, "Fighting", 1, false);
        insert_test_character_skill(&conn, 1, 1, 1, Some(6), 0);

        // Attempting to insert duplicate character_id/skill_id pair should fail
        let result = conn.execute(
            "INSERT INTO character_skills (character_id, skill_id, current_die_size,
                                          current_die_modifier, created_at, updated_at)
             VALUES (1, 1, 8, 0, '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            [],
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_row_to_model_all_fields() {
        let conn = setup_test_db();
        insert_test_rank(&conn, 1, "Novice");
        insert_test_character(&conn, 42, "Test Character");
        insert_test_attribute(&conn, 5, "Vigor");
        insert_test_skill(&conn, 99, "Swimming", 5, false);

        conn.execute(
            "INSERT INTO character_skills (id, character_id, skill_id, current_die_size,
                                          current_die_modifier, created_at, updated_at)
             VALUES (100, 42, 99, 10, 3, '2024-12-25 10:30:00', '2024-12-26 15:45:00')",
            [],
        )
        .unwrap();

        let character_skill = CharacterSkillRepository::get_by_id(&conn, 100)
            .unwrap()
            .unwrap();

        assert_eq!(character_skill.id, 100);
        assert_eq!(character_skill.character_id, 42);
        assert_eq!(character_skill.skill_id, 99);
        assert_eq!(character_skill.current_die_size, Some(10));
        assert_eq!(character_skill.current_die_modifier, 3);
        assert_eq!(character_skill.created_at, "2024-12-25 10:30:00");
        assert_eq!(character_skill.updated_at, "2024-12-26 15:45:00");
    }

    #[test]
    fn test_foreign_key_constraint() {
        let conn = setup_test_db();
        insert_test_rank(&conn, 1, "Novice");
        insert_test_character(&conn, 1, "Test Character");
        insert_test_attribute(&conn, 1, "Agility");
        insert_test_skill(&conn, 1, "Fighting", 1, false);

        // Attempting to insert with non-existent character should fail
        let result = conn.execute(
            "INSERT INTO character_skills (character_id, skill_id, current_die_size,
                                          current_die_modifier, created_at, updated_at)
             VALUES (999, 1, 6, 0, '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            [],
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_multiple_characters_same_skill() {
        let conn = setup_test_db();
        insert_test_rank(&conn, 1, "Novice");
        insert_test_character(&conn, 1, "Character 1");
        insert_test_character(&conn, 2, "Character 2");
        insert_test_attribute(&conn, 1, "Agility");
        insert_test_skill(&conn, 1, "Fighting", 1, false);

        insert_test_character_skill(&conn, 1, 1, 1, Some(6), 0);
        insert_test_character_skill(&conn, 2, 2, 1, Some(8), 1);

        let char1_skills = CharacterSkillRepository::get_by_character_id(&conn, 1).unwrap();
        let char2_skills = CharacterSkillRepository::get_by_character_id(&conn, 2).unwrap();

        assert_eq!(char1_skills.len(), 1);
        assert_eq!(char2_skills.len(), 1);
        assert_eq!(char1_skills[0].current_die_size, Some(6));
        assert_eq!(char2_skills[0].current_die_size, Some(8));
    }
}
