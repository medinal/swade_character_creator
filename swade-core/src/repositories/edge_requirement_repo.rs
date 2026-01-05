use crate::error::Result;
use crate::models::EdgeRequirement;
use super::base_repository::{query_one_by_id, query_all_ordered, query_by_fk_id};
use rusqlite::{Connection, Row};

pub struct EdgeRequirementRepository;

impl EdgeRequirementRepository {
    const TABLE: &'static str = "edge_requirements";
    const COLUMNS: &'static str = "id, edge_id, requirement_expression_id, created_at, updated_at";

    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<EdgeRequirement>> {
        query_one_by_id(conn, Self::TABLE, Self::COLUMNS, id, Self::row_to_model)
    }

    pub fn get_all(conn: &Connection) -> Result<Vec<EdgeRequirement>> {
        query_all_ordered(conn, Self::TABLE, Self::COLUMNS, "edge_id, id", Self::row_to_model)
    }

    pub fn get_by_edge_id(conn: &Connection, edge_id: i64) -> Result<Vec<EdgeRequirement>> {
        query_by_fk_id(conn, Self::TABLE, Self::COLUMNS, "edge_id", edge_id, "id", Self::row_to_model)
    }

    fn row_to_model(row: &Row) -> rusqlite::Result<EdgeRequirement> {
        Ok(EdgeRequirement {
            id: row.get(0)?,
            edge_id: row.get(1)?,
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

    fn insert_test_edge(conn: &Connection) {
        conn.execute(
            "INSERT INTO edges (id, name, background, source, description, can_take_multiple_times, created_at, updated_at)
             VALUES (1, 'Alertness', 0, 'core', 'Bonus to Notice', 0, '2024-01-01', '2024-01-01')",
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

    fn insert_test_edge_requirement(conn: &Connection) {
        conn.execute(
            "INSERT INTO edge_requirements (id, edge_id, requirement_expression_id, created_at, updated_at)
             VALUES (1, 1, 1, '2024-01-01', '2024-01-01')",
            [],
        ).unwrap();
    }

    #[test]
    fn test_get_by_id_found() {
        let conn = setup_test_db();
        insert_test_edge(&conn);
        insert_test_requirement(&conn);
        insert_test_requirement_expression(&conn);
        insert_test_edge_requirement(&conn);

        let result = EdgeRequirementRepository::get_by_id(&conn, 1).unwrap();

        assert!(result.is_some());
        let edge_req = result.unwrap();
        assert_eq!(edge_req.id, 1);
        assert_eq!(edge_req.edge_id, 1);
        assert_eq!(edge_req.requirement_expression_id, 1);
    }

    #[test]
    fn test_get_by_id_not_found() {
        let conn = setup_test_db();

        let result = EdgeRequirementRepository::get_by_id(&conn, 999).unwrap();

        assert!(result.is_none());
    }

    #[test]
    fn test_get_all() {
        let conn = setup_test_db();
        insert_test_edge(&conn);
        insert_test_requirement(&conn);
        insert_test_requirement_expression(&conn);
        insert_test_edge_requirement(&conn);

        let requirements = EdgeRequirementRepository::get_all(&conn).unwrap();

        assert_eq!(requirements.len(), 1);
        assert_eq!(requirements[0].edge_id, 1);
    }

    #[test]
    fn test_get_by_edge_id_multiple_requirements() {
        let conn = setup_test_db();
        insert_test_edge(&conn);
        insert_test_requirement(&conn);
        insert_test_requirement_expression(&conn);

        // Insert another requirement and expression
        conn.execute(
            "INSERT INTO requirements (id, requirement_type, target_id, value, description, created_at, updated_at)
             VALUES (2, 'attribute', 1, 8, 'Agility d8+', '2024-01-01', '2024-01-01')",
            [],
        ).unwrap();
        conn.execute(
            "INSERT INTO requirement_expressions (id, parent_id, node_type, requirement_id, position, created_at, updated_at)
             VALUES (2, NULL, 'requirement', 2, 0, '2024-01-01', '2024-01-01')",
            [],
        ).unwrap();

        // Insert multiple edge requirements
        insert_test_edge_requirement(&conn);
        conn.execute(
            "INSERT INTO edge_requirements (id, edge_id, requirement_expression_id, created_at, updated_at)
             VALUES (2, 1, 2, '2024-01-01', '2024-01-01')",
            [],
        ).unwrap();

        let requirements = EdgeRequirementRepository::get_by_edge_id(&conn, 1).unwrap();

        assert_eq!(requirements.len(), 2);
        assert_eq!(requirements[0].edge_id, 1);
        assert_eq!(requirements[1].edge_id, 1);
    }

    #[test]
    fn test_get_by_edge_id_empty() {
        let conn = setup_test_db();

        let requirements = EdgeRequirementRepository::get_by_edge_id(&conn, 999).unwrap();

        assert_eq!(requirements.len(), 0);
    }

    #[test]
    fn test_get_by_edge_id_filters_correctly() {
        let conn = setup_test_db();
        insert_test_edge(&conn);

        // Insert second edge
        conn.execute(
            "INSERT INTO edges (id, name, background, source, description, can_take_multiple_times, created_at, updated_at)
             VALUES (2, 'Brave', 0, 'core', 'Bonus to Fear checks', 0, '2024-01-01', '2024-01-01')",
            [],
        ).unwrap();

        insert_test_requirement(&conn);
        insert_test_requirement_expression(&conn);

        // Insert requirements for both edges
        insert_test_edge_requirement(&conn);
        conn.execute(
            "INSERT INTO edge_requirements (id, edge_id, requirement_expression_id, created_at, updated_at)
             VALUES (2, 2, 1, '2024-01-01', '2024-01-01')",
            [],
        ).unwrap();

        let requirements = EdgeRequirementRepository::get_by_edge_id(&conn, 1).unwrap();

        assert_eq!(requirements.len(), 1);
        assert_eq!(requirements[0].edge_id, 1);
    }
}
