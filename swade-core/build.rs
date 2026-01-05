use sha2::{Digest, Sha256};
use std::fs;
use std::path::Path;

fn main() {
    // Database is at workspace root, build.rs runs from swade-core/
    let db_path = Path::new("../db/swade.db");

    // Calculate SHA256 checksum of the database
    if db_path.exists() {
        let db_bytes = fs::read(db_path).expect("Failed to read ../db/swade.db");

        let mut hasher = Sha256::new();
        hasher.update(&db_bytes);
        let checksum = format!("{:x}", hasher.finalize());

        // Set as environment variable for compile-time inclusion
        println!("cargo:rustc-env=SWADE_DB_CHECKSUM={}", checksum);
    } else {
        // Provide a placeholder checksum if database doesn't exist yet
        // This allows the project to compile without the database for CI/testing
        println!("cargo:rustc-env=SWADE_DB_CHECKSUM=no_database");
    }

    // Rebuild if database changes
    println!("cargo:rerun-if-changed=../db/swade.db");
    println!("cargo:rerun-if-changed=build.rs");
}
