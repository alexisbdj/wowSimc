mod types;
use mysql::*;
use mysql::prelude::*;

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

pub fn connect_to_database() -> std::result::Result<mysql::PooledConn, &'static str> {
    let mut builder = OptsBuilder::new();
    builder = builder.user(Some("wowsimc"));
    builder = builder.pass(Some("abcd"));
    builder = builder.db_name(Some("WowData"));
    if let Ok(pool) = Pool::new(builder) {
        match pool.get_conn() {
            Ok(conn) => {
                Ok(conn)
            },
            Err(err) => {
                eprintln!("{}", err);
                Err("getconn")
            }
        }

    }
    else {
        Err("poll")
    }
}