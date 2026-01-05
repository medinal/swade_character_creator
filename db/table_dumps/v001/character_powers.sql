PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
CREATE TABLE character_powers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    character_id INTEGER NOT NULL,
    power_id INTEGER NOT NULL,
    advance_taken INTEGER, -- Which advance was used (NULL for starting powers from arcane background)
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (character_id) REFERENCES characters(id),
    FOREIGN KEY (power_id) REFERENCES powers(id),
    UNIQUE(character_id, power_id)
);
COMMIT;
