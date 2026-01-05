PRAGMA foreign_keys=OFF;
BEGIN TRANSACTION;
CREATE TABLE ancestry_choice_options (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    choice_id INTEGER NOT NULL,
    option_type VARCHAR(50) NOT NULL, -- 'edge', 'hindrance', 'modifier'
    option_id INTEGER, -- specific ID or NULL for "any"
    option_description TEXT,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (choice_id) REFERENCES ancestry_choices(id) ON DELETE CASCADE
);
INSERT INTO ancestry_choice_options VALUES(1,1,'edge',1,'Alertness - +2 to Notice rolls','2025-11-01 02:47:45','2025-11-01 02:47:45');
INSERT INTO ancestry_choice_options VALUES(2,1,'edge',3,'Arcane Background - Gain supernatural powers','2025-11-01 02:47:45','2025-11-01 02:47:45');
INSERT INTO ancestry_choice_options VALUES(3,1,'edge',6,'Aristocrat - +2 Persuasion with elites','2025-11-01 02:47:45','2025-11-01 02:47:45');
INSERT INTO ancestry_choice_options VALUES(4,1,'edge',9,'Berserk - Rage in combat for increased power','2025-11-01 02:47:45','2025-11-01 02:47:45');
INSERT INTO ancestry_choice_options VALUES(5,1,'edge',15,'Fame - Known for your skills, earn double fees','2025-11-01 02:47:45','2025-11-01 02:47:45');
INSERT INTO ancestry_choice_options VALUES(6,1,'edge',20,'Luck - Draw one extra Benny per session','2025-11-01 02:47:45','2025-11-01 02:47:45');
INSERT INTO ancestry_choice_options VALUES(7,1,'edge',23,'Rich - Start with 3x normal funds','2025-11-01 02:47:45','2025-11-01 02:47:45');
INSERT INTO ancestry_choice_options VALUES(8,2,'hindrance',9,'Pacifist (Major): May not injure sapient beings or allow them to be harmed through inaction','2025-10-08 06:13:12','2025-10-08 06:13:12');
INSERT INTO ancestry_choice_options VALUES(9,3,'ancestry',1,'Humans: -2 Persuasion when dealing with each other, often attack on sight','2025-10-08 06:13:12','2025-10-08 06:13:12');
INSERT INTO ancestry_choice_options VALUES(10,3,'ancestry',2,'Dwarves: -2 Persuasion when dealing with each other, often attack on sight','2025-10-08 06:13:12','2025-10-08 06:13:12');
INSERT INTO ancestry_choice_options VALUES(11,3,'ancestry',3,'Elves: -2 Persuasion when dealing with each other, often attack on sight','2025-10-08 06:13:12','2025-10-08 06:13:12');
INSERT INTO ancestry_choice_options VALUES(12,3,'ancestry',4,'Aquarians: -2 Persuasion when dealing with each other, often attack on sight','2025-10-08 06:13:12','2025-10-08 06:13:12');
INSERT INTO ancestry_choice_options VALUES(13,3,'ancestry',5,'Avions: -2 Persuasion when dealing with each other, often attack on sight','2025-10-08 06:13:12','2025-10-08 06:13:12');
INSERT INTO ancestry_choice_options VALUES(14,4,'edge',7,'Attractive: +1 Persuasion, +1 Performance','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(15,5,'hindrance',54,'Code of Honor (Major): Abhor lies, protect the weak, seek justice','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(16,6,'hindrance',27,'Vow (Major): Must serve the deity who created them','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(17,7,'modifier',NULL,'Cone Template: Breath weapon affects Cone Template area','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(18,7,'modifier',NULL,'12" Line: Breath weapon affects 12" line','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(19,8,'hindrance',1,'Arrogant (Major) - Must humiliate opponent in one-on-one combat','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(20,9,'modifier',610,'Air Scion - Inner Air: Agility starts at d6 instead of d4 (maximum Agility d12+1)','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(21,9,'modifier',611,'Earth Scion - Rock Solid: Vigor starts at d6 instead of d4 (maximum Vigor d12+1)','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(22,9,'edge',22,'Fire Scion - Quick: Has the Quick Edge (frenetic and always in motion)','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(23,9,'modifier',612,'Water Scion - Aquatic: Spirit starts at d6 instead of d4 (maximum Spirit d12+1)','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(24,10,'hindrance',37,'All Thumbs (Minor): -2 penalty when using mechanical or electronic devices','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(25,11,'hindrance',37,'All Thumbs (Minor): -2 penalty when using mechanical or electronic devices','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(26,12,'hindrance',2,'Big Mouth (Minor): Unable to keep secrets, blabs at the worst times','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(27,13,'hindrance',5,'Curious (Major): Insatiable curiosity that leads into danger','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(28,14,'hindrance',6,'Impulsive (Major): Acts without thinking, difficult to coordinate','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(29,15,'edge',1,'Alertness - +2 to Notice rolls','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(30,15,'edge',3,'Arcane Background - Gain supernatural powers','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(31,15,'edge',6,'Aristocrat - +2 Persuasion with upper class, +2 Common Knowledge for high society','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(32,15,'edge',9,'Berserk - After being Shaken or Wounded, go berserk for +1 to Fighting, +2 damage, +2 Toughness, ignore Wound penalties','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(33,15,'edge',15,'Fame - +1 Persuasion, +1 extra money from starting funds','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(34,15,'edge',20,'Luck - +1 Benny per session','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(35,15,'edge',23,'Rich - 3x starting funds, $150,000 annual salary','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(36,15,'edge',106,'Connections - Call in favors from a group or organization','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(37,15,'edge',118,'Beast Bond - Speak with and control animals','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(38,15,'edge',122,'Danger Sense - Notice roll at -2 to detect ambushes and similar dangers','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(39,16,'hindrance',74,'Outsider (Major): -2 Persuasion, give off uneasy aura, may be mistaken for vampires','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(40,17,'modifier',653,'Elven Grace: Agility starts at d6 instead of d4 (maximum Agility d12+1)','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(41,17,'edge',1,'Alertness - +2 to Notice rolls','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(42,17,'edge',3,'Arcane Background - Gain supernatural powers','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(43,17,'edge',6,'Aristocrat - +2 Persuasion with upper class, +2 Common Knowledge for high society','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(44,17,'edge',9,'Berserk - After being Shaken or Wounded, go berserk for +1 to Fighting, +2 damage, +2 Toughness, ignore Wound penalties','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(45,17,'edge',15,'Fame - +1 Persuasion, +1 extra money from starting funds','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(46,17,'edge',20,'Luck - +1 Benny per session','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(47,17,'edge',23,'Rich - 3x starting funds, $150,000 annual salary','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(48,17,'edge',106,'Connections - Call in favors from a group or organization','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(49,17,'edge',118,'Beast Bond - Speak with and control animals','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(50,17,'edge',122,'Danger Sense - Notice roll at -2 to detect ambushes and similar dangers','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(51,18,'hindrance',74,'Outsider (Major): -2 Persuasion, no basic rights outside own clans','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(52,19,'attribute_bonus',4,'Orc Blood: Strength starts at d6 instead of d4 (maximum Strength d12+1)','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(53,19,'attribute_bonus',5,'Human Heritage: Vigor starts at d6 instead of d4 (maximum Vigor d12+1)','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(54,20,'edge',1,'Alertness - +2 to Notice rolls','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(55,20,'edge',3,'Arcane Background - Gain supernatural powers','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(56,20,'edge',6,'Aristocrat - +2 Persuasion with upper class, +2 Common Knowledge for high society','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(57,20,'edge',9,'Berserk - After being Shaken or Wounded, go berserk for +1 to Fighting, +2 damage, +2 Toughness, ignore Wound penalties','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(58,20,'edge',15,'Fame - +1 Persuasion, +1 extra money from starting funds','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(59,20,'edge',20,'Luck - +1 Benny per session','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(60,20,'edge',23,'Rich - 3x starting funds, $150,000 annual salary','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(61,20,'edge',106,'Connections - Call in favors from a group or organization','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(62,20,'edge',118,'Beast Bond - Speak with and control animals','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(63,20,'edge',122,'Danger Sense - Notice roll at -2 to detect ambushes and similar dangers','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(64,21,'hindrance',73,'Outsider (Minor): -2 Persuasion (no one truly trusts devilish heritage)','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(65,22,'hindrance',78,'Thin Skinned (Major) - Very quick to anger when Taunted','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(66,23,'hindrance',71,'Mean (Minor) - -1 Persuasion due to ill-temper and lack of social graces','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(67,24,'hindrance',74,'Outsider (Major): -2 Persuasion, no basic rights outside own people','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(68,25,'hindrance',61,'Phobia (Minor) Cats: Instinctual fear of cats, -2 to Trait rolls in presence of cats','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(69,26,'hindrance',72,'Mild Mannered (Minor): -2 to Intimidation rolls','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(70,27,'hindrance',1,'Arrogant (Major) - Must humiliate opponent in one-on-one combat','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(71,28,'ancestry',1,'Humans: -2 Persuasion when dealing with each other, often attack on sight','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(72,28,'ancestry',2,'Dwarves: -2 Persuasion when dealing with each other, often attack on sight','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(73,28,'ancestry',3,'Elves: -2 Persuasion when dealing with each other, often attack on sight','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(74,28,'ancestry',4,'Aquarians: -2 Persuasion when dealing with each other, often attack on sight','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(75,28,'ancestry',5,'Avions: -2 Persuasion when dealing with each other, often attack on sight','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(76,29,'edge',125,'Scavenger: May find or fashion useful items from junk','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(77,30,'hindrance',15,'Greedy (Minor): Obsessed with acquiring wealth and treasures','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(78,31,'hindrance',74,'Outsider (Major): -2 Persuasion, treated as monstrous vermin outside own packs','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(79,32,'hindrance',35,'Timid/Yellow (Major): Character is cowardly and must make Spirit roll to engage dangerous foes','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(80,33,'edge',1,'Alertness: +2 to Notice rolls','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(81,34,'edge',13,'Charismatic: Free reroll on Persuasion','2026-01-03 21:24:41','2026-01-03 21:24:41');
INSERT INTO ancestry_choice_options VALUES(82,35,'hindrance',21,'Secret (Major): If discovered by non-allies, trade for Enemy/Shamed/Wanted/Outsider','2026-01-03 21:24:41','2026-01-03 21:24:41');
COMMIT;
