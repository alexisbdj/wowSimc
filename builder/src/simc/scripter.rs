use crate::database::types::Item;
use crate::simc::item_set;
use crate::simc::item_info::enchant_list::*;
/// same as single_item_profile but creates one profile for each enchant
fn all_enchants_item_profile(item: &Item, name :&String, inv_type: &String, precision: Option<&String>) -> String
{
    let mut result = String::new();
    
    match get_slot_enchants(item_set::get_slot_from_name(inv_type)) {
        Some(enchant_list) => {
            for enchant in enchant_list {
                let precision = match precision {
                    Some(precision) => format!("({} | {})", precision, enchant),
                    None => format!("({})", enchant),
                };
                result.push_str(&format!("\ncopy=\"{}{},{}\"\n", item.name,
                    precision,
                    name
                ));
                result.push_str(&format!("{}=\",id={},ilevel={},enchant={}\"\n", inv_type, item.id, 262, enchant));
            }
        },
        None => {
            result.push_str(&single_item_profile(item, name, inv_type, precision));
        }
    }


    result
}

/// use copy syntax to create profile with only one new item
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

/// quickly get first finger from list
fn get_finger<'a>(items: &'a Vec<Item>) -> Option<&'a Item> {
    for i in items {
        if i.inv_type == "FINGER" {
            return Some(i);
        }
    }
    None
}

/// test each item separately
#[allow(dead_code)]
pub fn fast_droptimizer(items: Vec<Item>, name: &String) {
    let mut result = String::new();
    for item in items {
        match item.inv_type.as_str() {
            "FINGER" => {
                result.push_str(&all_enchants_item_profile(&item, name, &String::from("finger1"), Some(&String::from("finger1"))));
                result.push_str(&all_enchants_item_profile(&item, name, &String::from("finger2"), Some(&String::from("finger2"))));
            },
            "TWOHWEAPON" => {
                result.push_str(&all_enchants_item_profile(&item, name, &String::from("main_hand"), None));
            },
            "HAND" => {
                result.push_str(&all_enchants_item_profile(&item, name, &String::from("hands"), None));
            },
            _ => {
                result.push_str(&all_enchants_item_profile(&item, name, &item.inv_type.to_lowercase(), None));
            }
        }
    }
    
    println!("{}", result);
}

/// find best possible dungeon set tesing all possibilities
#[allow(dead_code)]
pub fn full_dungeon_set(items: Vec<Item>, name: &String) {
    let mut iset = item_set::ItemSet::new();
    
    if let Some(fitem) = get_finger(&items) {
        iset.finger1 = item_set::EquippedItem::Item(crate::simc::item_info::ItemInfo {
            id: fitem.id,
            inv_type: fitem.inv_type.clone(),
            enchant: None,
        });
        iset.finger2 = item_set::EquippedItem::Item(crate::simc::item_info::ItemInfo {
            id: fitem.id,
            inv_type: fitem.inv_type.clone(),
            enchant: None,
        });
    }
    
    let result = iset.to_simc_input(String::from("testFinger"), name.to_string());

    println!("{}", result);
}