PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
CREATE TABLE character_edges (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    character_id INTEGER NOT NULL,
    edge_id INTEGER NOT NULL,
    advance_taken INTEGER NOT NULL, -- Which advance number was used to take this edge
    notes TEXT, -- Optional notes for edges taken multiple times (e.g., which weapon for Trademark Weapon, which skill for Scholar)
    source VARCHAR(20) NOT NULL DEFAULT 'advancement', -- 'ancestry', 'advancement', 'hindrance_points'
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (character_id) REFERENCES characters(id),
    FOREIGN KEY (edge_id) REFERENCES edges(id)
    -- Note: UNIQUE constraint removed to allow taking same edge multiple times (for edges where can_take_multiple_times = 1)
);
COMMIT;
