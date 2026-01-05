use crate::error::Result;
use crate::models::PowerModifier;
use super::base_repository::{query_one_by_id, query_by_fk_id};
use rusqlite::{Connection, Row};

pub struct PowerModifierRepository;

impl PowerModifierRepository {
    const TABLE: &'static str = "power_modifiers";
    const COLUMNS: &'static str = "id, power_id, modifier_id, created_at, updated_at";

    /// Get a single power modifier by ID
    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<PowerModifier>> {
        query_one_by_id(conn, Self::TABLE, Self::COLUMNS, id, Self::row_to_model)
    }

    /// Get all power modifiers for a specific power
    pub fn get_by_power_id(conn: &Connection, power_id: i64) -> Result<Vec<PowerModifier>> {
        query_by_fk_id(conn, Self::TABLE, Self::COLUMNS, "power_id", power_id, "id", Self::row_to_model)
    }

    /// Convert a database row to a PowerModifier model
    fn row_to_model(row: &Row) -> rusqlite::Result<PowerModifier> {
        Ok(PowerModifier {
            id: row.get(0)?,
            power_id: row.get(1)?,
            modifier_id: row.get(2)?,
            created_at: row.get(3)?,
            updated_at: row.get(4)?,
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

    fn insert_test_power(conn: &Connection, id: i64, name: &str) {
        conn.execute(
            "INSERT INTO powers (id, name, power_points, range, duration, source, description,
                                created_at, updated_at)
             VALUES (?, ?, 2, 'Smarts', '5', 'core', 'Test description',
                     '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            params![id, name],
        )
        .unwrap();
    }

    fn insert_test_modifier(conn: &Connection, id: i64) {
        conn.execute(
            "INSERT INTO modifiers (id, target_type, target_identifier, value_type,
                                   value, description, created_at, updated_at)
             VALUES (?, 'derived_stat', 'damage', 'roll_bonus',
                     2, 'Test modifier', '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            params![id],
        )
        .unwrap();
    }

    fn insert_test_power_modifier(conn: &Connection, id: i64, power_id: i64, modifier_id: i64) {
        conn.execute(
            "INSERT INTO power_modifiers (id, power_id, modifier_id, created_at, updated_at)
             VALUES (?, ?, ?, '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            params![id, power_id, modifier_id],
        )
        .unwrap();
    }

    #[test]
    fn test_get_by_id_found() {
        let conn = setup_test_db();
        insert_test_power(&conn, 1, "Bolt");
        insert_test_modifier(&conn, 1);
        insert_test_power_modifier(&conn, 1, 1, 1);

        let result = PowerModifierRepository::get_by_id(&conn, 1).unwrap();

        assert!(result.is_some());
        let power_modifier = result.unwrap();
        assert_eq!(power_modifier.id, 1);
        assert_eq!(power_modifier.power_id, 1);
        assert_eq!(power_modifier.modifier_id, 1);
        assert_eq!(power_modifier.created_at, "2024-01-01 00:00:00");
        assert_eq!(power_modifier.updated_at, "2024-01-01 00:00:00");
    }

    #[test]
    fn test_get_by_id_not_found() {
        let conn = setup_test_db();

        let result = PowerModifierRepository::get_by_id(&conn, 999).unwrap();

        assert!(result.is_none());
    }

    #[test]
    fn test_get_by_power_id_multiple_modifiers() {
        let conn = setup_test_db();
        insert_test_power(&conn, 1, "Blast");
        insert_test_modifier(&conn, 1);
        insert_test_modifier(&conn, 2);

        insert_test_power_modifier(&conn, 1, 1, 1);
        insert_test_power_modifier(&conn, 2, 1, 2);

        let modifiers = PowerModifierRepository::get_by_power_id(&conn, 1).unwrap();

        assert_eq!(modifiers.len(), 2);
        assert_eq!(modifiers[0].power_id, 1);
        assert_eq!(modifiers[0].modifier_id, 1);
        assert_eq!(modifiers[1].power_id, 1);
        assert_eq!(modifiers[1].modifier_id, 2);
    }

    #[test]
    fn test_get_by_power_id_empty() {
        let conn = setup_test_db();

        let modifiers = PowerModifierRepository::get_by_power_id(&conn, 999).unwrap();

        assert_eq!(modifiers.len(), 0);
    }

    #[test]
    fn test_get_by_power_id_filters_correctly() {
        let conn = setup_test_db();
        insert_test_power(&conn, 1, "Bolt");
        insert_test_power(&conn, 2, "Healing");
        insert_test_modifier(&conn, 1);

        insert_test_power_modifier(&conn, 1, 1, 1);
        insert_test_power_modifier(&conn, 2, 2, 1);

        let modifiers = PowerModifierRepository::get_by_power_id(&conn, 1).unwrap();

        assert_eq!(modifiers.len(), 1);
        assert_eq!(modifiers[0].power_id, 1);
    }

    #[test]
    fn test_row_to_model_all_fields() {
        let conn = setup_test_db();
        insert_test_power(&conn, 42, "Test Power");
        insert_test_modifier(&conn, 99);

        conn.execute(
            "INSERT INTO power_modifiers (id, power_id, modifier_id, created_at, updated_at)
             VALUES (100, 42, 99, '2024-12-25 10:30:00', '2024-12-26 15:45:00')",
            [],
        )
        .unwrap();

        let power_modifier = PowerModifierRepository::get_by_id(&conn, 100)
            .unwrap()
            .unwrap();

        assert_eq!(power_modifier.id, 100);
        assert_eq!(power_modifier.power_id, 42);
        assert_eq!(power_modifier.modifier_id, 99);
        assert_eq!(power_modifier.created_at, "2024-12-25 10:30:00");
        assert_eq!(power_modifier.updated_at, "2024-12-26 15:45:00");
    }

    #[test]
    fn test_get_by_power_id_with_single_modifier() {
        let conn = setup_test_db();
        insert_test_power(&conn, 1, "Barrier");
        insert_test_modifier(&conn, 1);
        insert_test_power_modifier(&conn, 1, 1, 1);

        let modifiers = PowerModifierRepository::get_by_power_id(&conn, 1).unwrap();

        assert_eq!(modifiers.len(), 1);
        assert_eq!(modifiers[0].power_id, 1);
        assert_eq!(modifiers[0].modifier_id, 1);
    }

    #[test]
    fn test_unique_constraint_power_modifier_pair() {
        let conn = setup_test_db();
        insert_test_power(&conn, 1, "Bolt");
        insert_test_modifier(&conn, 1);
        insert_test_power_modifier(&conn, 1, 1, 1);

        // Attempting to insert duplicate power_id/modifier_id pair should fail
        let result = conn.execute(
            "INSERT INTO power_modifiers (power_id, modifier_id, created_at, updated_at)
             VALUES (1, 1, '2024-01-01 00:00:00', '2024-01-01 00:00:00')",
            [],
        );

        assert!(result.is_err());
    }
}
