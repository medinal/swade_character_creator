use crate::error::Result;
use crate::models::Power;
use super::base_repository::{query_one_by_id, query_all_ordered};
use rusqlite::{Connection, Row};

pub struct PowerRepository;

impl PowerRepository {
    const TABLE: &'static str = "powers";
    const COLUMNS: &'static str = "id, name, power_points, range, duration, source, description,
                                   created_at, updated_at";

    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<Power>> {
        query_one_by_id(conn, Self::TABLE, Self::COLUMNS, id, Self::row_to_model)
    }

    pub fn get_all(conn: &Connection) -> Result<Vec<Power>> {
        query_all_ordered(conn, Self::TABLE, Self::COLUMNS, "name", Self::row_to_model)
    }

    fn row_to_model(row: &Row) -> rusqlite::Result<Power> {
        Ok(Power {
            id: row.get(0)?,
            name: row.get(1)?,
            power_points: row.get(2)?,
            range: row.get(3)?,
            duration: row.get(4)?,
            source: row.get(5)?,
            description: row.get(6)?,
            created_at: row.get(7)?,
            updated_at: row.get(8)?,
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
            "INSERT INTO powers (id, name, power_points, range, duration, source, description,
                                created_at, updated_at)
             VALUES (1, 'Bolt', 1, 'Smarts × 2', 'Instant', 'core', 'Ranged elemental attack',
                     '2024-01-01', '2024-01-01')",
            [],
        )
        .unwrap();
    }

    fn insert_multiple_test_powers(conn: &Connection) {
        conn.execute(
            "INSERT INTO powers (id, name, power_points, range, duration, source, description,
                                created_at, updated_at)
             VALUES (1, 'Bolt', 1, 'Smarts × 2', 'Instant', 'core', 'Ranged elemental attack',
                     '2024-01-01', '2024-01-01')",
            [],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO powers (id, name, power_points, range, duration, source, description,
                                created_at, updated_at)
             VALUES (2, 'Healing', 3, 'Touch', 'Instant', 'core', 'Heal wounds',
                     '2024-01-01', '2024-01-01')",
            [],
        )
        .unwrap();
        conn.execute(
            "INSERT INTO powers (id, name, power_points, range, duration, source, description,
                                created_at, updated_at)
             VALUES (3, 'Barrier', 2, 'Smarts', '5 rounds', 'core', 'Create protective wall',
                     '2024-01-01', '2024-01-01')",
            [],
        )
        .unwrap();
    }

    #[test]
    fn test_get_by_id_found() {
        let conn = setup_test_db();
        insert_test_power(&conn);

        let result = PowerRepository::get_by_id(&conn, 1).unwrap();

        assert!(result.is_some());
        let power = result.unwrap();
        assert_eq!(power.id, 1);
        assert_eq!(power.name, "Bolt");
        assert_eq!(power.power_points, 1);
        assert_eq!(power.range, "Smarts × 2");
        assert_eq!(power.duration, "Instant");
        assert_eq!(power.source, "core");
        assert_eq!(power.description, "Ranged elemental attack");
    }

    #[test]
    fn test_get_by_id_not_found() {
        let conn = setup_test_db();

        let result = PowerRepository::get_by_id(&conn, 999).unwrap();

        assert!(result.is_none());
    }

    #[test]
    fn test_get_all_empty() {
        let conn = setup_test_db();

        let powers = PowerRepository::get_all(&conn).unwrap();

        assert_eq!(powers.len(), 0);
    }

    #[test]
    fn test_get_all_single() {
        let conn = setup_test_db();
        insert_test_power(&conn);

        let powers = PowerRepository::get_all(&conn).unwrap();

        assert_eq!(powers.len(), 1);
        assert_eq!(powers[0].name, "Bolt");
    }

    #[test]
    fn test_get_all_multiple_sorted() {
        let conn = setup_test_db();
        insert_multiple_test_powers(&conn);

        let powers = PowerRepository::get_all(&conn).unwrap();

        assert_eq!(powers.len(), 3);
        // Should be sorted alphabetically by name
        assert_eq!(powers[0].name, "Barrier");
        assert_eq!(powers[1].name, "Bolt");
        assert_eq!(powers[2].name, "Healing");
    }

    #[test]
    fn test_row_to_model_all_fields() {
        let conn = setup_test_db();
        insert_test_power(&conn);

        let result = PowerRepository::get_by_id(&conn, 1).unwrap();

        assert!(result.is_some());
        let power = result.unwrap();
        // Verify all fields are correctly mapped
        assert_eq!(power.id, 1);
        assert_eq!(power.name, "Bolt");
        assert_eq!(power.power_points, 1);
        assert_eq!(power.range, "Smarts × 2");
        assert_eq!(power.duration, "Instant");
        assert_eq!(power.source, "core");
        assert_eq!(power.description, "Ranged elemental attack");
        assert_eq!(power.created_at, "2024-01-01");
        assert_eq!(power.updated_at, "2024-01-01");
    }
}
