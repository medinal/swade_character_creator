use rusqlite::Connection;

use crate::error::Result;
use crate::repositories::HindranceRepository;
use crate::services::{ModifierService, RequirementService};
use crate::views::HindranceView;

pub struct HindranceService;

impl HindranceService {
    pub fn get_all(conn: &Connection) -> Result<Vec<HindranceView>> {
        let hindrances = HindranceRepository::get_all(conn)?;

        let mut views = Vec::new();
        for hindrance in hindrances {
            let modifiers = ModifierService::get_for_hindrance(conn, hindrance.id)?;
            let requirements = RequirementService::get_for_hindrance(conn, hindrance.id)?;
            views.push(HindranceView::new(hindrance, modifiers, requirements));
        }

        Ok(views)
    }

    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<HindranceView>> {
        let hindrance = HindranceRepository::get_by_id(conn, id)?;

        match hindrance {
            Some(hindrance) => {
                let modifiers = ModifierService::get_for_hindrance(conn, hindrance.id)?;
                let requirements = RequirementService::get_for_hindrance(conn, hindrance.id)?;
                Ok(Some(HindranceView::new(hindrance, modifiers, requirements)))
            }
            None => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::setup_test_db;
    use rusqlite::params;

    fn insert_test_hindrance(
        conn: &Connection,
        id: i64,
        name: &str,
        severity: &str,
        point_value: i64,
    ) {
        // severity must be lowercase per CHECK constraint
        conn.execute(
            "INSERT INTO hindrances (id, name, severity, point_value, source, description,
                                    created_at, updated_at)
             VALUES (?, ?, ?, ?, 'core', 'Test description', '2024-01-01', '2024-01-01')",
            params![id, name, severity.to_lowercase(), point_value],
        )
        .unwrap();
    }

    fn insert_modifier(conn: &Connection, id: i64, description: &str) {
        conn.execute(
            "INSERT INTO modifiers (id, target_type, target_identifier, value_type,
                                   value, description, created_at, updated_at)
             VALUES (?, 'skill', 'Notice', 'roll_bonus', -2, ?, '2024-01-01', '2024-01-01')",
            params![id, description],
        )
        .unwrap();
    }

    fn insert_hindrance_modifier(conn: &Connection, hindrance_id: i64, modifier_id: i64) {
        conn.execute(
            "INSERT INTO hindrance_modifiers (hindrance_id, modifier_id, created_at, updated_at)
             VALUES (?, ?, '2024-01-01', '2024-01-01')",
            params![hindrance_id, modifier_id],
        )
        .unwrap();
    }

    fn insert_requirement(conn: &Connection, id: i64, req_type: &str, description: &str) {
        conn.execute(
            "INSERT INTO requirements (id, requirement_type, target_id, value, description,
                                       created_at, updated_at)
             VALUES (?, ?, 1, 1, ?, '2024-01-01', '2024-01-01')",
            params![id, req_type, description],
        )
        .unwrap();
    }

    fn insert_requirement_expression(
        conn: &Connection,
        id: i64,
        parent_id: Option<i64>,
        node_type: &str,
        requirement_id: Option<i64>,
        position: i64,
    ) {
        conn.execute(
            "INSERT INTO requirement_expressions (id, parent_id, node_type, requirement_id,
                                                  position, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, '2024-01-01', '2024-01-01')",
            params![id, parent_id, node_type, requirement_id, position],
        )
        .unwrap();
    }

    fn insert_hindrance_requirement(conn: &Connection, hindrance_id: i64, expression_id: i64) {
        conn.execute(
            "INSERT INTO hindrance_requirements (hindrance_id, requirement_expression_id, created_at, updated_at)
             VALUES (?, ?, '2024-01-01', '2024-01-01')",
            params![hindrance_id, expression_id],
        )
        .unwrap();
    }

    #[test]
    fn get_all_returns_empty_when_no_hindrances() {
        let conn = setup_test_db();

        let hindrances = HindranceService::get_all(&conn).unwrap();

        assert!(hindrances.is_empty());
    }

    #[test]
    fn get_all_returns_hindrances() {
        let conn = setup_test_db();
        insert_test_hindrance(&conn, 1, "Blind", "Major", 2);
        insert_test_hindrance(&conn, 2, "Curious", "Major", 2);

        let hindrances = HindranceService::get_all(&conn).unwrap();

        assert_eq!(hindrances.len(), 2);
    }

    #[test]
    fn get_all_includes_modifiers() {
        let conn = setup_test_db();
        insert_test_hindrance(&conn, 1, "Bad Eyes", "Minor", 1);
        insert_modifier(&conn, 1, "-2 to Notice (sight)");
        insert_hindrance_modifier(&conn, 1, 1);

        let hindrances = HindranceService::get_all(&conn).unwrap();

        assert_eq!(hindrances.len(), 1);
        assert_eq!(hindrances[0].modifiers.len(), 1);
        assert_eq!(
            hindrances[0].modifiers[0].description,
            "-2 to Notice (sight)"
        );
    }

    #[test]
    fn get_all_includes_requirements() {
        let conn = setup_test_db();
        insert_test_hindrance(&conn, 1, "Test Hindrance", "Major", 2);
        insert_requirement(&conn, 1, "rank", "Seasoned");
        insert_requirement_expression(&conn, 1, None, "requirement", Some(1), 0);
        insert_hindrance_requirement(&conn, 1, 1);

        let hindrances = HindranceService::get_all(&conn).unwrap();

        assert_eq!(hindrances.len(), 1);
        assert!(!hindrances[0].requirements.is_empty());
    }

    #[test]
    fn get_by_id_returns_none_when_not_found() {
        let conn = setup_test_db();

        let hindrance = HindranceService::get_by_id(&conn, 999).unwrap();

        assert!(hindrance.is_none());
    }

    #[test]
    fn get_by_id_returns_hindrance_with_modifiers_and_requirements() {
        let conn = setup_test_db();
        insert_test_hindrance(&conn, 1, "Bad Eyes", "Minor", 1);
        insert_modifier(&conn, 1, "-2 to Notice (sight)");
        insert_hindrance_modifier(&conn, 1, 1);
        insert_requirement(&conn, 1, "rank", "Novice");
        insert_requirement_expression(&conn, 1, None, "requirement", Some(1), 0);
        insert_hindrance_requirement(&conn, 1, 1);

        let hindrance = HindranceService::get_by_id(&conn, 1).unwrap();

        assert!(hindrance.is_some());
        let hindrance = hindrance.unwrap();
        assert_eq!(hindrance.name, "Bad Eyes");
        assert_eq!(hindrance.modifiers.len(), 1);
        assert!(!hindrance.requirements.is_empty());
    }

    #[test]
    fn get_by_id_maps_fields_correctly() {
        let conn = setup_test_db();
        insert_test_hindrance(&conn, 1, "Blind", "Major", 2);

        let hindrance = HindranceService::get_by_id(&conn, 1).unwrap().unwrap();

        assert_eq!(hindrance.id, 1);
        assert_eq!(hindrance.name, "Blind");
        assert_eq!(hindrance.severity, crate::views::Severity::Major);
        assert_eq!(hindrance.point_value, 2);
        assert_eq!(hindrance.source, "core");
        assert_eq!(hindrance.description, "Test description");
    }
}
