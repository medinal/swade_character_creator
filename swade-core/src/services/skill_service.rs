use rusqlite::Connection;

use crate::error::Result;
use crate::repositories::SkillRepository;
use crate::views::SkillView;

pub struct SkillService;

impl SkillService {
    pub fn get_all(conn: &Connection) -> Result<Vec<SkillView>> {
        let skills = SkillRepository::get_all(conn)?;
        Ok(skills.into_iter().map(SkillView::new).collect())
    }

    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<SkillView>> {
        let skill = SkillRepository::get_by_id(conn, id)?;
        Ok(skill.map(SkillView::new))
    }

    /// Get a skill by name (case-insensitive)
    pub fn get_by_name(conn: &Connection, name: &str) -> Result<Option<SkillView>> {
        let skill = SkillRepository::get_by_name(conn, name)?;
        Ok(skill.map(SkillView::new))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::setup_test_db;
    use crate::views::Die;
    use rusqlite::params;

    fn insert_test_attribute(conn: &Connection, id: i64, name: &str) {
        conn.execute(
            "INSERT INTO attributes (id, name, description, base_value, created_at, updated_at)
             VALUES (?, ?, 'Test', 4, '2024-01-01', '2024-01-01')",
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
        let default_die_size: Option<i64> = if is_core { Some(4) } else { None };
        conn.execute(
            "INSERT INTO skills (id, name, description, linked_attribute_id, is_core_skill,
                                 default_die_size, max_die_size, max_die_modifier, source,
                                 created_at, updated_at)
             VALUES (?, ?, 'Test description', ?, ?, ?, 12, 0, 'core', '2024-01-01', '2024-01-01')",
            params![id, name, linked_attribute_id, is_core, default_die_size],
        )
        .unwrap();
    }

    #[test]
    fn get_all_returns_skill_views() {
        let conn = setup_test_db();
        insert_test_attribute(&conn, 1, "Agility");
        insert_test_skill(&conn, 1, "Athletics", 1, true);
        insert_test_skill(&conn, 2, "Stealth", 1, true);

        let views = SkillService::get_all(&conn).unwrap();

        assert_eq!(views.len(), 2);
        assert_eq!(views[0].name, "Athletics");
        assert_eq!(views[0].is_core_skill, true);
        assert_eq!(views[0].default_die, Some(Die::d4()));
        assert_eq!(views[1].name, "Stealth");
    }

    #[test]
    fn get_all_returns_empty_when_no_skills() {
        let conn = setup_test_db();

        let views = SkillService::get_all(&conn).unwrap();

        assert_eq!(views.len(), 0);
    }

    #[test]
    fn get_by_id_returns_skill_view() {
        let conn = setup_test_db();
        insert_test_attribute(&conn, 1, "Smarts");
        insert_test_skill(&conn, 1, "Notice", 1, true);

        let view = SkillService::get_by_id(&conn, 1).unwrap();

        assert!(view.is_some());
        let view = view.unwrap();
        assert_eq!(view.id, 1);
        assert_eq!(view.name, "Notice");
        assert_eq!(view.linked_attribute_id, 1);
        assert_eq!(view.default_die, Some(Die::d4()));
    }

    #[test]
    fn get_by_id_returns_none_when_not_found() {
        let conn = setup_test_db();

        let view = SkillService::get_by_id(&conn, 999).unwrap();

        assert!(view.is_none());
    }

    #[test]
    fn get_by_id_non_core_skill_has_no_default_die() {
        let conn = setup_test_db();
        insert_test_attribute(&conn, 1, "Smarts");
        insert_test_skill(&conn, 1, "Occult", 1, false);

        let view = SkillService::get_by_id(&conn, 1).unwrap().unwrap();

        assert_eq!(view.is_core_skill, false);
        assert_eq!(view.default_die, None);
    }

    #[test]
    fn get_by_name_returns_skill_view() {
        let conn = setup_test_db();
        insert_test_attribute(&conn, 1, "Smarts");
        insert_test_skill(&conn, 1, "Notice", 1, true);

        let view = SkillService::get_by_name(&conn, "Notice").unwrap();

        assert!(view.is_some());
        let view = view.unwrap();
        assert_eq!(view.name, "Notice");
        assert_eq!(view.default_die, Some(Die::d4()));
    }

    #[test]
    fn get_by_name_is_case_insensitive() {
        let conn = setup_test_db();
        insert_test_attribute(&conn, 1, "Smarts");
        insert_test_skill(&conn, 1, "Notice", 1, true);

        assert!(
            SkillService::get_by_name(&conn, "notice")
                .unwrap()
                .is_some()
        );
        assert!(
            SkillService::get_by_name(&conn, "NOTICE")
                .unwrap()
                .is_some()
        );
        assert!(
            SkillService::get_by_name(&conn, "NoTiCe")
                .unwrap()
                .is_some()
        );
    }

    #[test]
    fn get_by_name_returns_none_when_not_found() {
        let conn = setup_test_db();

        let view = SkillService::get_by_name(&conn, "NonExistent").unwrap();

        assert!(view.is_none());
    }
}
