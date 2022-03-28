use crate::database::types::Item;
use crate::simc::item_set;

fn single_item_profile(item: &Item, name: &String, inv_type: &String, precision: Option<&String>) -> String
{
    let mut result = String::new();

    result.push_str(&format!("\ncopy=\"{}{},{}\"\n", item.name,
    match precision {
        Some(precision) => {
            format!(" ({})", precision)
        },
        None => {
            String::new()
        }
    },
    name
));
    result.push_str(&format!("{}=\",id={},ilevel={}\"\n", inv_type, item.id, 262));
    return result;
}

fn get_finger<'a>(items: &'a Vec<Item>) -> Option<&'a Item> {
    for i in items {
        if i.inv_type == "FINGER" {
            return Some(i);
        }
    }

    None
}

#[allow(dead_code)]
pub fn fast_droptimizer(items: Vec<Item>, name: &String) {
    let mut result = String::new();
    for item in items {
        if item.inv_type == "FINGER" {
            result.push_str(&single_item_profile(&item, name, &String::from("finger1"), Some(&String::from("finger1"))));
            result.push_str(&single_item_profile(&item, name, &String::from("finger2"), Some(&String::from("finger2"))));
        }
        else {
            result.push_str(&single_item_profile(&item, name, &item.inv_type.to_lowercase(), None));
        }
    }
    
    println!("{}", result);
}

#[allow(dead_code)]
pub fn full_dungeon_set(items: Vec<Item>, name: &String) {
    let mut iset = item_set::ItemSet::new();
    
    if let Some(fitem) = get_finger(&items) {
        iset.finger1 = item_set::EquippedItem::Item(crate::simc::item_info::ItemInfo {
            id: fitem.id,
            inv_type: fitem.inv_type.clone(),
        });
        iset.finger2 = item_set::EquippedItem::Item(crate::simc::item_info::ItemInfo {
            id: fitem.id,
            inv_type: fitem.inv_type.clone(),
        });
    }
    
    let result = iset.to_simc_input(String::from("testFinger"), name.to_string());

    println!("{}", result);
}