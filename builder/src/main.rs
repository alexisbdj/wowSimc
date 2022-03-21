mod simc;
mod general_parser;
mod database;

fn main() -> std::io::Result<()> {
    let file = std::fs::File::open("./example.simc")?;
    simc::parser::parse_from_file(file).dump();

    if let Ok(result) = dotenv::dotenv() {
        println!("{}", result.to_str().unwrap());
    }
    else {
        eprintln!("env not found");
    }

    match database::connect_to_database() {
        Ok(mut conn) => {
            if let Ok(result) = database::get_all_items(&mut conn) {
                for item in result {
                    println!("{}", item);
                }
            }
        },
        Err(err) => {
            eprintln!("{}", err);
        }
    }
    Ok(())
}