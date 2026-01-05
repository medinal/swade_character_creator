PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
CREATE TABLE character_hindrances (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    character_id INTEGER NOT NULL,
    hindrance_id INTEGER NOT NULL,
    source VARCHAR(20) NOT NULL DEFAULT 'chosen', -- 'ancestry', 'chosen'
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (character_id) REFERENCES characters(id),
    FOREIGN KEY (hindrance_id) REFERENCES hindrances(id),
    UNIQUE(character_id, hindrance_id)
);
COMMIT;
