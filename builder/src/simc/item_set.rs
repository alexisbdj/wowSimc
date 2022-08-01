use super::item_info;

pub enum EquippedItem {
    Default,
    Item(item_info::ItemInfo),
    Locked,
}

/// each item slots
pub enum ItemSlot {
    Head,
    Shoulder,
    Neck,
    Chest,
    Waist,
    Wrist,
    Hands,
    Legs,
    Feet,
    Finger1,
    Finger2,
}

/// contains item set
/// 
/// can be converted into simc input
pub struct ItemSet {
    pub head: EquippedItem,
    pub shoulder: EquippedItem,
    pub neck: EquippedItem,
    pub chest: EquippedItem,
    pub waist: EquippedItem,
    pub wrist: EquippedItem,
    pub hands: EquippedItem,
    pub legs: EquippedItem,
    pub feet: EquippedItem,
    pub finger1: EquippedItem,
    pub finger2: EquippedItem,
}

/// list that contains all slots to be iterated on
static SLOT_LIST: [ItemSlot; 11] = [
    ItemSlot::Head,
    ItemSlot::Shoulder,
    ItemSlot::Neck,
    ItemSlot::Chest,
    ItemSlot::Waist,
    ItemSlot::Wrist,
    ItemSlot::Hands,
    ItemSlot::Legs,
    ItemSlot::Feet,
    ItemSlot::Finger1,
    ItemSlot::Finger2,
];

impl ItemSet {
    pub fn new() -> Self {
        ItemSet {
            head: EquippedItem::Default,
            shoulder: EquippedItem::Default,
            neck: EquippedItem::Default,
            chest: EquippedItem::Default,
            waist: EquippedItem::Default,
            wrist: EquippedItem::Default,
            hands: EquippedItem::Default,
            legs: EquippedItem::Default,
            feet: EquippedItem::Default,
            finger1: EquippedItem::Default,
            finger2: EquippedItem::Default,
        }
    }

    /// get pointer to the item at the specified slot
    pub fn get_item_from_slot<'a>(&'a self, slot: &ItemSlot) -> &'a EquippedItem {
        match slot {
            ItemSlot::Head => &self.head,
            ItemSlot::Shoulder => &self.shoulder,
            ItemSlot::Neck => &self.neck,
            ItemSlot::Chest => &self.chest,
            ItemSlot::Waist => &self.waist,
            ItemSlot::Wrist => &self.wrist,
            ItemSlot::Hands => &self.hands,
            ItemSlot::Legs => &self.legs,
            ItemSlot::Feet => &self.feet,
            ItemSlot::Finger1 => &self.finger1,
            ItemSlot::Finger2 => &self.finger2,
        }
    }

    /// convert this set into simc input using copy syntax
    pub fn to_simc_input(&self, set_name: String, profile_name: String) -> String {
        let mut result = format!("\ncopy=\"{},{}\"\n",
            set_name,
            profile_name);
        for slot in &SLOT_LIST {
            let slot_name = get_slot_name(slot);
            if let EquippedItem::Item(item) = self.get_item_from_slot(slot) {
                result.push_str(&format!("{}=\",id={},ilevel={}\"\n", slot_name, item.id, 262));
            }
        }
        result
    }
}

/// get simc usage slot name
fn get_slot_name(slot: &ItemSlot) -> &'static str
{
    match slot {
        ItemSlot::Head => "head",
        ItemSlot::Shoulder => "shoulder",
        ItemSlot::Neck => "neck",
        ItemSlot::Chest => "chest",
        ItemSlot::Waist => "waist",
        ItemSlot::Wrist => "wrist",
        ItemSlot::Hands => "hands",
        ItemSlot::Legs => "legs",
        ItemSlot::Feet => "feet",
        ItemSlot::Finger1 => "finger1",
        ItemSlot::Finger2 => "finger2",
    }
}

pub fn get_slot_from_name(name: &String) -> ItemSlot
{
    match name.as_str() {
        "head" => ItemSlot::Head,
        "shoulder" => ItemSlot::Shoulder,
        "neck" => ItemSlot::Neck,
        "chest" => ItemSlot::Chest,
        "waist" => ItemSlot::Waist,
        "wrist" => ItemSlot::Wrist,
        "hands" => ItemSlot::Hands,
        "legs" => ItemSlot::Legs,
        "feet" => ItemSlot::Feet,
        "finger1" => ItemSlot::Finger1,
        "finger2" => ItemSlot::Finger2,
        name => panic!("get_slot_from_name: go unexpected slot name: {}", name)
    }
}