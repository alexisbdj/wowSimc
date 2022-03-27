pub struct Item {
    pub id: u32,
    pub name: String,
    pub level: u32,
    pub is_equippable: bool,
    pub item_class: u32,
    pub item_sub_class: u32,
    pub inv_type: String,
}

impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} | {}", self.id, self.name)
    }
}

#[derive(Copy, Clone)]
pub struct ItemClass {
    pub class: u32,
    pub sub_class: u32,
}

impl std::fmt::Display for ItemClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} | {}", self.class, self.sub_class)
    }
}