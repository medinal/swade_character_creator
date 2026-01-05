PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
CREATE TABLE character_modifiers (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    character_id INTEGER NOT NULL,
    modifier_id INTEGER NOT NULL,
    advance_taken INTEGER, -- Which advance was used (NULL for character creation)
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (character_id) REFERENCES characters(id),
    FOREIGN KEY (modifier_id) REFERENCES modifiers(id),
    UNIQUE(character_id, modifier_id) -- Enforces one-to-many
);
COMMIT;
