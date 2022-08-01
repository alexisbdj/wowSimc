use crate::simc::item_set::ItemSlot;

static CHEST_ENCHANTS: [&'static str; 1] = [
    "eternal_stats",
];

static FINGER_ENCHANTS: [&'static str; 4] = [
    "16haste",
    "16mastery",
    "16crit",
    "16vers",
];

pub fn get_slot_enchants(slot: ItemSlot) -> Option<&'static [&'static str]>
{
    match slot {
        ItemSlot::Chest => Some(&CHEST_ENCHANTS),
        ItemSlot::Finger1 => Some(&FINGER_ENCHANTS),
        ItemSlot::Finger2 => Some(&FINGER_ENCHANTS),
        _ => None,
    }
}