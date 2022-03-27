pub mod types;
mod connection;
pub mod items;
use mysql::*;
use mysql::prelude::Queryable;

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