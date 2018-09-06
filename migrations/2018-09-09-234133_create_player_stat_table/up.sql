-- Your SQL goes here
CREATE TABLE IF NOT EXISTS `player`(
    `id` INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    `cohort_id` INTEGER NOT NULL,
    `name` VARCHAR(40) NOT NULL,
    `rank` INTEGER NOT NULL,
    `revives` REAL NOT NULL,
    `repairs` REAL NOT NULL,
    `heals` REAL NOT NULL,
    `rounds_played` INTEGER NOT NULL,
    `squad_score` REAL NOT NULL,
    `flag_captures` INTEGER NOT NULL,
    `flag_defends` INTEGER NOT NULL,
    `gamemode_score` REAL NOT NULL,
    `time_played` INTEGER NOT NULL
);