use crate::database::types::Item;

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