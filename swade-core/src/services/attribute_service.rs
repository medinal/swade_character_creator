use rusqlite::Connection;

use crate::error::Result;
use crate::repositories::AttributeRepository;
use crate::views::AttributeView;

pub struct AttributeService;

impl AttributeService {
    pub fn get_all(conn: &Connection) -> Result<Vec<AttributeView>> {
        let attributes = AttributeRepository::get_all(conn)?;
        Ok(attributes.into_iter().map(AttributeView::new).collect())
    }

    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<AttributeView>> {
        let attribute = AttributeRepository::get_by_id(conn, id)?;
        Ok(attribute.map(AttributeView::new))
    }

    /// Get an attribute by name (case-insensitive)
    pub fn get_by_name(conn: &Connection, name: &str) -> Result<Option<AttributeView>> {
        let attribute = AttributeRepository::get_by_name(conn, name)?;
        Ok(attribute.map(AttributeView::new))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::setup_test_db;
    use crate::views::Die;
    use rusqlite::params;

    fn insert_test_attribute(conn: &Connection, id: i64, name: &str, base_value: i64) {
        conn.execute(
            "INSERT INTO attributes (id, name, description, base_value, created_at, updated_at)
             VALUES (?, ?, 'Test description', ?, '2024-01-01', '2024-01-01')",
            params![id, name, base_value],
        )
        .unwrap();
    }

    #[test]
    fn get_all_returns_attribute_views() {
        let conn = setup_test_db();
        insert_test_attribute(&conn, 1, "Agility", 4);
        insert_test_attribute(&conn, 2, "Smarts", 4);

        let views = AttributeService::get_all(&conn).unwrap();

        assert_eq!(views.len(), 2);
        assert_eq!(views[0].name, "Agility");
        assert_eq!(views[0].base_die, Die::d4());
        assert_eq!(views[1].name, "Smarts");
    }

    #[test]
    fn get_all_returns_empty_when_no_attributes() {
        let conn = setup_test_db();

        let views = AttributeService::get_all(&conn).unwrap();

        assert_eq!(views.len(), 0);
    }

    #[test]
    fn get_by_id_returns_attribute_view() {
        let conn = setup_test_db();
        insert_test_attribute(&conn, 1, "Vigor", 4);

        let view = AttributeService::get_by_id(&conn, 1).unwrap();

        assert!(view.is_some());
        let view = view.unwrap();
        assert_eq!(view.id, 1);
        assert_eq!(view.name, "Vigor");
        assert_eq!(view.base_die, Die::d4());
    }

    #[test]
    fn get_by_id_returns_none_when_not_found() {
        let conn = setup_test_db();

        let view = AttributeService::get_by_id(&conn, 999).unwrap();

        assert!(view.is_none());
    }

    #[test]
    fn get_by_name_returns_attribute_view() {
        let conn = setup_test_db();
        insert_test_attribute(&conn, 1, "Vigor", 4);

        let view = AttributeService::get_by_name(&conn, "Vigor").unwrap();

        assert!(view.is_some());
        let view = view.unwrap();
        assert_eq!(view.name, "Vigor");
        assert_eq!(view.base_die, Die::d4());
    }

    #[test]
    fn get_by_name_is_case_insensitive() {
        let conn = setup_test_db();
        insert_test_attribute(&conn, 1, "Vigor", 4);

        assert!(
            AttributeService::get_by_name(&conn, "vigor")
                .unwrap()
                .is_some()
        );
        assert!(
            AttributeService::get_by_name(&conn, "VIGOR")
                .unwrap()
                .is_some()
        );
        assert!(
            AttributeService::get_by_name(&conn, "ViGoR")
                .unwrap()
                .is_some()
        );
    }

    #[test]
    fn get_by_name_returns_none_when_not_found() {
        let conn = setup_test_db();

        let view = AttributeService::get_by_name(&conn, "NonExistent").unwrap();

        assert!(view.is_none());
    }
}
