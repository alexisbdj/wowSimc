use mysql::*;
use mysql::prelude::Queryable;
use crate::database::types;


/// filters used to build sql data
/// 
/// - byClass returns all items from item and item class id (and optionaly subClass)
/// - byClassList returns all items from class/subclass contained in the list
/// - byInvType returns all items from invType (ex: FINGER)
pub enum ItemFilter {
    ByClass(u32, Option<u32>),
    ByClassList(Vec<types::ItemClass>),
    ByInvType(String),
}

impl ItemFilter {
    /// convert ItemFilter to SQL statement
    pub fn get_filter(&self) -> String {
        match self {
            ItemFilter::ByClass(item_class, sub_class) => {
                let mut filter = format!("itemClass = {}", item_class);
                if let Some(sub_class) = sub_class {
                    filter.push_str(&format!(" AND itemSubClass = {}", sub_class));
                }
                filter
            },
            ItemFilter::ByClassList(class_list) => {
                let mut filter = String::new();
                
                for (index, el) in class_list.iter().enumerate() {
                    filter.push_str(&format!("{}(itemClass = {} AND itemSubClass = {})", 
                        (if index > 0 {
                            " OR "
                        }
                        else {
                            ""
                        }),
                        el.class,
                        el.sub_class
                    ))
                }
                filter
            },
            ItemFilter::ByInvType(inv_type) => {
                format!("invType = '{}'", inv_type)
            } 
        }
    }
}

/// get all items using a list of ItemFilter
pub fn by_filter_list(conn: &mut mysql::PooledConn, filter_list: &Vec<ItemFilter>) -> Result<Vec<types::Item>>
{
    let mut filter = String::new();

    for (index, el) in filter_list.iter().enumerate() {
        filter.push_str(&format!("{}({})",
            (if index > 0 {
                " OR "
            }
            else {
                ""
            }),
            el.get_filter()
        ));
    }
    get_items(conn, Some(filter))
}

/// shortcut for ItemFilter::ByClassList
#[allow(dead_code)]
pub fn by_class_list(conn: &mut mysql::PooledConn, class_list: &Vec<types::ItemClass>) -> Result<Vec<types::Item>>
{
    let filter = ItemFilter::ByClassList(class_list.to_vec()).get_filter();
    get_items(conn, Some(filter))
}

/// shortcut for ItemFilter::ByClass
#[allow(dead_code)]
pub fn by_class(conn: &mut mysql::PooledConn, item_class: u32, sub_class: Option<u32>) -> Result<Vec<types::Item>>
{
    let filter = ItemFilter::ByClass(item_class, sub_class).get_filter();
    get_items(conn, Some(filter))
}

/// no filter, simply SELECT * without condition
#[allow(dead_code)]
pub fn all(conn: &mut mysql::PooledConn) -> Result<Vec<types::Item>>
{
    get_items(conn, None)
}

/// generic sql request to Item list
fn get_items(conn: &mut mysql::PooledConn, filter: Option<String>) -> Result<Vec<types::Item>>
{
    let query = match filter {
        Some(filter) => {
            format!("SELECT * FROM Item WHERE {}",  filter)
        },
        None => {
            String::from("SELECT * FROM Item")
        }
    };
    conn.query_map(query, |(id, name, level, is_equippable, item_class, item_sub_class, inv_type)| {
        types::Item {
            id: id,
            name: name,
            level: level,
            is_equippable: is_equippable,
            item_class: item_class,
            item_sub_class: item_sub_class,
            inv_type: inv_type,
        }
    })
}