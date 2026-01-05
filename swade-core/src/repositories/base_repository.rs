//! Base repository helper functions to reduce boilerplate across repositories.
//!
//! These functions encapsulate the common patterns for querying data:
//! - `query_one_by_id` - Fetch a single record by ID
//! - `query_all_ordered` - Fetch all records with ordering
//! - `query_one_by_field` - Fetch a single record by a string field (case-insensitive)
//!
//! Each repository still implements its own `row_to_model` function since that's
//! specific to each model's fields.

use crate::error::Result;
use rusqlite::{params, Connection, Row};

/// Query a single record by ID from any table.
///
/// # Arguments
/// * `conn` - Database connection
/// * `table` - Table name
/// * `columns` - Comma-separated column names for SELECT
/// * `id` - The ID to query
/// * `row_mapper` - Function to convert a row to the model type
///
/// # Example
/// ```ignore
/// query_one_by_id(conn, "hindrances", "id, name, severity", 1, Self::row_to_model)
/// ```
pub fn query_one_by_id<T, F>(
    conn: &Connection,
    table: &str,
    columns: &str,
    id: i64,
    row_mapper: F,
) -> Result<Option<T>>
where
    F: FnOnce(&Row) -> rusqlite::Result<T>,
{
    let sql = format!("SELECT {} FROM {} WHERE id = ?", columns, table);
    let mut stmt = conn.prepare(&sql)?;
    let mut rows = stmt.query(params![id])?;

    match rows.next()? {
        Some(row) => Ok(Some(row_mapper(row)?)),
        None => Ok(None),
    }
}

/// Query all records from a table with ordering.
///
/// # Arguments
/// * `conn` - Database connection
/// * `table` - Table name
/// * `columns` - Comma-separated column names for SELECT
/// * `order_by` - Column(s) to order by
/// * `row_mapper` - Function to convert a row to the model type
///
/// # Example
/// ```ignore
/// query_all_ordered(conn, "hindrances", "id, name, severity", "name", Self::row_to_model)
/// ```
pub fn query_all_ordered<T, F>(
    conn: &Connection,
    table: &str,
    columns: &str,
    order_by: &str,
    row_mapper: F,
) -> Result<Vec<T>>
where
    F: Fn(&Row) -> rusqlite::Result<T>,
{
    let sql = format!("SELECT {} FROM {} ORDER BY {}", columns, table, order_by);
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map([], row_mapper)?;

    Ok(rows.filter_map(|r| r.ok()).collect())
}

/// Query a single record by a string field (case-insensitive).
///
/// # Arguments
/// * `conn` - Database connection
/// * `table` - Table name
/// * `columns` - Comma-separated column names for SELECT
/// * `field` - The field name to query by
/// * `value` - The value to match (case-insensitive)
/// * `row_mapper` - Function to convert a row to the model type
///
/// # Example
/// ```ignore
/// query_one_by_field(conn, "skills", "id, name", "name", "Fighting", Self::row_to_model)
/// ```
pub fn query_one_by_field<T, F>(
    conn: &Connection,
    table: &str,
    columns: &str,
    field: &str,
    value: &str,
    row_mapper: F,
) -> Result<Option<T>>
where
    F: FnOnce(&Row) -> rusqlite::Result<T>,
{
    let sql = format!(
        "SELECT {} FROM {} WHERE LOWER({}) = LOWER(?)",
        columns, table, field
    );
    let mut stmt = conn.prepare(&sql)?;
    let mut rows = stmt.query(params![value])?;

    match rows.next()? {
        Some(row) => Ok(Some(row_mapper(row)?)),
        None => Ok(None),
    }
}

/// Query records with a WHERE clause.
///
/// # Arguments
/// * `conn` - Database connection
/// * `table` - Table name
/// * `columns` - Comma-separated column names for SELECT
/// * `where_clause` - WHERE clause (without "WHERE" keyword)
/// * `order_by` - Column(s) to order by
/// * `row_mapper` - Function to convert a row to the model type
///
/// # Example
/// ```ignore
/// query_where(conn, "skills", "id, name", "is_core_skill = 1", "name", Self::row_to_model)
/// ```
pub fn query_where<T, F>(
    conn: &Connection,
    table: &str,
    columns: &str,
    where_clause: &str,
    order_by: &str,
    row_mapper: F,
) -> Result<Vec<T>>
where
    F: Fn(&Row) -> rusqlite::Result<T>,
{
    let sql = format!(
        "SELECT {} FROM {} WHERE {} ORDER BY {}",
        columns, table, where_clause, order_by
    );
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map([], row_mapper)?;

    Ok(rows.filter_map(|r| r.ok()).collect())
}

/// Query multiple records by a foreign key ID.
///
/// # Arguments
/// * `conn` - Database connection
/// * `table` - Table name
/// * `columns` - Comma-separated column names for SELECT
/// * `fk_column` - The foreign key column name to filter by
/// * `fk_id` - The foreign key ID value
/// * `order_by` - Column(s) to order by
/// * `row_mapper` - Function to convert a row to the model type
///
/// # Example
/// ```ignore
/// query_by_fk_id(conn, "edge_modifiers", "id, edge_id", "edge_id", 1, "id", Self::row_to_model)
/// ```
pub fn query_by_fk_id<T, F>(
    conn: &Connection,
    table: &str,
    columns: &str,
    fk_column: &str,
    fk_id: i64,
    order_by: &str,
    row_mapper: F,
) -> Result<Vec<T>>
where
    F: Fn(&Row) -> rusqlite::Result<T>,
{
    let sql = format!(
        "SELECT {} FROM {} WHERE {} = ? ORDER BY {}",
        columns, table, fk_column, order_by
    );
    let mut stmt = conn.prepare(&sql)?;
    let rows = stmt.query_map(params![fk_id], row_mapper)?;

    Ok(rows.filter_map(|r| r.ok()).collect())
}
