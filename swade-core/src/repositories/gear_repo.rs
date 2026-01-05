use crate::error::Result;
use crate::models::Gear;
use super::base_repository::{query_one_by_id, query_all_ordered, query_by_fk_id};
use rusqlite::{Connection, Row};

pub struct GearRepository;

impl GearRepository {
    const TABLE: &'static str = "gear";
    const COLUMNS: &'static str = "id, name, category_id, era, cost, weight, source, notes, description, created_at, updated_at";

    /// Get a single gear item by ID
    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<Gear>> {
        query_one_by_id(conn, Self::TABLE, Self::COLUMNS, id, Self::row_to_model)
    }

    /// Get all gear items ordered by name
    pub fn get_all(conn: &Connection) -> Result<Vec<Gear>> {
        query_all_ordered(conn, Self::TABLE, Self::COLUMNS, "name", Self::row_to_model)
    }

    /// Get all gear items for a specific category
    pub fn get_by_category_id(conn: &Connection, category_id: i64) -> Result<Vec<Gear>> {
        query_by_fk_id(conn, Self::TABLE, Self::COLUMNS, "category_id", category_id, "name", Self::row_to_model)
    }

    /// Convert a database row to a Gear model
    fn row_to_model(row: &Row) -> rusqlite::Result<Gear> {
        Ok(Gear {
            id: row.get(0)?,
            name: row.get(1)?,
            category_id: row.get(2)?,
            era: row.get(3)?,
            cost: row.get(4)?,
            weight: row.get(5)?,
            source: row.get(6)?,
            notes: row.get(7)?,
            description: row.get(8)?,
            created_at: row.get(9)?,
            updated_at: row.get(10)?,
        })
    }
}
