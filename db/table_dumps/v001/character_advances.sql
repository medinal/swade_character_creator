PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
CREATE TABLE character_advances (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    character_id INTEGER NOT NULL,
    advance_number INTEGER NOT NULL,  -- 1, 2, 3, etc.
    advance_type VARCHAR(30) NOT NULL,  -- 'edge', 'attribute', 'skill_expensive', 'skill_cheap', 'hindrance'
    -- For edge advances
    edge_id INTEGER,
    -- For attribute advances
    attribute_id INTEGER,
    -- For skill advances (expensive = 1 skill, cheap = 2 skills)
    skill_id_1 INTEGER,
    skill_id_2 INTEGER,  -- Only used for cheap skill advances
    -- For hindrance advances
    hindrance_id INTEGER,
    hindrance_action VARCHAR(20),  -- 'remove_minor', 'reduce_major', 'remove_major_half'
    -- Metadata
    notes TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (character_id) REFERENCES characters(id) ON DELETE CASCADE,
    FOREIGN KEY (edge_id) REFERENCES edges(id),
    FOREIGN KEY (attribute_id) REFERENCES attributes(id),
    FOREIGN KEY (skill_id_1) REFERENCES skills(id),
    FOREIGN KEY (skill_id_2) REFERENCES skills(id),
    FOREIGN KEY (hindrance_id) REFERENCES hindrances(id),
    UNIQUE (character_id, advance_number)
);
COMMIT;
