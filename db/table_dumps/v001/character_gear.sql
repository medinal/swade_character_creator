PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
CREATE TABLE character_gear (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    character_id INTEGER NOT NULL,
    gear_id INTEGER NOT NULL,
    quantity INTEGER NOT NULL DEFAULT 1,
    is_equipped BOOLEAN NOT NULL DEFAULT 0,
    custom_notes TEXT, -- Player notes about this specific item
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (character_id) REFERENCES characters(id) ON DELETE CASCADE,
    FOREIGN KEY (gear_id) REFERENCES gear(id)
);
COMMIT;
