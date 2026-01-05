use crate::error::Result;
use crate::models::Attribute;
use super::base_repository::{query_one_by_id, query_all_ordered, query_one_by_field};
use rusqlite::{Connection, Row};

pub struct AttributeRepository;

impl AttributeRepository {
    const TABLE: &'static str = "attributes";
    const COLUMNS: &'static str = "id, name, description, base_value, created_at, updated_at";

    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<Attribute>> {
        query_one_by_id(conn, Self::TABLE, Self::COLUMNS, id, Self::row_to_model)
    }

    pub fn get_all(conn: &Connection) -> Result<Vec<Attribute>> {
        query_all_ordered(conn, Self::TABLE, Self::COLUMNS, "name", Self::row_to_model)
    }

    /// Get an attribute by name (case-insensitive)
    pub fn get_by_name(conn: &Connection, name: &str) -> Result<Option<Attribute>> {
        query_one_by_field(conn, Self::TABLE, Self::COLUMNS, "name", name, Self::row_to_model)
    }

    fn row_to_model(row: &Row) -> rusqlite::Result<Attribute> {
        Ok(Attribute {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            base_value: row.get(3)?,
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

    // Local helper with custom signature for attribute-specific tests
    fn insert_test_attribute(
        conn: &Connection,
        id: i64,
        name: &str,
        description: &str,
        base_value: i64,
    ) {
        conn.execute(
            "INSERT INTO attributes (id, name, description, base_value, created_at, updated_at)
             VALUES (?, ?, ?, ?, '2024-01-01', '2024-01-01')",
            params![id, name, description, base_value],
        )
        .unwrap();
    }

    #[test]
    fn test_get_by_id_found() {
        let conn = setup_test_db();
        insert_test_attribute(&conn, 1, "Agility", "Measures nimbleness and dexterity", 4);

        let result = AttributeRepository::get_by_id(&conn, 1).unwrap();

        assert!(result.is_some());
        let attribute = result.unwrap();
        assert_eq!(attribute.id, 1);
        assert_eq!(attribute.name, "Agility");
        assert_eq!(attribute.description, "Measures nimbleness and dexterity");
        assert_eq!(attribute.base_value, 4);
        assert_eq!(attribute.created_at, "2024-01-01");
        assert_eq!(attribute.updated_at, "2024-01-01");
    }

    #[test]
    fn test_get_by_id_not_found() {
        let conn = setup_test_db();

        let result = AttributeRepository::get_by_id(&conn, 999).unwrap();

        assert!(result.is_none());
    }

    #[test]
    fn test_get_all_empty() {
        let conn = setup_test_db();

        let attributes = AttributeRepository::get_all(&conn).unwrap();

        assert_eq!(attributes.len(), 0);
    }

    #[test]
    fn test_get_all_multiple() {
        let conn = setup_test_db();
        insert_test_attribute(&conn, 1, "Agility", "Nimbleness and dexterity", 4);
        insert_test_attribute(&conn, 2, "Smarts", "Mental acuity and intelligence", 4);
        insert_test_attribute(&conn, 3, "Spirit", "Inner wisdom and willpower", 4);

        let attributes = AttributeRepository::get_all(&conn).unwrap();

        assert_eq!(attributes.len(), 3);
        // Verify alphabetical ordering by name
        assert_eq!(attributes[0].name, "Agility");
        assert_eq!(attributes[1].name, "Smarts");
        assert_eq!(attributes[2].name, "Spirit");
    }

    #[test]
    fn test_get_all_ordered_by_name() {
        let conn = setup_test_db();
        // Insert in non-alphabetical order
        insert_test_attribute(&conn, 3, "Vigor", "Toughness and endurance", 4);
        insert_test_attribute(&conn, 1, "Agility", "Nimbleness and dexterity", 4);
        insert_test_attribute(&conn, 2, "Strength", "Physical power", 4);

        let attributes = AttributeRepository::get_all(&conn).unwrap();

        assert_eq!(attributes.len(), 3);
        // Should be ordered alphabetically by name
        assert_eq!(attributes[0].name, "Agility");
        assert_eq!(attributes[1].name, "Strength");
        assert_eq!(attributes[2].name, "Vigor");
    }

    #[test]
    fn test_row_to_model_with_different_base_values() {
        let conn = setup_test_db();
        insert_test_attribute(&conn, 1, "Agility", "Test attribute", 6);

        let result = AttributeRepository::get_by_id(&conn, 1).unwrap();

        assert!(result.is_some());
        let attribute = result.unwrap();
        assert_eq!(attribute.base_value, 6);
    }

    #[test]
    fn test_get_by_name_found() {
        let conn = setup_test_db();
        insert_test_attribute(&conn, 1, "Agility", "Nimbleness", 4);

        let result = AttributeRepository::get_by_name(&conn, "Agility").unwrap();

        assert!(result.is_some());
        let attribute = result.unwrap();
        assert_eq!(attribute.id, 1);
        assert_eq!(attribute.name, "Agility");
    }

    #[test]
    fn test_get_by_name_case_insensitive() {
        let conn = setup_test_db();
        insert_test_attribute(&conn, 1, "Agility", "Nimbleness", 4);

        // Test various case combinations
        assert!(
            AttributeRepository::get_by_name(&conn, "agility")
                .unwrap()
                .is_some()
        );
        assert!(
            AttributeRepository::get_by_name(&conn, "AGILITY")
                .unwrap()
                .is_some()
        );
        assert!(
            AttributeRepository::get_by_name(&conn, "aGiLiTy")
                .unwrap()
                .is_some()
        );
    }

    #[test]
    fn test_get_by_name_not_found() {
        let conn = setup_test_db();

        let result = AttributeRepository::get_by_name(&conn, "NonExistent").unwrap();

        assert!(result.is_none());
    }
}
