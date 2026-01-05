use crate::error::Result;
use crate::models::GearRequirement;
use super::base_repository::{query_one_by_id, query_all_ordered, query_by_fk_id};
use rusqlite::{Connection, Row};

pub struct GearRequirementRepository;

impl GearRequirementRepository {
    const TABLE: &'static str = "gear_requirements";
    const COLUMNS: &'static str = "id, gear_id, requirement_expression_id, created_at, updated_at";

    /// Get a single gear requirement by ID
    pub fn get_by_id(conn: &Connection, id: i64) -> Result<Option<GearRequirement>> {
        query_one_by_id(conn, Self::TABLE, Self::COLUMNS, id, Self::row_to_model)
    }

    /// Get all gear requirements
    pub fn get_all(conn: &Connection) -> Result<Vec<GearRequirement>> {
        query_all_ordered(conn, Self::TABLE, Self::COLUMNS, "gear_id, requirement_expression_id", Self::row_to_model)
    }

    /// Get all gear requirements for a specific gear item
    pub fn get_by_gear_id(conn: &Connection, gear_id: i64) -> Result<Vec<GearRequirement>> {
        query_by_fk_id(conn, Self::TABLE, Self::COLUMNS, "gear_id", gear_id, "id", Self::row_to_model)
    }

    /// Convert a database row to a GearRequirement model
    fn row_to_model(row: &Row) -> rusqlite::Result<GearRequirement> {
        Ok(GearRequirement {
            id: row.get(0)?,
            gear_id: row.get(1)?,
            requirement_expression_id: row.get(2)?,
            created_at: row.get(3)?,
            updated_at: row.get(4)?,
        })
    }
}
