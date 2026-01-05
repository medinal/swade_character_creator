PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
CREATE TABLE character_arcane_backgrounds (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    character_id INTEGER NOT NULL,
    arcane_background_id INTEGER NOT NULL,
    advance_taken INTEGER DEFAULT 0, -- Which advance was used to take this Arcane Background edge
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (character_id) REFERENCES characters(id),
    FOREIGN KEY (arcane_background_id) REFERENCES arcane_backgrounds(id),
    UNIQUE(character_id, arcane_background_id)
);
COMMIT;
