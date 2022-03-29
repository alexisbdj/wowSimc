pub mod types;
mod connection;
pub mod items;
use mysql::*;
use mysql::prelude::Queryable;

/// list items class / subClass equippable for a given class /spec  
/// if spec is None, returns only items equippable by all specs of the given class
/// 
/// example : leather gloves are equippables by all druids and will be returned by this function  
/// but not intelligence staves
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

/// returns sql connection defined in environment variables
/// 
/// - DB_HOST = sql server hostname
/// - DB_NAME = sql server's database name
/// - DB_USERNAME = username of a user allowed to read the given database
/// - DB_PASSWORD = password of this user
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