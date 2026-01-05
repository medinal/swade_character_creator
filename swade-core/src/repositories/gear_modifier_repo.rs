use crate::error::Result;
use crate::models::GearModifier;
use super::base_repository::{query_one_by_id, query_all_ordered, query_by_fk_id};
use rusqlite::{Connection, Row};

pub struct GearModifierRepository;

impl GearModifierRepository {
    const TABLE: &'static str = "gear_modifiers";
    const COLUMNS: &'static str = "id, gear_id, modifier_id, created_at, updated_at";

    /// Get a single gear modifier by ID
    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<GearModifier>> {
        query_one_by_id(conn, Self::TABLE, Self::COLUMNS, id, Self::row_to_model)
    }

    /// Get all gear modifiers
    pub fn get_all(conn: &Connection) -> Result<Vec<GearModifier>> {
        query_all_ordered(conn, Self::TABLE, Self::COLUMNS, "gear_id, modifier_id", Self::row_to_model)
    }

    /// Get all gear modifiers for a specific gear item
    pub fn get_by_gear_id(conn: &Connection, gear_id: i64) -> Result<Vec<GearModifier>> {
        query_by_fk_id(conn, Self::TABLE, Self::COLUMNS, "gear_id", gear_id, "id", Self::row_to_model)
    }

    /// Convert a database row to a GearModifier model
    fn row_to_model(row: &Row) -> rusqlite::Result<GearModifier> {
        Ok(GearModifier {
            id: row.get(0)?,
            gear_id: row.get(1)?,
            modifier_id: row.get(2)?,
            created_at: row.get(3)?,
            updated_at: row.get(4)?,
        })
    }
}
