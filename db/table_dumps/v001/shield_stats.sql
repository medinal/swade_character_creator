PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
CREATE TABLE shield_stats (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    gear_id INTEGER NOT NULL UNIQUE,
    parry_bonus INTEGER NOT NULL, -- Bonus to Parry
    cover_penalty INTEGER NOT NULL DEFAULT 0, -- Penalty to attackers' ranged attacks
    min_strength INTEGER, -- Minimum strength die size
    notes TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (gear_id) REFERENCES gear(id) ON DELETE CASCADE
);
INSERT INTO shield_stats VALUES(1,86,1,0,NULL,NULL,'2025-12-30 18:25:49','2025-12-30 18:25:49');
INSERT INTO shield_stats VALUES(2,87,2,2,NULL,NULL,'2025-12-30 18:25:49','2025-12-30 18:25:49');
INSERT INTO shield_stats VALUES(3,88,2,4,8,NULL,'2025-12-30 18:25:49','2025-12-30 18:25:49');
INSERT INTO shield_stats VALUES(4,89,2,2,NULL,'Clear polycarbonate','2025-12-30 18:25:49','2025-12-30 18:25:49');
INSERT INTO shield_stats VALUES(5,90,2,4,8,'+2 Armor vs bullets','2025-12-30 18:25:49','2025-12-30 18:25:49');
INSERT INTO shield_stats VALUES(6,91,1,0,NULL,'Lightweight force field','2025-12-30 18:25:49','2025-12-30 18:25:49');
COMMIT;
