use crate::simc;
use crate::database;
use crate::simc::scripter;

use crate::database::items::ItemFilter;

pub fn run(mut conn: &mut mysql::PooledConn) -> std::io::Result<()>
{
    let file = std::fs::File::open("./example.simc")?;
    let character = simc::parser::parse_from_file(file);

    if let Ok(result) = database::get_equippable_items(&mut conn, String::from(character.class.get_name()), Some(character.spec)) {
        match database::items::by_filter_list(&mut conn, &vec!(ItemFilter::ByClassList(result), ItemFilter::ByInvType(String::from("FINGER")))) {
            Ok(result) => {
                scripter::fast_droptimizer(result, &character.name);
            },
            Err(error) => {
                eprintln!("{}", error);
            }
        }
    }
    Ok(())
}