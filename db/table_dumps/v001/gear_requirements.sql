PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
CREATE TABLE gear_requirements (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    gear_id INTEGER NOT NULL,
    requirement_expression_id INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (gear_id) REFERENCES gear(id) ON DELETE CASCADE,
    FOREIGN KEY (requirement_expression_id) REFERENCES requirement_expressions(id),
    UNIQUE(gear_id, requirement_expression_id)
);
COMMIT;
