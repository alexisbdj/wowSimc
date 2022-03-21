mod types;
mod connection;
use mysql::*;
use mysql::prelude::Queryable;

pub fn get_all_items(conn: &mut mysql::PooledConn) -> Result<Vec<types::Item>>
{
    conn.query_map("SELECT * FROM Item", |(id, name, level, is_equippable, item_class, item_sub_class, inv_type)| {
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