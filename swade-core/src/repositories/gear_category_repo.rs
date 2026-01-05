use crate::error::Result;
use crate::models::GearCategory;
use super::base_repository::{query_one_by_id, query_all_ordered, query_one_by_field};
use rusqlite::{Connection, Row};

pub struct GearCategoryRepository;

impl GearCategoryRepository {
    const TABLE: &'static str = "gear_categories";
    const COLUMNS: &'static str = "id, name, description, created_at, updated_at";

    /// Get a single gear category by ID
    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<GearCategory>> {
        query_one_by_id(conn, Self::TABLE, Self::COLUMNS, id, Self::row_to_model)
    }

    /// Get all gear categories ordered by name
    pub fn get_all(conn: &Connection) -> Result<Vec<GearCategory>> {
        query_all_ordered(conn, Self::TABLE, Self::COLUMNS, "name", Self::row_to_model)
    }

    /// Get a gear category by name (case-insensitive)
    pub fn get_by_name(conn: &Connection, name: &str) -> Result<Option<GearCategory>> {
        query_one_by_field(conn, Self::TABLE, Self::COLUMNS, "name", name, Self::row_to_model)
    }

    /// Convert a database row to a GearCategory model
    fn row_to_model(row: &Row) -> rusqlite::Result<GearCategory> {
        Ok(GearCategory {
            id: row.get(0)?,
            name: row.get(1)?,
            description: row.get(2)?,
            created_at: row.get(3)?,
            updated_at: row.get(4)?,
        })
    }
}
