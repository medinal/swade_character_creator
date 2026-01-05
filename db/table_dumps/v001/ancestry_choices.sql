PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
CREATE TABLE ancestry_choices (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    ancestry_id INTEGER NOT NULL,
    choice_type VARCHAR(50) NOT NULL, -- 'free_edge', 'mandatory_hindrance', 'heritage_trait', etc.
    choice_category VARCHAR(50), -- 'background_edge', 'combat_edge', etc. (for filtering)
    min_selections INTEGER NOT NULL DEFAULT 1,
    max_selections INTEGER NOT NULL DEFAULT 1,
    description TEXT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (ancestry_id) REFERENCES ancestries(id) ON DELETE CASCADE
);
INSERT INTO ancestry_choices VALUES(1,1,'free_edge',NULL,1,1,'Choose one free Novice Edge during character creation','2025-10-08 06:13:12','2025-10-08 06:13:12');
INSERT INTO ancestry_choices VALUES(2,6,'mandatory_hindrance',NULL,1,1,'Must take Pacifist (Major) hindrance due to Asimov Circuits programming','2025-10-08 06:13:12','2025-10-08 06:13:12');
INSERT INTO ancestry_choices VALUES(3,7,'ancestral_enemy',NULL,1,1,'Choose a common ancestry in your setting as ancestral enemy','2025-10-08 06:13:12','2025-10-08 06:13:12');
INSERT INTO ancestry_choices VALUES(4,10,'mandatory_edge',NULL,1,1,'Attractive: Has the Attractive Edge (unearthly beauty)','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choices VALUES(5,10,'mandatory_hindrance',NULL,1,1,'Code of Honor: Has the Code of Honor Hindrance (abhor lies, protect the weak, seek justice)','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choices VALUES(6,10,'mandatory_hindrance',NULL,1,1,'Vow (Major): Must serve the deity who created them','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choices VALUES(7,12,'breath_weapon_shape',NULL,1,1,'Breath Weapon Shape: Choose Cone Template or 12" Line at character creation','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choices VALUES(8,12,'mandatory_hindrance',NULL,1,1,'Ill-Tempered: Has the Arrogant Hindrance (proud of their dragon heritage)','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choices VALUES(9,14,'heritage',NULL,1,1,'Elemental Heritage: Choose your ancestral element (Air, Earth, Fire, or Water) to gain an additional ability','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choices VALUES(10,15,'mandatory_hindrance',NULL,1,1,'Has the All Thumbs Hindrance (inherent dislike of mechanical objects)','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choices VALUES(11,16,'mandatory_hindrance',NULL,1,1,'All Thumbs: Has the All Thumbs Hindrance (disdain for mechanical objects)','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choices VALUES(12,16,'mandatory_hindrance',NULL,1,1,'Big Mouth: Has the Big Mouth Hindrance (rarely keep secrets)','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choices VALUES(13,16,'mandatory_hindrance',NULL,1,1,'Curious: Has the Curious Hindrance (endless questions about life)','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choices VALUES(14,16,'mandatory_hindrance',NULL,1,1,'Impulsive: Has the Impulsive Hindrance (lack restraint)','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choices VALUES(15,18,'free_edge',NULL,1,1,'Survivors: Free Novice Edge of your choice (must meet requirements)','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choices VALUES(16,20,'mandatory_hindrance','outsider',1,1,'Outsider (Major): Give off uneasy aura, may be mistaken for vampires','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choices VALUES(17,21,'heritage',NULL,1,1,'Heritage: Choose either a free Novice Edge OR start with d6 Agility (maximum d12+1)','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choices VALUES(18,23,'mandatory_hindrance',NULL,1,1,'Outsider (Major): Terrifying to most humanoid races, no basic rights outside own familial clans','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choices VALUES(19,24,'heritage',NULL,1,1,'Hardened: Start with either d6 Strength or d6 Vigor instead of d4 (maximum d12+1)','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choices VALUES(20,25,'free_edge',NULL,1,1,'Adaptable: Choose one free Novice Edge during character creation (must meet requirements)','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choices VALUES(21,26,'mandatory_hindrance','outsider',1,1,'Outsider (Minor): -2 Persuasion (no one truly trusts devilish heritage)','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choices VALUES(22,28,'mandatory_hindrance','thin_skinned',1,1,'Thin Skinned (Major): Quick to lose temper when Taunted','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choices VALUES(23,28,'mandatory_hindrance','mean',1,1,'Mean: -1 Persuasion (might makes right mentality)','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choices VALUES(24,29,'mandatory_hindrance',NULL,1,1,'Outsider (Major): -2 Persuasion, no basic rights outside own people','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choices VALUES(25,29,'mandatory_hindrance',NULL,1,1,'Phobia (Minor) Cats: Instinctual fear of cats','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choices VALUES(26,29,'mandatory_hindrance',NULL,1,1,'Unimposing: Has the Mild Mannered Hindrance (most foes do not consider mouselings a threat)','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choices VALUES(27,30,'mandatory_hindrance',NULL,1,1,'Arrogant: Has the Arrogant Hindrance (culture of boasts and deeds)','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choices VALUES(28,32,'ancestral_enemy',NULL,1,1,'Ancestral Enemy: Choose a common race in your setting. -2 Persuasion when dealing with each other, often attack on sight','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choices VALUES(29,33,'mandatory_edge',NULL,1,1,'Scavenger: Has the Scavenger Edge (obsessively collect items)','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choices VALUES(30,33,'mandatory_hindrance',NULL,1,1,'Greedy (Minor): Obsessed with acquiring treasures and shinies','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choices VALUES(31,33,'mandatory_hindrance',NULL,1,1,'Outsider (Major): Treated as monstrous vermin by most other races','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choices VALUES(32,33,'mandatory_hindrance',NULL,1,1,'Craven: Has the Yellow/Timid Hindrance (discretion is the better part of valor)','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choices VALUES(33,34,'mandatory_edge',NULL,1,1,'Keen Senses: Has the Alertness Edge (acute senses)','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choices VALUES(34,36,'mandatory_edge','charismatic',1,1,'Charismatic: Has the Charismatic Edge (quickly learn to navigate tricky conversations)','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choices VALUES(35,36,'mandatory_hindrance',NULL,1,1,'Secret (Major): True nature rarely known; if discovered, trades for Enemy/Shamed/Wanted/Outsider','2026-01-03 21:24:41','2026-01-03 21:24:41');
COMMIT;
