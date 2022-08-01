pub mod enchant_list;

pub struct ItemInfo {
    pub id: u32,
    pub inv_type: String,
    pub enchant: Option<&'static str>,
}
