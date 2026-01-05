PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
CREATE TABLE arcane_background_choice_options (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    choice_id INTEGER NOT NULL,
    option_type VARCHAR(50) NOT NULL, -- 'power', 'hindrance', 'ability', 'edge_category'
    option_id INTEGER, -- FK to powers/hindrances (NULL for abilities/edge_categories)
    option_description TEXT, -- For abilities or power limitations (e.g., "Summoned creatures only")
    position INTEGER NOT NULL DEFAULT 0,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (choice_id) REFERENCES arcane_background_choices(id) ON DELETE CASCADE
);
COMMIT;
