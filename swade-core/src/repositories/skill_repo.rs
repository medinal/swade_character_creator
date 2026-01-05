use crate::error::Result;
use crate::models::Skill;
use super::base_repository::{query_one_by_id, query_all_ordered, query_one_by_field, query_where};
use rusqlite::{Connection, Row};

pub struct SkillRepository;

impl SkillRepository {
    const TABLE: &'static str = "skills";
    const COLUMNS: &'static str = "id, name, description, linked_attribute_id, is_core_skill,
                                   default_die_size, max_die_size, max_die_modifier, source,
                                   created_at, updated_at";

    /// Get a skill by its ID
    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<Skill>> {
        query_one_by_id(conn, Self::TABLE, Self::COLUMNS, id, Self::row_to_model)
    }

    /// Get all skills, ordered by name
    pub fn get_all(conn: &Connection) -> Result<Vec<Skill>> {
        query_all_ordered(conn, Self::TABLE, Self::COLUMNS, "name", Self::row_to_model)
    }

    /// Get all core skills
    pub fn get_core_skills(conn: &Connection) -> Result<Vec<Skill>> {
        query_where(conn, Self::TABLE, Self::COLUMNS, "is_core_skill = 1", "name", Self::row_to_model)
    }

    /// Get a skill by name (case-insensitive)
    pub fn get_by_name(conn: &Connection, name: &str) -> Result<Option<Skill>> {
        query_one_by_field(conn, Self::TABLE, Self::COLUMNS, "name", name, Self::row_to_model)
    }

    /// Convert a database row to a Skill model
    fn row_to_model(row: &Row) -> rusqlite::Result<Skill> {
        Ok(Skill {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            linked_attribute_id: row.get(3)?,
            is_core_skill: row.get(4)?,
            default_die_size: row.get(5)?,
            max_die_size: row.get(6)?,
            max_die_modifier: row.get(7)?,
            source: row.get(8)?,
            created_at: row.get(9)?,
            updated_at: row.get(10)?,
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

    fn insert_test_attribute(conn: &Connection, id: i64, name: &str) {
        conn.execute(
            "INSERT INTO attributes (id, name, description, base_value, created_at, updated_at)
             VALUES (?, ?, 'Test attribute', 4, '2024-01-01', '2024-01-01')",
            params![id, name],
        )
        .unwrap();
    }

    fn insert_test_skill(
        conn: &Connection,
        id: i64,
        name: &str,
        linked_attribute_id: i64,
        is_core: bool,
    ) {
        let default_die_size = if is_core { Some(4) } else { None };
        conn.execute(
            "INSERT INTO skills (id, name, description, linked_attribute_id, is_core_skill,
                                 default_die_size, max_die_size, max_die_modifier, source,
                                 created_at, updated_at)
             VALUES (?, ?, 'Test skill description', ?, ?, ?, 12, 0, 'core',
                     '2024-01-01', '2024-01-01')",
            params![id, name, linked_attribute_id, is_core, default_die_size],
        )
        .unwrap();
    }

    #[test]
    fn test_get_by_id_found() {
        let conn = setup_test_db();
        insert_test_attribute(&conn, 1, "Agility");
        insert_test_skill(&conn, 1, "Athletics", 1, true);

        let result = SkillRepository::get_by_id(&conn, 1).unwrap();

        assert!(result.is_some());
        let skill = result.unwrap();
        assert_eq!(skill.id, 1);
        assert_eq!(skill.name, "Athletics");
        assert_eq!(skill.description, "Test skill description");
        assert_eq!(skill.linked_attribute_id, 1);
        assert_eq!(skill.is_core_skill, true);
        assert_eq!(skill.default_die_size, Some(4));
        assert_eq!(skill.max_die_size, 12);
        assert_eq!(skill.max_die_modifier, 0);
        assert_eq!(skill.source, "core");
    }

    #[test]
    fn test_get_by_id_not_found() {
        let conn = setup_test_db();

        let result = SkillRepository::get_by_id(&conn, 999).unwrap();

        assert!(result.is_none());
    }

    #[test]
    fn test_get_by_id_non_core_skill() {
        let conn = setup_test_db();
        insert_test_attribute(&conn, 1, "Smarts");
        insert_test_skill(&conn, 1, "Occult", 1, false);

        let result = SkillRepository::get_by_id(&conn, 1).unwrap();

        assert!(result.is_some());
        let skill = result.unwrap();
        assert_eq!(skill.name, "Occult");
        assert_eq!(skill.is_core_skill, false);
        assert_eq!(skill.default_die_size, None);
    }

    #[test]
    fn test_get_all_empty() {
        let conn = setup_test_db();

        let skills = SkillRepository::get_all(&conn).unwrap();

        assert_eq!(skills.len(), 0);
    }

    #[test]
    fn test_get_all_single_skill() {
        let conn = setup_test_db();
        insert_test_attribute(&conn, 1, "Agility");
        insert_test_skill(&conn, 1, "Athletics", 1, true);

        let skills = SkillRepository::get_all(&conn).unwrap();

        assert_eq!(skills.len(), 1);
        assert_eq!(skills[0].name, "Athletics");
    }

    #[test]
    fn test_get_all_multiple_skills_ordered_by_name() {
        let conn = setup_test_db();
        insert_test_attribute(&conn, 1, "Agility");
        insert_test_attribute(&conn, 2, "Smarts");

        // Insert in non-alphabetical order to verify sorting
        insert_test_skill(&conn, 1, "Stealth", 1, true);
        insert_test_skill(&conn, 2, "Athletics", 1, true);
        insert_test_skill(&conn, 3, "Occult", 2, false);

        let skills = SkillRepository::get_all(&conn).unwrap();

        assert_eq!(skills.len(), 3);
        // Verify alphabetical ordering
        assert_eq!(skills[0].name, "Athletics");
        assert_eq!(skills[1].name, "Occult");
        assert_eq!(skills[2].name, "Stealth");
    }

    #[test]
    fn test_get_all_mix_of_core_and_non_core_skills() {
        let conn = setup_test_db();
        insert_test_attribute(&conn, 1, "Agility");
        insert_test_attribute(&conn, 2, "Smarts");

        insert_test_skill(&conn, 1, "Athletics", 1, true);
        insert_test_skill(&conn, 2, "Notice", 2, true);
        insert_test_skill(&conn, 3, "Occult", 2, false);
        insert_test_skill(&conn, 4, "Survival", 2, false);

        let skills = SkillRepository::get_all(&conn).unwrap();

        assert_eq!(skills.len(), 4);

        // Verify core skills have default_die_size
        let athletics = skills.iter().find(|s| s.name == "Athletics").unwrap();
        assert_eq!(athletics.is_core_skill, true);
        assert_eq!(athletics.default_die_size, Some(4));

        let notice = skills.iter().find(|s| s.name == "Notice").unwrap();
        assert_eq!(notice.is_core_skill, true);
        assert_eq!(notice.default_die_size, Some(4));

        // Verify non-core skills have no default_die_size
        let occult = skills.iter().find(|s| s.name == "Occult").unwrap();
        assert_eq!(occult.is_core_skill, false);
        assert_eq!(occult.default_die_size, None);

        let survival = skills.iter().find(|s| s.name == "Survival").unwrap();
        assert_eq!(survival.is_core_skill, false);
        assert_eq!(survival.default_die_size, None);
    }

    #[test]
    fn test_row_to_model_all_fields() {
        let conn = setup_test_db();
        insert_test_attribute(&conn, 1, "Vigor");

        conn.execute(
            "INSERT INTO skills (id, name, description, linked_attribute_id, is_core_skill,
                                 default_die_size, max_die_size, max_die_modifier, source,
                                 created_at, updated_at)
             VALUES (1, 'Test Skill', 'Detailed description', 1, 1, 4, 12, 2, 'test_source',
                     '2024-01-15 10:30:00', '2024-01-20 15:45:00')",
            [],
        )
        .unwrap();

        let skill = SkillRepository::get_by_id(&conn, 1).unwrap().unwrap();

        assert_eq!(skill.id, 1);
        assert_eq!(skill.name, "Test Skill");
        assert_eq!(skill.description, "Detailed description");
        assert_eq!(skill.linked_attribute_id, 1);
        assert_eq!(skill.is_core_skill, true);
        assert_eq!(skill.default_die_size, Some(4));
        assert_eq!(skill.max_die_size, 12);
        assert_eq!(skill.max_die_modifier, 2);
        assert_eq!(skill.source, "test_source");
        assert_eq!(skill.created_at, "2024-01-15 10:30:00");
        assert_eq!(skill.updated_at, "2024-01-20 15:45:00");
    }

    #[test]
    fn test_get_by_name_found() {
        let conn = setup_test_db();
        insert_test_attribute(&conn, 1, "Smarts");
        insert_test_skill(&conn, 1, "Notice", 1, true);

        let result = SkillRepository::get_by_name(&conn, "Notice").unwrap();

        assert!(result.is_some());
        let skill = result.unwrap();
        assert_eq!(skill.id, 1);
        assert_eq!(skill.name, "Notice");
    }

    #[test]
    fn test_get_by_name_case_insensitive() {
        let conn = setup_test_db();
        insert_test_attribute(&conn, 1, "Smarts");
        insert_test_skill(&conn, 1, "Notice", 1, true);

        // Test various case combinations
        assert!(
            SkillRepository::get_by_name(&conn, "notice")
                .unwrap()
                .is_some()
        );
        assert!(
            SkillRepository::get_by_name(&conn, "NOTICE")
                .unwrap()
                .is_some()
        );
        assert!(
            SkillRepository::get_by_name(&conn, "NoTiCe")
                .unwrap()
                .is_some()
        );
    }

    #[test]
    fn test_get_by_name_not_found() {
        let conn = setup_test_db();

        let result = SkillRepository::get_by_name(&conn, "NonExistent").unwrap();

        assert!(result.is_none());
    }
}
