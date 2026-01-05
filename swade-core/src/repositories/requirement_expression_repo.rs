use crate::error::Result;
use crate::models::RequirementExpression;
use super::base_repository::{query_one_by_id, query_all_ordered};
use rusqlite::{Connection, Row};

pub struct RequirementExpressionRepository;

impl RequirementExpressionRepository {
    const TABLE: &'static str = "requirement_expressions";
    const COLUMNS: &'static str = "id, parent_id, node_type, requirement_id, position, created_at, updated_at";

    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<RequirementExpression>> {
        query_one_by_id(conn, Self::TABLE, Self::COLUMNS, id, Self::row_to_model)
    }

    pub fn get_all(conn: &Connection) -> Result<Vec<RequirementExpression>> {
        query_all_ordered(conn, Self::TABLE, Self::COLUMNS, "id", Self::row_to_model)
    }

    fn row_to_model(row: &Row) -> rusqlite::Result<RequirementExpression> {
        Ok(RequirementExpression {
            id: row.get(0)?,
            parent_id: row.get(1)?,
            node_type: row.get(2)?,
            requirement_id: row.get(3)?,
            position: row.get(4)?,
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
        let schema = include_str!("../../../db/schema.sql");
        conn.execute_batch(schema).unwrap();
        conn
    }

    fn insert_test_requirement(conn: &Connection) {
        conn.execute(
            "INSERT INTO requirements (id, requirement_type, target_id, value, description, created_at, updated_at)
             VALUES (1, 'rank', 1, 1, 'Novice', '2024-01-01', '2024-01-01')",
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

    fn insert_test_operator_expression(conn: &Connection) {
        conn.execute(
            "INSERT INTO requirement_expressions (id, parent_id, node_type, requirement_id, position, created_at, updated_at)
             VALUES (2, NULL, 'and', NULL, 0, '2024-01-01', '2024-01-01')",
            [],
        ).unwrap();
    }

    #[test]
    fn test_get_by_id_found() {
        let conn = setup_test_db();
        insert_test_requirement(&conn);
        insert_test_requirement_expression(&conn);

        let result = RequirementExpressionRepository::get_by_id(&conn, 1).unwrap();

        assert!(result.is_some());
        let expr = result.unwrap();
        assert_eq!(expr.id, 1);
        assert_eq!(expr.parent_id, None);
        assert_eq!(expr.node_type, "requirement");
        assert_eq!(expr.requirement_id, Some(1));
        assert_eq!(expr.position, 0);
    }

    #[test]
    fn test_get_by_id_not_found() {
        let conn = setup_test_db();

        let result = RequirementExpressionRepository::get_by_id(&conn, 999).unwrap();

        assert!(result.is_none());
    }

    #[test]
    fn test_get_by_id_operator_node() {
        let conn = setup_test_db();
        insert_test_operator_expression(&conn);

        let result = RequirementExpressionRepository::get_by_id(&conn, 2).unwrap();

        assert!(result.is_some());
        let expr = result.unwrap();
        assert_eq!(expr.id, 2);
        assert_eq!(expr.node_type, "and");
        assert_eq!(expr.requirement_id, None);
    }

    #[test]
    fn test_get_all() {
        let conn = setup_test_db();
        insert_test_requirement(&conn);
        insert_test_requirement_expression(&conn);
        insert_test_operator_expression(&conn);

        let expressions = RequirementExpressionRepository::get_all(&conn).unwrap();

        assert_eq!(expressions.len(), 2);
        assert_eq!(expressions[0].id, 1);
        assert_eq!(expressions[1].id, 2);
    }

    #[test]
    fn test_get_all_empty() {
        let conn = setup_test_db();

        let expressions = RequirementExpressionRepository::get_all(&conn).unwrap();

        assert_eq!(expressions.len(), 0);
    }

    #[test]
    fn test_row_to_model_with_parent() {
        let conn = setup_test_db();
        insert_test_requirement(&conn);
        insert_test_operator_expression(&conn);

        // Insert a child expression with parent_id = 2
        conn.execute(
            "INSERT INTO requirement_expressions (id, parent_id, node_type, requirement_id, position, created_at, updated_at)
             VALUES (3, 2, 'requirement', 1, 1, '2024-01-01', '2024-01-01')",
            [],
        ).unwrap();

        let result = RequirementExpressionRepository::get_by_id(&conn, 3).unwrap();

        assert!(result.is_some());
        let expr = result.unwrap();
        assert_eq!(expr.id, 3);
        assert_eq!(expr.parent_id, Some(2));
        assert_eq!(expr.position, 1);
    }
}
