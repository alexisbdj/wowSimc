use super::item_info;

pub enum EquippedItem {
    Default,
    Item(item_info::ItemInfo),
    Locked,
}

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

pub struct ItemSet {
    head: EquippedItem,
    shoulder: EquippedItem,
    neck: EquippedItem,
    chest: EquippedItem,
    waist: EquippedItem,
    wrist: EquippedItem,
    hands: EquippedItem,
    legs: EquippedItem,
    feet: EquippedItem,
    finger1: EquippedItem,
    finger2: EquippedItem,
}

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

    pub fn get_item_from_slot<'a>(&'a self, slot: ItemSlot) -> &'a EquippedItem {
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

    pub fn to_simc_input(&self, set_name: String, profile_name: String) -> String {
        let mut result = String::new();
        result
    }
}
