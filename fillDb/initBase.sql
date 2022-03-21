CREATE TABLE Item (
    id INT UNSIGNED PRIMARY KEY,
    name varchar(100) NOT NULL,
    level INT UNSIGNED NOT NULL,
    isEquippable BOOLEAN NOT NULL,
    itemClass INT UNSIGNED NOT NULL,
    itemSubClass INT UNSIGNED NOT NULL,
    invType varchar(25) NOT NULL
);

CREATE TABLE Dungeon (
    name varchar(50) PRIMARY KEY
);

CREATE TABLE SourceDrop (
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    dungeonName varchar(50) NOT NULL,
    itemId INT UNSIGNED NOT NULL,
    FOREIGN KEY (itemId) REFERENCES Item(id) ON DELETE CASCADE,
    FOREIGN KEY (dungeonName) REFERENCES Dungeon(name) ON DELETE CASCADE
);

CREATE TABLE ClassEquipment (
    id INT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
    class VARCHAR(25) NOT NULL,
    spec VARCHAR(25),
    itemClass INT UNSIGNED NOT NULL,
    itemSubClass INT UNSIGNED NOT NULL
);

INSERT INTO ClassEquipment (class, spec, itemClass, itemSubClass) VALUES
("druid", NULL, 4, 2),
("druid", "balance", 2, 10);