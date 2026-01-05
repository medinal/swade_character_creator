PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
CREATE TABLE character_skills (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    character_id INTEGER NOT NULL,
    skill_id INTEGER NOT NULL,
    current_die_size INTEGER, -- NULL for untrained skills
    current_die_modifier INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (character_id) REFERENCES characters(id),
    FOREIGN KEY (skill_id) REFERENCES skills(id),
    UNIQUE(character_id, skill_id)
);
COMMIT;
