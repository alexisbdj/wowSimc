use crate::simc;
use crate::database;

pub fn run(mut conn: &mut mysql::PooledConn) -> std::io::Result<()>
{
    let file = std::fs::File::open("./example.simc")?;
    let character = simc::parser::parse_from_file(file);

    character.dump();
    if let Ok(result) = database::get_equippable_items(&mut conn, String::from(character.class.get_name()), Some(character.spec)) {
        println!("can equip :");
        for item in &result {
            println!("{}", item);
        }
        match database::get_items_by_class_list(&mut conn, &result) {
            Ok(result) => {
                for item in result {
                    println!("{}", item);
                }
            },
            Err(error) => {
                eprintln!("{}", error);
            }
        }
    }
    Ok(())
}