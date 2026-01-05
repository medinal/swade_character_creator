PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
CREATE TABLE ammunition_stats (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    gear_id INTEGER NOT NULL UNIQUE,
    ammo_type VARCHAR(30) NOT NULL, -- e.g., "pistol", "rifle", "shotgun", "arrow"
    quantity_per_unit INTEGER NOT NULL DEFAULT 1, -- How many rounds per purchase
    notes TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (gear_id) REFERENCES gear(id) ON DELETE CASCADE
);
INSERT INTO ammunition_stats VALUES(1,92,'arrow',20,NULL,'2025-12-30 18:25:49','2025-12-30 18:25:49');
INSERT INTO ammunition_stats VALUES(2,93,'bolt',20,NULL,'2025-12-30 18:25:49','2025-12-30 18:25:49');
INSERT INTO ammunition_stats VALUES(3,94,'small_bullet',50,'.22 caliber','2025-12-30 18:25:49','2025-12-30 18:25:49');
INSERT INTO ammunition_stats VALUES(4,95,'medium_bullet',50,'9mm, .45 ACP','2025-12-30 18:25:49','2025-12-30 18:25:49');
INSERT INTO ammunition_stats VALUES(5,96,'large_bullet',50,'.357, .50 caliber','2025-12-30 18:25:49','2025-12-30 18:25:49');
INSERT INTO ammunition_stats VALUES(6,97,'shotgun_shell',25,'12-gauge','2025-12-30 18:25:49','2025-12-30 18:25:49');
INSERT INTO ammunition_stats VALUES(7,98,'rifle_round',50,'Rifle caliber','2025-12-30 18:25:49','2025-12-30 18:25:49');
INSERT INTO ammunition_stats VALUES(8,99,'energy_cell',1,'Power for energy weapons','2025-12-30 18:25:49','2025-12-30 18:25:49');
COMMIT;
