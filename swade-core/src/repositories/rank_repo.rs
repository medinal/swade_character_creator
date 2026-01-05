use crate::error::Result;
use crate::models::Rank;
use super::base_repository::{query_one_by_id, query_all_ordered};
use rusqlite::{Connection, Row};

pub struct RankRepository;

impl RankRepository {
    const TABLE: &'static str = "ranks";
    const COLUMNS: &'static str = "id, name, min_advances, max_advances, description,
                                   created_at, updated_at";

    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<Rank>> {
        query_one_by_id(conn, Self::TABLE, Self::COLUMNS, id, Self::row_to_model)
    }

    pub fn get_all(conn: &Connection) -> Result<Vec<Rank>> {
        query_all_ordered(conn, Self::TABLE, Self::COLUMNS, "min_advances", Self::row_to_model)
    }

    fn row_to_model(row: &Row) -> rusqlite::Result<Rank> {
        Ok(Rank {
            id: row.get(0)?,
            name: row.get(1)?,
            min_advances: row.get(2)?,
            max_advances: row.get(3)?,
            description: row.get(4)?,
            created_at: row.get(5)?,
            updated_at: row.get(6)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_utils::setup_test_db;
    use rusqlite::params;

    // Local helper with custom signature for rank-specific tests
    fn insert_test_rank(
        conn: &Connection,
        id: i64,
        name: &str,
        min_advances: i64,
        max_advances: Option<i64>,
    ) {
        conn.execute(
            "INSERT INTO ranks (id, name, min_advances, max_advances, description,
                               created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, '2024-01-01', '2024-01-01')",
            params![
                id,
                name,
                min_advances,
                max_advances,
                format!("{} rank description", name)
            ],
        )
        .unwrap();
    }

    #[test]
    fn test_get_by_id_found() {
        let conn = setup_test_db();
        insert_test_rank(&conn, 1, "Novice", 0, Some(3));

        let result = RankRepository::get_by_id(&conn, 1).unwrap();

        assert!(result.is_some());
        let rank = result.unwrap();
        assert_eq!(rank.name, "Novice");
        assert_eq!(rank.min_advances, 0);
        assert_eq!(rank.max_advances, Some(3));
        assert_eq!(rank.description, "Novice rank description");
    }

    #[test]
    fn test_get_by_id_not_found() {
        let conn = setup_test_db();

        let result = RankRepository::get_by_id(&conn, 999).unwrap();

        assert!(result.is_none());
    }

    #[test]
    fn test_get_by_id_with_null_max_advances() {
        let conn = setup_test_db();
        insert_test_rank(&conn, 1, "Legendary", 16, None);

        let result = RankRepository::get_by_id(&conn, 1).unwrap();

        assert!(result.is_some());
        let rank = result.unwrap();
        assert_eq!(rank.name, "Legendary");
        assert_eq!(rank.min_advances, 16);
        assert_eq!(rank.max_advances, None);
    }

    #[test]
    fn test_get_all() {
        let conn = setup_test_db();
        insert_test_rank(&conn, 1, "Novice", 0, Some(3));
        insert_test_rank(&conn, 2, "Seasoned", 4, Some(7));
        insert_test_rank(&conn, 3, "Legendary", 16, None);

        let ranks = RankRepository::get_all(&conn).unwrap();

        assert_eq!(ranks.len(), 3);
        // Verify ordering by min_advances
        assert_eq!(ranks[0].name, "Novice");
        assert_eq!(ranks[1].name, "Seasoned");
        assert_eq!(ranks[2].name, "Legendary");
    }

    #[test]
    fn test_get_all_empty() {
        let conn = setup_test_db();

        let ranks = RankRepository::get_all(&conn).unwrap();

        assert_eq!(ranks.len(), 0);
    }
}
