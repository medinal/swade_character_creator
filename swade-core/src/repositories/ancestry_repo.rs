use crate::error::Result;
use crate::models::Ancestry;
use super::base_repository::{query_one_by_id, query_all_ordered};
use rusqlite::{Connection, Row};

pub struct AncestryRepository;

impl AncestryRepository {
    const TABLE: &'static str = "ancestries";
    const COLUMNS: &'static str = "id, name, source, description, created_at, updated_at";

    /// Get a single ancestry by ID
    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<Ancestry>> {
        query_one_by_id(conn, Self::TABLE, Self::COLUMNS, id, Self::row_to_model)
    }

    /// Get all ancestries ordered by name
    pub fn get_all(conn: &Connection) -> Result<Vec<Ancestry>> {
        query_all_ordered(conn, Self::TABLE, Self::COLUMNS, "name", Self::row_to_model)
    }

    /// Convert a database row to an Ancestry model
    fn row_to_model(row: &Row) -> rusqlite::Result<Ancestry> {
        Ok(Ancestry {
            id: row.get(0)?,
            name: row.get(1)?,
            source: row.get(2)?,
            description: row.get(3)?,
            created_at: row.get(4)?,
            updated_at: row.get(5)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::setup_test_db;
    use rusqlite::params;

    // Local helper with custom signature for ancestry-specific tests
    fn insert_test_ancestry(conn: &Connection, id: i64, name: &str, source: &str) {
        conn.execute(
            "INSERT INTO ancestries (id, name, source, description, created_at, updated_at)
             VALUES (?, ?, ?, ?, '2024-01-01', '2024-01-01')",
            params![id, name, source, format!("Description for {}", name)],
        )
        .unwrap();
    }

    #[test]
    fn test_get_by_id_found() {
        let conn = setup_test_db();
        insert_test_ancestry(&conn, 1, "Human", "core");

        let result = AncestryRepository::get_by_id(&conn, 1).unwrap();

        assert!(result.is_some());
        let ancestry = result.unwrap();
        assert_eq!(ancestry.id, 1);
        assert_eq!(ancestry.name, "Human");
        assert_eq!(ancestry.source, "core");
        assert_eq!(ancestry.description, "Description for Human");
        assert_eq!(ancestry.created_at, "2024-01-01");
        assert_eq!(ancestry.updated_at, "2024-01-01");
    }

    #[test]
    fn test_get_by_id_not_found() {
        let conn = setup_test_db();

        let result = AncestryRepository::get_by_id(&conn, 999).unwrap();

        assert!(result.is_none());
    }

    #[test]
    fn test_get_all_empty() {
        let conn = setup_test_db();

        let ancestries = AncestryRepository::get_all(&conn).unwrap();

        assert_eq!(ancestries.len(), 0);
    }

    #[test]
    fn test_get_all_ordered_by_name() {
        let conn = setup_test_db();
        insert_test_ancestry(&conn, 1, "Half-Elf", "core");
        insert_test_ancestry(&conn, 2, "Dwarf", "core");
        insert_test_ancestry(&conn, 3, "Elf", "core");
        insert_test_ancestry(&conn, 4, "Human", "core");

        let ancestries = AncestryRepository::get_all(&conn).unwrap();

        assert_eq!(ancestries.len(), 4);
        // Verify ordering by name
        assert_eq!(ancestries[0].name, "Dwarf");
        assert_eq!(ancestries[1].name, "Elf");
        assert_eq!(ancestries[2].name, "Half-Elf");
        assert_eq!(ancestries[3].name, "Human");
    }

    #[test]
    fn test_get_all_multiple_sources() {
        let conn = setup_test_db();
        insert_test_ancestry(&conn, 1, "Human", "core");
        insert_test_ancestry(&conn, 2, "Android", "sci-fi");
        insert_test_ancestry(&conn, 3, "Elf", "fantasy");

        let ancestries = AncestryRepository::get_all(&conn).unwrap();

        assert_eq!(ancestries.len(), 3);
        // Find each by name to verify they all loaded correctly
        let android = ancestries.iter().find(|a| a.name == "Android").unwrap();
        assert_eq!(android.source, "sci-fi");

        let elf = ancestries.iter().find(|a| a.name == "Elf").unwrap();
        assert_eq!(elf.source, "fantasy");

        let human = ancestries.iter().find(|a| a.name == "Human").unwrap();
        assert_eq!(human.source, "core");
    }

    #[test]
    fn test_row_to_model_all_fields() {
        let conn = setup_test_db();
        insert_test_ancestry(&conn, 42, "Test Ancestry", "test-source");

        let ancestry = AncestryRepository::get_by_id(&conn, 42).unwrap().unwrap();

        // Verify all fields are correctly mapped
        assert_eq!(ancestry.id, 42);
        assert_eq!(ancestry.name, "Test Ancestry");
        assert_eq!(ancestry.source, "test-source");
        assert_eq!(ancestry.description, "Description for Test Ancestry");
        assert_eq!(ancestry.created_at, "2024-01-01");
        assert_eq!(ancestry.updated_at, "2024-01-01");
    }
}
