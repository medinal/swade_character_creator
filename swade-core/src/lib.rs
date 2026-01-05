pub mod constants;
pub mod db;
pub mod error;
pub mod models;
pub mod repositories;
pub mod services;
pub mod views;

#[cfg(test)]
pub mod test_utils;

// Re-export rusqlite::Connection for CLI convenience
pub use rusqlite::Connection;
