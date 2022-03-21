mod types;
mod connection;
use mysql::*;
use mysql::prelude::Queryable;

pub fn get_items_by_class_list(conn: &mut mysql::PooledConn, class_list: &Vec<types::ItemClass>) -> Result<Vec<types::Item>>
{
    let mut filter = String::new();
    
    for (index, el) in class_list.iter().enumerate() {
        filter.push_str(&format!("{} (itemClass = {} AND itemSubClass = {})", 
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
    get_items(conn, Some(filter))
}

pub fn get_items_by_class(conn: &mut mysql::PooledConn, item_class: u32, sub_class: Option<u32>) -> Result<Vec<types::Item>>
{
    let mut filter = String::from("itemClass = {}");
    if let Some(sub_class) = sub_class {
        filter.push_str(&format!(" AND itemSubClass = {}", sub_class));
    }
    get_items(conn, Some(format!("itemClass = {}", item_class)))
}

#[allow(dead_code)]
pub fn get_all_items(conn: &mut mysql::PooledConn) -> Result<Vec<types::Item>>
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

pub fn get_equippable_items(conn: &mut mysql::PooledConn, class: String, spec: Option<String>) -> Result<Vec<types::ItemClass>>
{
    let query = format!("SELECT itemClass, itemSubClass FROM ClassEquipment WHERE class = '{}' AND (spec IS NULL{})", class,
        match spec {
            Some(spec_name) => {
                format!(" OR spec = '{}'", spec_name)
            },
            None => {
                String::new()
            }
        }
    );
    conn.query_map(query, |(item_class, item_sub_class)| {
        types::ItemClass {
            class: item_class,
            sub_class: item_sub_class,
        }
    })
}

pub fn connect_to_database() -> std::result::Result<mysql::PooledConn, String> {
    match connection::get_conn_builder() {
        Ok(builder) => {
            match Pool::new(builder) {
                Ok(pool) => {
                    match pool.get_conn() {
                        Ok(conn) => {
                            Ok(conn)
                        },
                        Err(err) => {
                            eprintln!("{}", err);
                            Err(err.to_string())
                        }
                    }
                },
                Err(err) => {
                    Err(err.to_string())
                }
                
            }
        },
        Err(error) => {
            eprintln!("{}", error);
            Err(error.to_string())
        }
    }
}