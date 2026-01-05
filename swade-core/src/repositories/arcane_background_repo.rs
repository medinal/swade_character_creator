use crate::error::Result;
use crate::models::ArcaneBackground;
use super::base_repository::{query_one_by_id, query_all_ordered};
use rusqlite::{Connection, Row};

pub struct ArcaneBackgroundRepository;

impl ArcaneBackgroundRepository {
    const TABLE: &'static str = "arcane_backgrounds";
    const COLUMNS: &'static str = "id, name, arcane_skill_id, starting_powers, starting_power_points,
                                   has_power_list, source, description, created_at, updated_at";

    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<ArcaneBackground>> {
        query_one_by_id(conn, Self::TABLE, Self::COLUMNS, id, Self::row_to_model)
    }

    pub fn get_all(conn: &Connection) -> Result<Vec<ArcaneBackground>> {
        query_all_ordered(conn, Self::TABLE, Self::COLUMNS, "name", Self::row_to_model)
    }

    fn row_to_model(row: &Row) -> rusqlite::Result<ArcaneBackground> {
        Ok(ArcaneBackground {
            id: row.get(0)?,
            name: row.get(1)?,
            arcane_skill_id: row.get(2)?,
            starting_powers: row.get(3)?,
            starting_power_points: row.get(4)?,
            has_power_list: row.get(5)?,
            source: row.get(6)?,
            description: row.get(7)?,
            created_at: row.get(8)?,
            updated_at: row.get(9)?,
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
            "INSERT INTO attributes (id, name, description, base_value, created_at, updated_at)
             VALUES (1, 'Smarts', 'Intelligence', 4, '2024-01-01', '2024-01-01')",
            [],
        )
        .unwrap();
    }

    fn insert_test_skill(conn: &Connection) {
        conn.execute(
            "INSERT INTO skills (id, name, linked_attribute_id, description, created_at, updated_at)
             VALUES (1, 'Spellcasting', 1, 'Magic skill', '2024-01-01', '2024-01-01')",
            [],
        ).unwrap();
    }

    fn insert_test_arcane_background(conn: &Connection) {
        conn.execute(
            "INSERT INTO arcane_backgrounds (id, name, arcane_skill_id, starting_powers,
                                            starting_power_points, source, description,
                                            created_at, updated_at)
             VALUES (1, 'Magic', 1, 3, 10, 'core', 'Wizards and sorcerers',
                     '2024-01-01', '2024-01-01')",
            [],
        )
        .unwrap();
    }

    #[test]
    fn test_get_by_id_found() {
        let conn = setup_test_db();
        insert_test_attribute(&conn);
        insert_test_skill(&conn);
        insert_test_arcane_background(&conn);

        let result = ArcaneBackgroundRepository::get_by_id(&conn, 1).unwrap();

        assert!(result.is_some());
        let arcane_bg = result.unwrap();
        assert_eq!(arcane_bg.name, "Magic");
        assert_eq!(arcane_bg.arcane_skill_id, 1);
        assert_eq!(arcane_bg.starting_powers, 3);
        assert_eq!(arcane_bg.starting_power_points, 10);
        assert_eq!(arcane_bg.source, "core");
        assert_eq!(arcane_bg.description, "Wizards and sorcerers");
    }

    #[test]
    fn test_get_by_id_not_found() {
        let conn = setup_test_db();

        let result = ArcaneBackgroundRepository::get_by_id(&conn, 999).unwrap();

        assert!(result.is_none());
    }

    #[test]
    fn test_get_all() {
        let conn = setup_test_db();
        insert_test_attribute(&conn);
        insert_test_skill(&conn);
        insert_test_arcane_background(&conn);

        // Insert another arcane background
        conn.execute(
            "INSERT INTO arcane_backgrounds (id, name, arcane_skill_id, starting_powers,
                                            starting_power_points, source, description,
                                            created_at, updated_at)
             VALUES (2, 'Miracles', 1, 3, 10, 'core', 'Divine powers',
                     '2024-01-01', '2024-01-01')",
            [],
        )
        .unwrap();

        let arcane_backgrounds = ArcaneBackgroundRepository::get_all(&conn).unwrap();

        assert_eq!(arcane_backgrounds.len(), 2);
        // Verify ordering by name
        assert_eq!(arcane_backgrounds[0].name, "Magic");
        assert_eq!(arcane_backgrounds[1].name, "Miracles");
    }

    #[test]
    fn test_get_all_empty() {
        let conn = setup_test_db();

        let arcane_backgrounds = ArcaneBackgroundRepository::get_all(&conn).unwrap();

        assert_eq!(arcane_backgrounds.len(), 0);
    }
}
