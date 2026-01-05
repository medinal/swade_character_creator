use crate::error::Result;
use crate::models::Requirement;
use super::base_repository::{query_one_by_id, query_all_ordered};
use rusqlite::{Connection, Row};

pub struct RequirementRepository;

impl RequirementRepository {
    const TABLE: &'static str = "requirements";
    const COLUMNS: &'static str = "id, requirement_type, target_id, value, description,
                                   created_at, updated_at";

    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<Requirement>> {
        query_one_by_id(conn, Self::TABLE, Self::COLUMNS, id, Self::row_to_model)
    }

    pub fn get_all(conn: &Connection) -> Result<Vec<Requirement>> {
        query_all_ordered(conn, Self::TABLE, Self::COLUMNS, "id", Self::row_to_model)
    }

    fn row_to_model(row: &Row) -> rusqlite::Result<Requirement> {
        Ok(Requirement {
            id: row.get(0)?,
            requirement_type: row.get(1)?,
            target_id: row.get(2)?,
            value: row.get(3)?,
            description: row.get(4)?,
            created_at: row.get(5)?,
            updated_at: row.get(6)?,
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

    fn insert_test_requirement(
        conn: &Connection,
        id: i64,
        requirement_type: &str,
        target_id: Option<i64>,
        value: Option<i64>,
        description: &str,
    ) {
        conn.execute(
            "INSERT INTO requirements (id, requirement_type, target_id, value, description,
                                      created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, '2024-01-01', '2024-01-01')",
            params![id, requirement_type, target_id, value, description],
        )
        .unwrap();
    }

    #[test]
    fn test_get_by_id_found() {
        let conn = setup_test_db();
        insert_test_requirement(&conn, 1, "rank", Some(1), Some(1), "Novice");

        let result = RequirementRepository::get_by_id(&conn, 1).unwrap();

        assert!(result.is_some());
        let requirement = result.unwrap();
        assert_eq!(requirement.id, 1);
        assert_eq!(requirement.requirement_type, "rank");
        assert_eq!(requirement.target_id, Some(1));
        assert_eq!(requirement.value, Some(1));
        assert_eq!(requirement.description, "Novice");
    }

    #[test]
    fn test_get_by_id_not_found() {
        let conn = setup_test_db();

        let result = RequirementRepository::get_by_id(&conn, 999).unwrap();

        assert!(result.is_none());
    }

    #[test]
    fn test_get_by_id_with_null_fields() {
        let conn = setup_test_db();
        insert_test_requirement(&conn, 1, "description", None, None, "Must be wild card");

        let result = RequirementRepository::get_by_id(&conn, 1).unwrap();

        assert!(result.is_some());
        let requirement = result.unwrap();
        assert_eq!(requirement.requirement_type, "description");
        assert_eq!(requirement.target_id, None);
        assert_eq!(requirement.value, None);
        assert_eq!(requirement.description, "Must be wild card");
    }

    #[test]
    fn test_get_by_id_attribute_requirement() {
        let conn = setup_test_db();
        insert_test_requirement(&conn, 1, "attribute", Some(1), Some(8), "Agility d8+");

        let result = RequirementRepository::get_by_id(&conn, 1).unwrap();

        assert!(result.is_some());
        let requirement = result.unwrap();
        assert_eq!(requirement.requirement_type, "attribute");
        assert_eq!(requirement.target_id, Some(1));
        assert_eq!(requirement.value, Some(8));
        assert_eq!(requirement.description, "Agility d8+");
    }

    #[test]
    fn test_get_by_id_skill_requirement() {
        let conn = setup_test_db();
        insert_test_requirement(&conn, 1, "skill", Some(5), Some(6), "Fighting d6+");

        let result = RequirementRepository::get_by_id(&conn, 1).unwrap();

        assert!(result.is_some());
        let requirement = result.unwrap();
        assert_eq!(requirement.requirement_type, "skill");
        assert_eq!(requirement.target_id, Some(5));
        assert_eq!(requirement.value, Some(6));
        assert_eq!(requirement.description, "Fighting d6+");
    }

    #[test]
    fn test_get_by_id_edge_requirement() {
        let conn = setup_test_db();
        insert_test_requirement(&conn, 1, "edge", Some(10), None, "Arcane Background");

        let result = RequirementRepository::get_by_id(&conn, 1).unwrap();

        assert!(result.is_some());
        let requirement = result.unwrap();
        assert_eq!(requirement.requirement_type, "edge");
        assert_eq!(requirement.target_id, Some(10));
        assert_eq!(requirement.value, None);
        assert_eq!(requirement.description, "Arcane Background");
    }

    #[test]
    fn test_get_by_id_wild_card_requirement() {
        let conn = setup_test_db();
        insert_test_requirement(&conn, 1, "wild_card", None, None, "Wild Card only");

        let result = RequirementRepository::get_by_id(&conn, 1).unwrap();

        assert!(result.is_some());
        let requirement = result.unwrap();
        assert_eq!(requirement.requirement_type, "wild_card");
        assert_eq!(requirement.target_id, None);
        assert_eq!(requirement.value, None);
        assert_eq!(requirement.description, "Wild Card only");
    }

    #[test]
    fn test_get_all() {
        let conn = setup_test_db();
        insert_test_requirement(&conn, 1, "rank", Some(1), Some(1), "Novice");
        insert_test_requirement(&conn, 2, "attribute", Some(1), Some(8), "Agility d8+");
        insert_test_requirement(&conn, 3, "description", None, None, "Must be wild card");

        let requirements = RequirementRepository::get_all(&conn).unwrap();

        assert_eq!(requirements.len(), 3);
        // Verify ordering by id
        assert_eq!(requirements[0].id, 1);
        assert_eq!(requirements[0].requirement_type, "rank");
        assert_eq!(requirements[1].id, 2);
        assert_eq!(requirements[1].requirement_type, "attribute");
        assert_eq!(requirements[2].id, 3);
        assert_eq!(requirements[2].requirement_type, "description");
    }

    #[test]
    fn test_get_all_empty() {
        let conn = setup_test_db();

        let requirements = RequirementRepository::get_all(&conn).unwrap();

        assert_eq!(requirements.len(), 0);
    }

    #[test]
    fn test_get_all_multiple_requirement_types() {
        let conn = setup_test_db();
        insert_test_requirement(&conn, 1, "rank", Some(1), Some(1), "Novice");
        insert_test_requirement(&conn, 2, "skill", Some(5), Some(6), "Fighting d6+");
        insert_test_requirement(&conn, 3, "edge", Some(10), None, "Arcane Background");
        insert_test_requirement(&conn, 4, "wild_card", None, None, "Wild Card only");
        insert_test_requirement(&conn, 5, "arcane_background", Some(2), None, "Magic");

        let requirements = RequirementRepository::get_all(&conn).unwrap();

        assert_eq!(requirements.len(), 5);

        // Verify all types are present
        let types: Vec<String> = requirements
            .iter()
            .map(|r| r.requirement_type.clone())
            .collect();
        assert!(types.contains(&"rank".to_string()));
        assert!(types.contains(&"skill".to_string()));
        assert!(types.contains(&"edge".to_string()));
        assert!(types.contains(&"wild_card".to_string()));
        assert!(types.contains(&"arcane_background".to_string()));
    }
}
