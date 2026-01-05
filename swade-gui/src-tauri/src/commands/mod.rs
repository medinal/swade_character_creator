//! Tauri command modules organized by domain.

pub mod advancement;
pub mod ancestry;
pub mod attributes;
pub mod character;
pub mod edges;
pub mod gear;
pub mod hindrances;
pub mod notes;
pub mod powers;
pub mod skills;

// Re-export all commands for convenient access
pub use advancement::*;
pub use ancestry::*;
pub use attributes::*;
pub use character::*;
pub use edges::*;
pub use gear::*;
pub use hindrances::*;
pub use notes::*;
pub use powers::*;
pub use skills::*;
