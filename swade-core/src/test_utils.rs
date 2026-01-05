//! Shared test utilities for repository tests.
//!
//! This module provides common setup functions and test data helpers
//! to reduce duplication across repository test modules.

use rusqlite::{Connection, params};

/// Default timestamp used in test data
pub const TEST_TIMESTAMP: &str = "2024-01-01 00:00:00";

/// Creates an in-memory SQLite database with the schema loaded.
///
/// This is the standard setup for most repository tests.
pub fn setup_test_db() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    let schema = include_str!("../../db/schema.sql");
    conn.execute_batch(schema).unwrap();
    conn
}

/// Creates an in-memory database with foreign key constraints disabled.
///
/// Useful for tests that need to insert data without satisfying all FK dependencies.
pub fn setup_test_db_no_fk() -> Connection {
    let conn = setup_test_db();
    conn.execute_batch("PRAGMA foreign_keys = OFF;").unwrap();
    conn
}

/// Creates an in-memory database with foreign key constraints enabled.
///
/// Use this when tests need to verify FK constraint behavior.
pub fn setup_test_db_with_fk() -> Connection {
    let conn = Connection::open_in_memory().unwrap();
    conn.execute("PRAGMA foreign_keys = ON", []).unwrap();
    let schema = include_str!("../../db/schema.sql");
    conn.execute_batch(schema).unwrap();
    conn
}

/// Inserts a test rank into the database.
pub fn insert_test_rank(conn: &Connection, id: i64, name: &str) {
    conn.execute(
        "INSERT INTO ranks (id, name, min_advances, max_advances, description, created_at, updated_at)
         VALUES (?, ?, 0, 3, 'Test rank', ?, ?)",
        params![id, name, TEST_TIMESTAMP, TEST_TIMESTAMP],
    )
    .unwrap();
}

/// Inserts a test attribute into the database.
pub fn insert_test_attribute(conn: &Connection, id: i64, name: &str) {
    conn.execute(
        "INSERT INTO attributes (id, name, description, created_at, updated_at)
         VALUES (?, ?, 'Test attribute', ?, ?)",
        params![id, name, TEST_TIMESTAMP, TEST_TIMESTAMP],
    )
    .unwrap();
}

/// Inserts a test skill into the database.
pub fn insert_test_skill(conn: &Connection, id: i64, name: &str, linked_attribute_id: i64) {
    conn.execute(
        "INSERT INTO skills (id, name, description, linked_attribute_id, is_core_skill, source, created_at, updated_at)
         VALUES (?, ?, 'Test skill', ?, 0, 'core', ?, ?)",
        params![id, name, linked_attribute_id, TEST_TIMESTAMP, TEST_TIMESTAMP],
    )
    .unwrap();
}

/// Inserts a test core skill into the database.
pub fn insert_test_core_skill(conn: &Connection, id: i64, name: &str, linked_attribute_id: i64) {
    conn.execute(
        "INSERT INTO skills (id, name, description, linked_attribute_id, is_core_skill, source, created_at, updated_at)
         VALUES (?, ?, 'Test core skill', ?, 1, 'core', ?, ?)",
        params![id, name, linked_attribute_id, TEST_TIMESTAMP, TEST_TIMESTAMP],
    )
    .unwrap();
}

/// Inserts a test edge into the database.
pub fn insert_test_edge(conn: &Connection, id: i64, name: &str, category: &str) {
    conn.execute(
        "INSERT INTO edges (id, name, background, source, description, can_take_multiple_times, created_at, updated_at)
         VALUES (?, ?, ?, 'core', 'Test description', 0, ?, ?)",
        params![id, name, category, TEST_TIMESTAMP, TEST_TIMESTAMP],
    )
    .unwrap();
}

/// Inserts a test edge that can be taken multiple times.
pub fn insert_test_edge_multi(conn: &Connection, id: i64, name: &str, category: &str) {
    conn.execute(
        "INSERT INTO edges (id, name, background, source, description, can_take_multiple_times, created_at, updated_at)
         VALUES (?, ?, ?, 'core', 'Test description', 1, ?, ?)",
        params![id, name, category, TEST_TIMESTAMP, TEST_TIMESTAMP],
    )
    .unwrap();
}

/// Inserts a test hindrance into the database.
pub fn insert_test_hindrance(conn: &Connection, id: i64, name: &str, severity: &str, point_value: i64) {
    conn.execute(
        "INSERT INTO hindrances (id, name, severity, point_value, source, description, created_at, updated_at)
         VALUES (?, ?, ?, ?, 'core', 'Test description', ?, ?)",
        params![id, name, severity, point_value, TEST_TIMESTAMP, TEST_TIMESTAMP],
    )
    .unwrap();
}

/// Inserts a test modifier into the database.
pub fn insert_test_modifier(conn: &Connection, id: i64, target_type: &str, value_type: &str, description: &str) {
    conn.execute(
        "INSERT INTO modifiers (id, target_type, target_identifier, value_type, value, description, created_at, updated_at)
         VALUES (?, ?, 'test_target', ?, 1, ?, ?, ?)",
        params![id, target_type, value_type, description, TEST_TIMESTAMP, TEST_TIMESTAMP],
    )
    .unwrap();
}

/// Inserts a test ancestry into the database.
pub fn insert_test_ancestry(conn: &Connection, id: i64, name: &str) {
    conn.execute(
        "INSERT INTO ancestries (id, name, source, description, created_at, updated_at)
         VALUES (?, ?, 'core', 'Test ancestry', ?, ?)",
        params![id, name, TEST_TIMESTAMP, TEST_TIMESTAMP],
    )
    .unwrap();
}

/// Inserts a test character into the database.
pub fn insert_test_character(conn: &Connection, id: i64, name: &str) {
    conn.execute(
        "INSERT INTO characters (id, name, is_wild_card, created_at, updated_at)
         VALUES (?, ?, 1, ?, ?)",
        params![id, name, TEST_TIMESTAMP, TEST_TIMESTAMP],
    )
    .unwrap();
}

/// Inserts a test character with ancestry.
/// Requires the specified ancestry to exist.
pub fn insert_test_character_with_ancestry(conn: &Connection, id: i64, name: &str, ancestry_id: i64) {
    conn.execute(
        "INSERT INTO characters (id, name, is_wild_card, ancestry_id, created_at, updated_at)
         VALUES (?, ?, 1, ?, ?, ?)",
        params![id, name, ancestry_id, TEST_TIMESTAMP, TEST_TIMESTAMP],
    )
    .unwrap();
}

/// Inserts a test requirement into the database.
pub fn insert_test_requirement(conn: &Connection, id: i64, requirement_type: &str, description: &str) {
    conn.execute(
        "INSERT INTO requirements (id, requirement_type, target_id, value, description, created_at, updated_at)
         VALUES (?, ?, NULL, NULL, ?, ?, ?)",
        params![id, requirement_type, description, TEST_TIMESTAMP, TEST_TIMESTAMP],
    )
    .unwrap();
}

/// Inserts a test requirement with target and value.
pub fn insert_test_requirement_with_target(
    conn: &Connection,
    id: i64,
    requirement_type: &str,
    target_id: i64,
    value: i64,
    description: &str,
) {
    conn.execute(
        "INSERT INTO requirements (id, requirement_type, target_id, value, description, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?, ?, ?)",
        params![id, requirement_type, target_id, value, description, TEST_TIMESTAMP, TEST_TIMESTAMP],
    )
    .unwrap();
}

/// Inserts a test requirement expression into the database.
pub fn insert_test_requirement_expression(conn: &Connection, id: i64, node_type: &str, requirement_id: Option<i64>) {
    conn.execute(
        "INSERT INTO requirement_expressions (id, parent_id, node_type, requirement_id, position, created_at, updated_at)
         VALUES (?, NULL, ?, ?, 0, ?, ?)",
        params![id, node_type, requirement_id, TEST_TIMESTAMP, TEST_TIMESTAMP],
    )
    .unwrap();
}

/// Inserts a test power into the database.
pub fn insert_test_power(conn: &Connection, id: i64, name: &str) {
    conn.execute(
        "INSERT INTO powers (id, name, power_points, range, duration, source, description, created_at, updated_at)
         VALUES (?, ?, 2, 'Smarts', '5', 'core', 'Test power', ?, ?)",
        params![id, name, TEST_TIMESTAMP, TEST_TIMESTAMP],
    )
    .unwrap();
}

/// Inserts a test arcane background into the database.
/// Requires a skill with the specified arcane_skill_id to exist.
pub fn insert_test_arcane_background(conn: &Connection, id: i64, name: &str, arcane_skill_id: i64) {
    conn.execute(
        "INSERT INTO arcane_backgrounds (id, name, arcane_skill_id, starting_powers, starting_power_points, source, description, created_at, updated_at)
         VALUES (?, ?, ?, 3, 10, 'core', 'Test arcane background', ?, ?)",
        params![id, name, arcane_skill_id, TEST_TIMESTAMP, TEST_TIMESTAMP],
    )
    .unwrap();
}

// Link table helpers

/// Inserts a test edge modifier link.
pub fn insert_test_edge_modifier(conn: &Connection, id: i64, edge_id: i64, modifier_id: i64) {
    conn.execute(
        "INSERT INTO edge_modifiers (id, edge_id, modifier_id, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?)",
        params![id, edge_id, modifier_id, TEST_TIMESTAMP, TEST_TIMESTAMP],
    )
    .unwrap();
}

/// Inserts a test hindrance modifier link.
pub fn insert_test_hindrance_modifier(conn: &Connection, id: i64, hindrance_id: i64, modifier_id: i64) {
    conn.execute(
        "INSERT INTO hindrance_modifiers (id, hindrance_id, modifier_id, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?)",
        params![id, hindrance_id, modifier_id, TEST_TIMESTAMP, TEST_TIMESTAMP],
    )
    .unwrap();
}

/// Inserts a test ancestry modifier link.
pub fn insert_test_ancestry_modifier(conn: &Connection, id: i64, ancestry_id: i64, modifier_id: i64) {
    conn.execute(
        "INSERT INTO ancestry_modifiers (id, ancestry_id, modifier_id, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?)",
        params![id, ancestry_id, modifier_id, TEST_TIMESTAMP, TEST_TIMESTAMP],
    )
    .unwrap();
}

/// Inserts a test power modifier link.
pub fn insert_test_power_modifier(conn: &Connection, id: i64, power_id: i64, modifier_id: i64) {
    conn.execute(
        "INSERT INTO power_modifiers (id, power_id, modifier_id, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?)",
        params![id, power_id, modifier_id, TEST_TIMESTAMP, TEST_TIMESTAMP],
    )
    .unwrap();
}

/// Inserts a test edge requirement link.
pub fn insert_test_edge_requirement(conn: &Connection, id: i64, edge_id: i64, requirement_expression_id: i64) {
    conn.execute(
        "INSERT INTO edge_requirements (id, edge_id, requirement_expression_id, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?)",
        params![id, edge_id, requirement_expression_id, TEST_TIMESTAMP, TEST_TIMESTAMP],
    )
    .unwrap();
}

// Character link table helpers

/// Inserts a test character attribute link.
pub fn insert_test_character_attribute(conn: &Connection, id: i64, character_id: i64, attribute_id: i64, die_type: i64) {
    conn.execute(
        "INSERT INTO character_attributes (id, character_id, attribute_id, die_type, source, created_at, updated_at)
         VALUES (?, ?, ?, ?, 'chosen', ?, ?)",
        params![id, character_id, attribute_id, die_type, TEST_TIMESTAMP, TEST_TIMESTAMP],
    )
    .unwrap();
}

/// Inserts a test character skill link.
pub fn insert_test_character_skill(conn: &Connection, id: i64, character_id: i64, skill_id: i64, die_type: i64) {
    conn.execute(
        "INSERT INTO character_skills (id, character_id, skill_id, die_type, source, created_at, updated_at)
         VALUES (?, ?, ?, ?, 'chosen', ?, ?)",
        params![id, character_id, skill_id, die_type, TEST_TIMESTAMP, TEST_TIMESTAMP],
    )
    .unwrap();
}

/// Inserts a test character edge link.
pub fn insert_test_character_edge(conn: &Connection, id: i64, character_id: i64, edge_id: i64) {
    conn.execute(
        "INSERT INTO character_edges (id, character_id, edge_id, source, created_at, updated_at)
         VALUES (?, ?, ?, 'chosen', ?, ?)",
        params![id, character_id, edge_id, TEST_TIMESTAMP, TEST_TIMESTAMP],
    )
    .unwrap();
}

/// Inserts a test character hindrance link.
pub fn insert_test_character_hindrance(conn: &Connection, id: i64, character_id: i64, hindrance_id: i64) {
    conn.execute(
        "INSERT INTO character_hindrances (id, character_id, hindrance_id, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?)",
        params![id, character_id, hindrance_id, TEST_TIMESTAMP, TEST_TIMESTAMP],
    )
    .unwrap();
}

/// Inserts a test character power link.
pub fn insert_test_character_power(conn: &Connection, id: i64, character_id: i64, power_id: i64) {
    conn.execute(
        "INSERT INTO character_powers (id, character_id, power_id, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?)",
        params![id, character_id, power_id, TEST_TIMESTAMP, TEST_TIMESTAMP],
    )
    .unwrap();
}

/// Inserts a test character arcane background link.
pub fn insert_test_character_arcane_background(conn: &Connection, id: i64, character_id: i64, arcane_background_id: i64) {
    conn.execute(
        "INSERT INTO character_arcane_backgrounds (id, character_id, arcane_background_id, created_at, updated_at)
         VALUES (?, ?, ?, ?, ?)",
        params![id, character_id, arcane_background_id, TEST_TIMESTAMP, TEST_TIMESTAMP],
    )
    .unwrap();
}
