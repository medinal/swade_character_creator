PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
CREATE TABLE poison_stats (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    gear_id INTEGER NOT NULL UNIQUE,
    poison_type VARCHAR(30) NOT NULL, -- 'Lethal', 'Knockout', 'Paralyzing', 'Mild', 'Disabling'
    delivery_method VARCHAR(20) NOT NULL, -- 'Contact', 'Inhaled', 'Ingested', 'Injury'
    affected_attribute VARCHAR(20), -- For Disabling type: 'Agility', 'Smarts', 'Spirit', 'Strength', 'Vigor'
    notes TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (gear_id) REFERENCES gear(id) ON DELETE CASCADE
);
INSERT INTO poison_stats VALUES(1,251,'Lethal','Ingested',NULL,'-2 Notice to detect','2026-01-04 23:33:12','2026-01-04 23:33:12');
INSERT INTO poison_stats VALUES(2,252,'Mild','Inhaled',NULL,NULL,'2026-01-04 23:33:12','2026-01-04 23:33:12');
INSERT INTO poison_stats VALUES(3,253,'Knockout','Inhaled',NULL,NULL,'2026-01-04 23:33:12','2026-01-04 23:33:12');
INSERT INTO poison_stats VALUES(4,254,'Disabling','Ingested','Smarts',NULL,'2026-01-04 23:33:12','2026-01-04 23:33:12');
INSERT INTO poison_stats VALUES(5,255,'Lethal','Injury',NULL,NULL,'2026-01-04 23:33:12','2026-01-04 23:33:12');
INSERT INTO poison_stats VALUES(6,256,'Disabling','Injury','Agility','Goblins immune','2026-01-04 23:33:12','2026-01-04 23:33:12');
INSERT INTO poison_stats VALUES(7,257,'Paralyzing','Contact',NULL,'+1 Notice to detect','2026-01-04 23:33:12','2026-01-04 23:33:12');
INSERT INTO poison_stats VALUES(8,258,'Disabling','Inhaled','Strength','May cause addiction','2026-01-04 23:33:12','2026-01-04 23:33:12');
INSERT INTO poison_stats VALUES(9,259,'Disabling','Contact','Vigor',NULL,'2026-01-04 23:33:12','2026-01-04 23:33:12');
INSERT INTO poison_stats VALUES(10,260,'Disabling','Ingested','Spirit',NULL,'2026-01-04 23:33:12','2026-01-04 23:33:12');
INSERT INTO poison_stats VALUES(11,261,'Paralyzing','Injury',NULL,NULL,'2026-01-04 23:33:12','2026-01-04 23:33:12');
COMMIT;
