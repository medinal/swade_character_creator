use crate::error::Result;
use crate::models::HindranceRequirement;
use super::base_repository::{query_one_by_id, query_all_ordered, query_by_fk_id};
use rusqlite::{Connection, Row};

pub struct HindranceRequirementRepository;

impl HindranceRequirementRepository {
    const TABLE: &'static str = "hindrance_requirements";
    const COLUMNS: &'static str = "id, hindrance_id, requirement_expression_id, created_at, updated_at";

    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<HindranceRequirement>> {
        query_one_by_id(conn, Self::TABLE, Self::COLUMNS, id, Self::row_to_model)
    }

    pub fn get_all(conn: &Connection) -> Result<Vec<HindranceRequirement>> {
        query_all_ordered(conn, Self::TABLE, Self::COLUMNS, "hindrance_id, id", Self::row_to_model)
    }

    pub fn get_by_hindrance_id(
        conn: &Connection,
        hindrance_id: i64,
    ) -> Result<Vec<HindranceRequirement>> {
        query_by_fk_id(conn, Self::TABLE, Self::COLUMNS, "hindrance_id", hindrance_id, "id", Self::row_to_model)
    }

    fn row_to_model(row: &Row) -> rusqlite::Result<HindranceRequirement> {
        Ok(HindranceRequirement {
            id: row.get(0)?,
            hindrance_id: row.get(1)?,
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

    fn insert_test_hindrance(conn: &Connection) {
        conn.execute(
            "INSERT INTO hindrances (id, name, severity, point_value, source, description, created_at, updated_at)
             VALUES (1, 'Blind', 'major', 2, 'core', 'Cannot see', '2024-01-01', '2024-01-01')",
            [],
        ).unwrap();
    }

    fn insert_test_requirement_expression(conn: &Connection) {
        // First insert a requirement
        conn.execute(
            "INSERT INTO requirements (id, requirement_type, target_id, value, description, created_at, updated_at)
             VALUES (1, 'rank', NULL, 0, 'Novice', '2024-01-01', '2024-01-01')",
            [],
        ).unwrap();

        // Then insert a requirement_expression that references the requirement
        conn.execute(
            "INSERT INTO requirement_expressions (id, parent_id, node_type, requirement_id, position, created_at, updated_at)
             VALUES (1, NULL, 'requirement', 1, 0, '2024-01-01', '2024-01-01')",
            [],
        ).unwrap();
    }

    fn insert_test_hindrance_requirement(conn: &Connection) {
        conn.execute(
            "INSERT INTO hindrance_requirements (id, hindrance_id, requirement_expression_id, created_at, updated_at)
             VALUES (1, 1, 1, '2024-01-01', '2024-01-01')",
            [],
        ).unwrap();
    }

    #[test]
    fn test_get_by_id_found() {
        let conn = setup_test_db();
        insert_test_hindrance(&conn);
        insert_test_requirement_expression(&conn);
        insert_test_hindrance_requirement(&conn);

        let result = HindranceRequirementRepository::get_by_id(&conn, 1).unwrap();

        assert!(result.is_some());
        let hindrance_req = result.unwrap();
        assert_eq!(hindrance_req.id, 1);
        assert_eq!(hindrance_req.hindrance_id, 1);
        assert_eq!(hindrance_req.requirement_expression_id, 1);
    }

    #[test]
    fn test_get_by_id_not_found() {
        let conn = setup_test_db();

        let result = HindranceRequirementRepository::get_by_id(&conn, 999).unwrap();

        assert!(result.is_none());
    }

    #[test]
    fn test_get_all() {
        let conn = setup_test_db();
        insert_test_hindrance(&conn);
        insert_test_requirement_expression(&conn);
        insert_test_hindrance_requirement(&conn);

        let requirements = HindranceRequirementRepository::get_all(&conn).unwrap();

        assert_eq!(requirements.len(), 1);
        assert_eq!(requirements[0].hindrance_id, 1);
    }

    #[test]
    fn test_get_all_empty() {
        let conn = setup_test_db();

        let requirements = HindranceRequirementRepository::get_all(&conn).unwrap();

        assert_eq!(requirements.len(), 0);
    }

    #[test]
    fn test_get_by_hindrance_id_multiple_requirements() {
        let conn = setup_test_db();
        insert_test_hindrance(&conn);
        insert_test_requirement_expression(&conn);

        // Insert multiple requirement expressions
        conn.execute(
            "INSERT INTO requirements (id, requirement_type, target_id, value, description, created_at, updated_at)
             VALUES (2, 'attribute', 1, 8, 'Agility d8', '2024-01-01', '2024-01-01')",
            [],
        ).unwrap();
        conn.execute(
            "INSERT INTO requirement_expressions (id, parent_id, node_type, requirement_id, position, created_at, updated_at)
             VALUES (2, NULL, 'requirement', 2, 0, '2024-01-01', '2024-01-01')",
            [],
        ).unwrap();

        // Insert multiple hindrance requirements
        insert_test_hindrance_requirement(&conn);
        conn.execute(
            "INSERT INTO hindrance_requirements (id, hindrance_id, requirement_expression_id, created_at, updated_at)
             VALUES (2, 1, 2, '2024-01-01', '2024-01-01')",
            [],
        ).unwrap();

        let requirements = HindranceRequirementRepository::get_by_hindrance_id(&conn, 1).unwrap();

        assert_eq!(requirements.len(), 2);
        assert_eq!(requirements[0].hindrance_id, 1);
        assert_eq!(requirements[1].hindrance_id, 1);
    }

    #[test]
    fn test_get_by_hindrance_id_empty() {
        let conn = setup_test_db();

        let requirements = HindranceRequirementRepository::get_by_hindrance_id(&conn, 999).unwrap();

        assert_eq!(requirements.len(), 0);
    }

    #[test]
    fn test_get_by_hindrance_id_filters_correctly() {
        let conn = setup_test_db();
        insert_test_hindrance(&conn);

        // Insert second hindrance
        conn.execute(
            "INSERT INTO hindrances (id, name, severity, point_value, source, description, created_at, updated_at)
             VALUES (2, 'Deaf', 'major', 2, 'core', 'Cannot hear', '2024-01-01', '2024-01-01')",
            [],
        ).unwrap();

        insert_test_requirement_expression(&conn);

        // Insert requirements for both hindrances
        insert_test_hindrance_requirement(&conn);
        conn.execute(
            "INSERT INTO hindrance_requirements (id, hindrance_id, requirement_expression_id, created_at, updated_at)
             VALUES (2, 2, 1, '2024-01-01', '2024-01-01')",
            [],
        ).unwrap();

        let requirements = HindranceRequirementRepository::get_by_hindrance_id(&conn, 1).unwrap();

        assert_eq!(requirements.len(), 1);
        assert_eq!(requirements[0].hindrance_id, 1);
    }
}
