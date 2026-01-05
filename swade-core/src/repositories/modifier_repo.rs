use crate::error::Result;
use crate::models::Modifier;
use super::base_repository::{query_one_by_id, query_all_ordered};
use rusqlite::{Connection, Row};

pub struct ModifierRepository;

impl ModifierRepository {
    const TABLE: &'static str = "modifiers";
    const COLUMNS: &'static str = "id, target_type, target_identifier, value_type, value,
                                   description, created_at, updated_at";

    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<Modifier>> {
        query_one_by_id(conn, Self::TABLE, Self::COLUMNS, id, Self::row_to_model)
    }

    pub fn get_all(conn: &Connection) -> Result<Vec<Modifier>> {
        query_all_ordered(conn, Self::TABLE, Self::COLUMNS, "id", Self::row_to_model)
    }

    fn row_to_model(row: &Row) -> rusqlite::Result<Modifier> {
        Ok(Modifier {
            id: row.get(0)?,
            target_type: row.get(1)?,
            target_identifier: row.get(2)?,
            value_type: row.get(3)?,
            value: row.get(4)?,
            description: row.get(5)?,
            created_at: row.get(6)?,
            updated_at: row.get(7)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::setup_test_db;

    // Local helper with custom signature for modifier-specific tests
    fn insert_test_modifier(conn: &Connection) {
        conn.execute(
            "INSERT INTO modifiers (id, target_type, target_identifier, value_type,
                                   value, description, created_at, updated_at)
             VALUES (1, 'attribute', 'Vigor', 'die_increment', 1,
                     'Vigor starts at d6', '2024-01-01', '2024-01-01')",
            [],
        )
        .unwrap();
    }

    fn insert_test_modifier_with_nulls(conn: &Connection) {
        conn.execute(
            "INSERT INTO modifiers (id, target_type, target_identifier, value_type,
                                   value, description, created_at, updated_at)
             VALUES (2, NULL, NULL, 'description', NULL,
                     'Low Light Vision', '2024-01-01', '2024-01-01')",
            [],
        )
        .unwrap();
    }

    #[test]
    fn test_get_by_id_found() {
        let conn = setup_test_db();
        insert_test_modifier(&conn);

        let result = ModifierRepository::get_by_id(&conn, 1).unwrap();

        assert!(result.is_some());
        let modifier = result.unwrap();
        assert_eq!(modifier.id, 1);
        assert_eq!(modifier.target_type, Some("attribute".to_string()));
        assert_eq!(modifier.target_identifier, Some("Vigor".to_string()));
        assert_eq!(modifier.value_type, "die_increment");
        assert_eq!(modifier.value, Some(1));
        assert_eq!(modifier.description, "Vigor starts at d6");
    }

    #[test]
    fn test_get_by_id_not_found() {
        let conn = setup_test_db();

        let result = ModifierRepository::get_by_id(&conn, 999).unwrap();

        assert!(result.is_none());
    }

    #[test]
    fn test_get_by_id_with_null_fields() {
        let conn = setup_test_db();
        insert_test_modifier_with_nulls(&conn);

        let result = ModifierRepository::get_by_id(&conn, 2).unwrap();

        assert!(result.is_some());
        let modifier = result.unwrap();
        assert_eq!(modifier.id, 2);
        assert_eq!(modifier.target_type, None);
        assert_eq!(modifier.target_identifier, None);
        assert_eq!(modifier.value_type, "description");
        assert_eq!(modifier.value, None);
        assert_eq!(modifier.description, "Low Light Vision");
    }

    #[test]
    fn test_get_all() {
        let conn = setup_test_db();
        insert_test_modifier(&conn);
        insert_test_modifier_with_nulls(&conn);

        let modifiers = ModifierRepository::get_all(&conn).unwrap();

        assert_eq!(modifiers.len(), 2);
        assert_eq!(modifiers[0].id, 1);
        assert_eq!(modifiers[1].id, 2);
    }

    #[test]
    fn test_get_all_empty() {
        let conn = setup_test_db();

        let modifiers = ModifierRepository::get_all(&conn).unwrap();

        assert_eq!(modifiers.len(), 0);
    }
}
