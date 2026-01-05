use crate::error::Result;
use crate::models::AncestryRequirement;
use super::base_repository::{query_one_by_id, query_all_ordered, query_by_fk_id};
use rusqlite::{Connection, Row};

pub struct AncestryRequirementRepository;

impl AncestryRequirementRepository {
    const TABLE: &'static str = "ancestry_requirements";
    const COLUMNS: &'static str = "id, ancestry_id, requirement_expression_id, created_at, updated_at";

    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<AncestryRequirement>> {
        query_one_by_id(conn, Self::TABLE, Self::COLUMNS, id, Self::row_to_model)
    }

    pub fn get_all(conn: &Connection) -> Result<Vec<AncestryRequirement>> {
        query_all_ordered(conn, Self::TABLE, Self::COLUMNS, "ancestry_id, id", Self::row_to_model)
    }

    pub fn get_by_ancestry_id(
        conn: &Connection,
        ancestry_id: i64,
    ) -> Result<Vec<AncestryRequirement>> {
        query_by_fk_id(conn, Self::TABLE, Self::COLUMNS, "ancestry_id", ancestry_id, "id", Self::row_to_model)
    }

    fn row_to_model(row: &Row) -> rusqlite::Result<AncestryRequirement> {
        Ok(AncestryRequirement {
            id: row.get(0)?,
            ancestry_id: row.get(1)?,
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

    fn insert_test_ancestry(conn: &Connection) {
        conn.execute(
            "INSERT INTO ancestries (id, name, source, description, created_at, updated_at)
             VALUES (1, 'Human', 'core', 'Basic human ancestry', '2024-01-01', '2024-01-01')",
            [],
        )
        .unwrap();
    }

    fn insert_test_requirement_expression(conn: &Connection) {
        // Insert a requirement first
        conn.execute(
            "INSERT INTO requirements (id, requirement_type, target_id, value, description, created_at, updated_at)
             VALUES (1, 'rank', 1, 0, 'Must be at least Novice rank', '2024-01-01', '2024-01-01')",
            [],
        ).unwrap();

        // Insert the requirement expression
        conn.execute(
            "INSERT INTO requirement_expressions (id, parent_id, node_type, requirement_id, position,
                                                  created_at, updated_at)
             VALUES (1, NULL, 'requirement', 1, 0, '2024-01-01', '2024-01-01')",
            [],
        ).unwrap();
    }

    fn insert_test_ancestry_requirement(conn: &Connection) {
        conn.execute(
            "INSERT INTO ancestry_requirements (id, ancestry_id, requirement_expression_id, created_at, updated_at)
             VALUES (1, 1, 1, '2024-01-01', '2024-01-01')",
            [],
        ).unwrap();
    }

    #[test]
    fn test_get_by_id_found() {
        let conn = setup_test_db();
        insert_test_ancestry(&conn);
        insert_test_requirement_expression(&conn);
        insert_test_ancestry_requirement(&conn);

        let result = AncestryRequirementRepository::get_by_id(&conn, 1).unwrap();

        assert!(result.is_some());
        let ancestry_req = result.unwrap();
        assert_eq!(ancestry_req.id, 1);
        assert_eq!(ancestry_req.ancestry_id, 1);
        assert_eq!(ancestry_req.requirement_expression_id, 1);
    }

    #[test]
    fn test_get_by_id_not_found() {
        let conn = setup_test_db();

        let result = AncestryRequirementRepository::get_by_id(&conn, 999).unwrap();

        assert!(result.is_none());
    }

    #[test]
    fn test_get_all() {
        let conn = setup_test_db();
        insert_test_ancestry(&conn);
        insert_test_requirement_expression(&conn);
        insert_test_ancestry_requirement(&conn);

        let requirements = AncestryRequirementRepository::get_all(&conn).unwrap();

        assert_eq!(requirements.len(), 1);
        assert_eq!(requirements[0].id, 1);
    }

    #[test]
    fn test_get_all_empty() {
        let conn = setup_test_db();

        let requirements = AncestryRequirementRepository::get_all(&conn).unwrap();

        assert_eq!(requirements.len(), 0);
    }

    #[test]
    fn test_get_all_orders_by_ancestry_then_id() {
        let conn = setup_test_db();
        insert_test_ancestry(&conn);

        // Insert second ancestry
        conn.execute(
            "INSERT INTO ancestries (id, name, source, description, created_at, updated_at)
             VALUES (2, 'Elf', 'core', 'Elven ancestry', '2024-01-01', '2024-01-01')",
            [],
        )
        .unwrap();

        // Insert multiple requirement expressions
        insert_test_requirement_expression(&conn);
        conn.execute(
            "INSERT INTO requirements (id, requirement_type, target_id, value, description, created_at, updated_at)
             VALUES (2, 'attribute', 1, 8, 'Agility d8 or higher', '2024-01-01', '2024-01-01')",
            [],
        ).unwrap();
        conn.execute(
            "INSERT INTO requirement_expressions (id, parent_id, node_type, requirement_id, position,
                                                  created_at, updated_at)
             VALUES (2, NULL, 'requirement', 2, 0, '2024-01-01', '2024-01-01')",
            [],
        ).unwrap();

        // Insert ancestry requirements in non-sequential order to test sorting
        conn.execute(
            "INSERT INTO ancestry_requirements (id, ancestry_id, requirement_expression_id, created_at, updated_at)
             VALUES (3, 2, 1, '2024-01-01', '2024-01-01')",
            [],
        ).unwrap();
        conn.execute(
            "INSERT INTO ancestry_requirements (id, ancestry_id, requirement_expression_id, created_at, updated_at)
             VALUES (1, 1, 1, '2024-01-01', '2024-01-01')",
            [],
        ).unwrap();
        conn.execute(
            "INSERT INTO ancestry_requirements (id, ancestry_id, requirement_expression_id, created_at, updated_at)
             VALUES (2, 1, 2, '2024-01-01', '2024-01-01')",
            [],
        ).unwrap();

        let requirements = AncestryRequirementRepository::get_all(&conn).unwrap();

        assert_eq!(requirements.len(), 3);
        // Should be ordered by ancestry_id first, then id
        assert_eq!(requirements[0].ancestry_id, 1);
        assert_eq!(requirements[0].id, 1);
        assert_eq!(requirements[1].ancestry_id, 1);
        assert_eq!(requirements[1].id, 2);
        assert_eq!(requirements[2].ancestry_id, 2);
        assert_eq!(requirements[2].id, 3);
    }

    #[test]
    fn test_get_by_ancestry_id_multiple_requirements() {
        let conn = setup_test_db();
        insert_test_ancestry(&conn);
        insert_test_requirement_expression(&conn);

        // Insert multiple requirement expressions
        conn.execute(
            "INSERT INTO requirements (id, requirement_type, target_id, value, description, created_at, updated_at)
             VALUES (2, 'attribute', 1, 8, 'Agility d8 or higher', '2024-01-01', '2024-01-01')",
            [],
        ).unwrap();
        conn.execute(
            "INSERT INTO requirement_expressions (id, parent_id, node_type, requirement_id, position,
                                                  created_at, updated_at)
             VALUES (2, NULL, 'requirement', 2, 0, '2024-01-01', '2024-01-01')",
            [],
        ).unwrap();

        // Insert multiple ancestry requirements
        insert_test_ancestry_requirement(&conn);
        conn.execute(
            "INSERT INTO ancestry_requirements (id, ancestry_id, requirement_expression_id, created_at, updated_at)
             VALUES (2, 1, 2, '2024-01-01', '2024-01-01')",
            [],
        ).unwrap();

        let requirements = AncestryRequirementRepository::get_by_ancestry_id(&conn, 1).unwrap();

        assert_eq!(requirements.len(), 2);
        assert_eq!(requirements[0].ancestry_id, 1);
        assert_eq!(requirements[1].ancestry_id, 1);
    }

    #[test]
    fn test_get_by_ancestry_id_empty() {
        let conn = setup_test_db();

        let requirements = AncestryRequirementRepository::get_by_ancestry_id(&conn, 999).unwrap();

        assert_eq!(requirements.len(), 0);
    }

    #[test]
    fn test_get_by_ancestry_id_filters_correctly() {
        let conn = setup_test_db();
        insert_test_ancestry(&conn);

        // Insert second ancestry
        conn.execute(
            "INSERT INTO ancestries (id, name, source, description, created_at, updated_at)
             VALUES (2, 'Elf', 'core', 'Elven ancestry', '2024-01-01', '2024-01-01')",
            [],
        )
        .unwrap();

        insert_test_requirement_expression(&conn);

        // Insert requirements for both ancestries
        insert_test_ancestry_requirement(&conn);
        conn.execute(
            "INSERT INTO ancestry_requirements (id, ancestry_id, requirement_expression_id, created_at, updated_at)
             VALUES (2, 2, 1, '2024-01-01', '2024-01-01')",
            [],
        ).unwrap();

        let requirements = AncestryRequirementRepository::get_by_ancestry_id(&conn, 1).unwrap();

        assert_eq!(requirements.len(), 1);
        assert_eq!(requirements[0].ancestry_id, 1);
    }
}
