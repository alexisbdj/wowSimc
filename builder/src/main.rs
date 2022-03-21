mod simc;
mod general_parser;
mod database;
mod process;

fn main() -> std::io::Result<()> {
    if let Ok(_) = dotenv::dotenv() {
        println!("env updated");
    }
    else {
        eprintln!("env not found");
    }

    match database::connect_to_database() {
        Ok(mut conn) => {
            process::run(&mut conn)?
        },
        Err(err) => {
            eprintln!("{}", err);
        }
    }
    Ok(())
}