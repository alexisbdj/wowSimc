use crate::general_parser::{get_key_value, trim_quotes};

#[derive(PartialEq, Eq, Copy, Clone)]
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

pub struct CharacterInfo {
    pub class: CharacterClass,
    pub name: String,
    pub spec: String,
}

fn get_class_from_name(name: &str) -> CharacterClass
{
    for pair in &CLASS_NAMES {
        let current = pair.0;
        if  name == current {
            return pair.1
        }
    }
    CharacterClass::Unknown
}

impl CharacterInfo {
    pub fn new() -> Self {
        CharacterInfo {
            class: CharacterClass::Unknown,
            name: String::from("unknown"),
            spec: String::from("unknown"),
        }
    }

    fn update_except_class_name(&mut self, key: &str, value: &str)
    {
        if key == "spec" {
            self.spec = String::from(value);
            self.spec = self.spec.replace("\"", "");    //TODO : retirer ça gérer les guillemets dans update_from_line
        }
    }

    fn update_from_key_value(&mut self, key: &str, value: &str)
    {
        match get_class_from_name(key) {
            CharacterClass::Unknown => {
                self.update_except_class_name(key, value);
            },
            class_type => {
                self.class = class_type;
                self.name = String::from(value);
            }
        }
    }
    
    pub fn update_from_line(&mut self, line: String) {
        if !(line.len() == 0 || line.starts_with("#")) {
            match get_key_value(&line) {
                Ok(result) => {
                    self.update_from_key_value(result.0, trim_quotes(result.1));
                },
                Err(error) => {
                    eprintln!("ERROR: {}", error);
                }
            }
        }
    }

    pub fn dump(&self) {
        println!("name: {}", self.name);
        println!("class: {}", self.class);
        println!("spec: {}", self.spec);
    }
}