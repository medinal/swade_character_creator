PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
CREATE TABLE ancestry_requirements (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    ancestry_id INTEGER NOT NULL,
    requirement_expression_id INTEGER NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (ancestry_id) REFERENCES ancestries(id),
    FOREIGN KEY (requirement_expression_id) REFERENCES requirement_expressions(id),
    UNIQUE(ancestry_id, requirement_expression_id)
);
COMMIT;
