use crate::error::Result;
use crate::models::PowerRequirement;
use super::base_repository::{query_one_by_id, query_by_fk_id};
use rusqlite::{Connection, Row};

pub struct PowerRequirementRepository;

impl PowerRequirementRepository {
    const TABLE: &'static str = "power_requirements";
    const COLUMNS: &'static str = "id, power_id, requirement_expression_id, created_at, updated_at";

    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<PowerRequirement>> {
        query_one_by_id(conn, Self::TABLE, Self::COLUMNS, id, Self::row_to_model)
    }

    pub fn get_by_power_id(conn: &Connection, power_id: i64) -> Result<Vec<PowerRequirement>> {
        query_by_fk_id(conn, Self::TABLE, Self::COLUMNS, "power_id", power_id, "id", Self::row_to_model)
    }

    fn row_to_model(row: &Row) -> rusqlite::Result<PowerRequirement> {
        Ok(PowerRequirement {
            id: row.get(0)?,
            power_id: row.get(1)?,
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

    fn insert_test_power(conn: &Connection) {
        conn.execute(
            "INSERT INTO powers (id, name, power_points, range, duration, source, description, created_at, updated_at)
             VALUES (1, 'Bolt', 1, 'Smarts x 2', '5', 'core', 'Fire a lightning bolt', '2024-01-01', '2024-01-01')",
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

    fn insert_test_power_requirement(conn: &Connection) {
        conn.execute(
            "INSERT INTO power_requirements (id, power_id, requirement_expression_id, created_at, updated_at)
             VALUES (1, 1, 1, '2024-01-01', '2024-01-01')",
            [],
        ).unwrap();
    }

    #[test]
    fn test_get_by_id_found() {
        let conn = setup_test_db();
        insert_test_power(&conn);
        insert_test_requirement(&conn);
        insert_test_requirement_expression(&conn);
        insert_test_power_requirement(&conn);

        let result = PowerRequirementRepository::get_by_id(&conn, 1).unwrap();

        assert!(result.is_some());
        let power_req = result.unwrap();
        assert_eq!(power_req.id, 1);
        assert_eq!(power_req.power_id, 1);
        assert_eq!(power_req.requirement_expression_id, 1);
    }

    #[test]
    fn test_get_by_id_not_found() {
        let conn = setup_test_db();

        let result = PowerRequirementRepository::get_by_id(&conn, 999).unwrap();

        assert!(result.is_none());
    }

    #[test]
    fn test_get_by_power_id_multiple_requirements() {
        let conn = setup_test_db();
        insert_test_power(&conn);
        insert_test_requirement(&conn);
        insert_test_requirement_expression(&conn);

        // Insert another requirement and expression
        conn.execute(
            "INSERT INTO requirements (id, requirement_type, target_id, value, description, created_at, updated_at)
             VALUES (2, 'arcane_background', 1, NULL, 'Must have an Arcane Background', '2024-01-01', '2024-01-01')",
            [],
        ).unwrap();
        conn.execute(
            "INSERT INTO requirement_expressions (id, parent_id, node_type, requirement_id, position, created_at, updated_at)
             VALUES (2, NULL, 'requirement', 2, 0, '2024-01-01', '2024-01-01')",
            [],
        ).unwrap();

        // Insert multiple power requirements
        insert_test_power_requirement(&conn);
        conn.execute(
            "INSERT INTO power_requirements (id, power_id, requirement_expression_id, created_at, updated_at)
             VALUES (2, 1, 2, '2024-01-01', '2024-01-01')",
            [],
        ).unwrap();

        let requirements = PowerRequirementRepository::get_by_power_id(&conn, 1).unwrap();

        assert_eq!(requirements.len(), 2);
        assert_eq!(requirements[0].power_id, 1);
        assert_eq!(requirements[1].power_id, 1);
    }

    #[test]
    fn test_get_by_power_id_empty() {
        let conn = setup_test_db();

        let requirements = PowerRequirementRepository::get_by_power_id(&conn, 999).unwrap();

        assert_eq!(requirements.len(), 0);
    }

    #[test]
    fn test_get_by_power_id_filters_correctly() {
        let conn = setup_test_db();
        insert_test_power(&conn);

        // Insert second power
        conn.execute(
            "INSERT INTO powers (id, name, power_points, range, duration, source, description, created_at, updated_at)
             VALUES (2, 'Healing', 3, 'Touch', '5', 'core', 'Heal wounds', '2024-01-01', '2024-01-01')",
            [],
        ).unwrap();

        insert_test_requirement(&conn);
        insert_test_requirement_expression(&conn);

        // Insert requirements for both powers
        insert_test_power_requirement(&conn);
        conn.execute(
            "INSERT INTO power_requirements (id, power_id, requirement_expression_id, created_at, updated_at)
             VALUES (2, 2, 1, '2024-01-01', '2024-01-01')",
            [],
        ).unwrap();

        let requirements = PowerRequirementRepository::get_by_power_id(&conn, 1).unwrap();

        assert_eq!(requirements.len(), 1);
        assert_eq!(requirements[0].power_id, 1);
    }
}
