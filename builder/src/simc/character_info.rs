use crate::general_parser::get_key_value;

#[derive(PartialEq, Eq)]
pub enum CharacterClass {
    Unknown,
    Warrior,
    Paladin,
    Hunter,
    Rogue,
    Priest,
    Shaman,
    Mage,
    Warlock,
    Monk,
    Druid,
    DemonHunter,
    DeathKnight,
}

static CLASS_NAMES: [(&'static str, CharacterClass); 12] = [
    ("death_knight", CharacterClass::DeathKnight),
    ("demon_hunter", CharacterClass::DemonHunter),
    ("druid", CharacterClass::Druid),
    ("hunter", CharacterClass::Hunter),
    ("mage", CharacterClass::Mage),
    ("monk", CharacterClass::Monk),
    ("paladin", CharacterClass::Paladin),
    ("priest", CharacterClass::Priest),
    ("shaman", CharacterClass::Shaman),
    ("rogue", CharacterClass::Rogue),
    ("warlock", CharacterClass::Warlock),
    ("warrior", CharacterClass::Warrior)
];

impl CharacterClass {
    pub fn get_name(&self) -> &'static str {
        for pair in &CLASS_NAMES {
            let current = &pair.1;
            if *self == *current {
                return pair.0
            }
        }
        "unknown"
    }
}

impl std::fmt::Display for CharacterClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.get_name())
    }
}

pub struct character_info {
    class: CharacterClass,
}

impl character_info {
    pub fn new() -> Self {
        character_info {
            class: CharacterClass::Unknown,
        }
    }

    fn update_from_key_value(&mut self, key: &str, value: &str)
    {
        println!("{}: {}", key, value)
    }
    
    pub fn update_from_line(&mut self, line: String) {
        if line.len() == 0 || line.starts_with("#") {
            println!("comment found");
        }
        else {
            println!("process: {}", line);
            match get_key_value(&line) {
                Ok(result) => {
                    self.update_from_key_value(result.0, result.1);
                },
                Err(error) => {
                    eprintln!("ERROR: {}", error);
                }
            }
        }


    }

    pub fn dump(&self) {
        println!("class: {}", self.class);
    }
}