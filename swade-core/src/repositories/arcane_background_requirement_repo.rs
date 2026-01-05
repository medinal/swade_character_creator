use crate::error::Result;
use crate::models::ArcaneBackgroundRequirement;
use super::base_repository::{query_one_by_id, query_by_fk_id};
use rusqlite::{Connection, Row};

pub struct ArcaneBackgroundRequirementRepository;

impl ArcaneBackgroundRequirementRepository {
    const TABLE: &'static str = "arcane_background_requirements";
    const COLUMNS: &'static str = "id, arcane_background_id, requirement_expression_id, created_at, updated_at";

    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<ArcaneBackgroundRequirement>> {
        query_one_by_id(conn, Self::TABLE, Self::COLUMNS, id, Self::row_to_model)
    }

    pub fn get_by_arcane_background_id(
        conn: &Connection,
        arcane_background_id: i64,
    ) -> Result<Vec<ArcaneBackgroundRequirement>> {
        query_by_fk_id(conn, Self::TABLE, Self::COLUMNS, "arcane_background_id", arcane_background_id, "id", Self::row_to_model)
    }

    fn row_to_model(row: &Row) -> rusqlite::Result<ArcaneBackgroundRequirement> {
        Ok(ArcaneBackgroundRequirement {
            id: row.get(0)?,
            arcane_background_id: row.get(1)?,
            requirement_expression_id: row.get(2)?,
            created_at: row.get(3)?,
            updated_at: row.get(4)?,
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
        conn
    }

    fn insert_test_attribute(conn: &Connection) {
        conn.execute(
            "INSERT INTO attributes (id, name, description, created_at, updated_at)
             VALUES (1, 'Smarts', 'Mental aptitude', '2024-01-01', '2024-01-01')",
            [],
        )
        .unwrap();
    }

    fn insert_test_skill(conn: &Connection) {
        conn.execute(
            "INSERT INTO skills (id, name, description, linked_attribute_id, is_core_skill, source, created_at, updated_at)
             VALUES (1, 'Spellcasting', 'Arcane skill', 1, 0, 'core', '2024-01-01', '2024-01-01')",
            [],
        ).unwrap();
    }

    fn insert_test_arcane_background(conn: &Connection) {
        conn.execute(
            "INSERT INTO arcane_backgrounds (id, name, arcane_skill_id, starting_powers, starting_power_points, source, description, created_at, updated_at)
             VALUES (1, 'Magic', 1, 3, 10, 'core', 'Study of arcane arts', '2024-01-01', '2024-01-01')",
            [],
        ).unwrap();
    }

    fn insert_test_requirement_expression(conn: &Connection) {
        conn.execute(
            "INSERT INTO requirement_expressions (id, parent_id, node_type, requirement_id, position, created_at, updated_at)
             VALUES (1, NULL, 'requirement', 1, 0, '2024-01-01', '2024-01-01')",
            [],
        ).unwrap();
    }

    fn insert_test_requirement(conn: &Connection) {
        conn.execute(
            "INSERT INTO requirements (id, requirement_type, target_id, value, description, created_at, updated_at)
             VALUES (1, 'rank', 1, 1, 'Novice', '2024-01-01', '2024-01-01')",
            [],
        ).unwrap();
    }

    fn insert_test_arcane_background_requirement(conn: &Connection) {
        conn.execute(
            "INSERT INTO arcane_background_requirements (id, arcane_background_id, requirement_expression_id, created_at, updated_at)
             VALUES (1, 1, 1, '2024-01-01', '2024-01-01')",
            [],
        ).unwrap();
    }

    #[test]
    fn test_get_by_id_found() {
        let conn = setup_test_db();
        insert_test_attribute(&conn);
        insert_test_skill(&conn);
        insert_test_arcane_background(&conn);
        insert_test_requirement(&conn);
        insert_test_requirement_expression(&conn);
        insert_test_arcane_background_requirement(&conn);

        let result = ArcaneBackgroundRequirementRepository::get_by_id(&conn, 1).unwrap();

        assert!(result.is_some());
        let ab_req = result.unwrap();
        assert_eq!(ab_req.id, 1);
        assert_eq!(ab_req.arcane_background_id, 1);
        assert_eq!(ab_req.requirement_expression_id, 1);
    }

    #[test]
    fn test_get_by_id_not_found() {
        let conn = setup_test_db();

        let result = ArcaneBackgroundRequirementRepository::get_by_id(&conn, 999).unwrap();

        assert!(result.is_none());
    }

    #[test]
    fn test_get_by_arcane_background_id_multiple_requirements() {
        let conn = setup_test_db();
        insert_test_attribute(&conn);
        insert_test_skill(&conn);
        insert_test_arcane_background(&conn);
        insert_test_requirement(&conn);
        insert_test_requirement_expression(&conn);

        // Insert another requirement and expression
        conn.execute(
            "INSERT INTO requirements (id, requirement_type, target_id, value, description, created_at, updated_at)
             VALUES (2, 'attribute', 1, 8, 'Smarts d8+', '2024-01-01', '2024-01-01')",
            [],
        ).unwrap();
        conn.execute(
            "INSERT INTO requirement_expressions (id, parent_id, node_type, requirement_id, position, created_at, updated_at)
             VALUES (2, NULL, 'requirement', 2, 0, '2024-01-01', '2024-01-01')",
            [],
        ).unwrap();

        // Insert multiple arcane background requirements
        insert_test_arcane_background_requirement(&conn);
        conn.execute(
            "INSERT INTO arcane_background_requirements (id, arcane_background_id, requirement_expression_id, created_at, updated_at)
             VALUES (2, 1, 2, '2024-01-01', '2024-01-01')",
            [],
        ).unwrap();

        let requirements =
            ArcaneBackgroundRequirementRepository::get_by_arcane_background_id(&conn, 1).unwrap();

        assert_eq!(requirements.len(), 2);
        assert_eq!(requirements[0].arcane_background_id, 1);
        assert_eq!(requirements[1].arcane_background_id, 1);
    }

    #[test]
    fn test_get_by_arcane_background_id_empty() {
        let conn = setup_test_db();

        let requirements =
            ArcaneBackgroundRequirementRepository::get_by_arcane_background_id(&conn, 999).unwrap();

        assert_eq!(requirements.len(), 0);
    }

    #[test]
    fn test_get_by_arcane_background_id_filters_correctly() {
        let conn = setup_test_db();
        insert_test_attribute(&conn);
        insert_test_skill(&conn);
        insert_test_arcane_background(&conn);

        // Insert second arcane background
        conn.execute(
            "INSERT INTO arcane_backgrounds (id, name, arcane_skill_id, starting_powers, starting_power_points, source, description, created_at, updated_at)
             VALUES (2, 'Psionics', 1, 3, 10, 'core', 'Mental powers', '2024-01-01', '2024-01-01')",
            [],
        ).unwrap();

        insert_test_requirement(&conn);
        insert_test_requirement_expression(&conn);

        // Insert requirements for both arcane backgrounds
        insert_test_arcane_background_requirement(&conn);
        conn.execute(
            "INSERT INTO arcane_background_requirements (id, arcane_background_id, requirement_expression_id, created_at, updated_at)
             VALUES (2, 2, 1, '2024-01-01', '2024-01-01')",
            [],
        ).unwrap();

        let requirements =
            ArcaneBackgroundRequirementRepository::get_by_arcane_background_id(&conn, 1).unwrap();

        assert_eq!(requirements.len(), 1);
        assert_eq!(requirements[0].arcane_background_id, 1);
    }
}
