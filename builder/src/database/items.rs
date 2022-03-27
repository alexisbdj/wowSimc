use mysql::*;
use mysql::prelude::Queryable;
use crate::database::types;

pub enum ItemFilter {
    ByClass(u32, Option<u32>),
    ByClassList(Vec<types::ItemClass>),
    ByInvType(String),
}

impl ItemFilter {
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

#[allow(dead_code)]
pub fn by_class_list(conn: &mut mysql::PooledConn, class_list: &Vec<types::ItemClass>) -> Result<Vec<types::Item>>
{
    let filter = ItemFilter::ByClassList(class_list.to_vec()).get_filter();
    get_items(conn, Some(filter))
}

#[allow(dead_code)]
pub fn by_class(conn: &mut mysql::PooledConn, item_class: u32, sub_class: Option<u32>) -> Result<Vec<types::Item>>
{
    let filter = ItemFilter::ByClass(item_class, sub_class).get_filter();
    get_items(conn, Some(filter))
}

#[allow(dead_code)]
pub fn all(conn: &mut mysql::PooledConn) -> Result<Vec<types::Item>>
{
    get_items(conn, None)
}

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