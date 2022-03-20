mod simc;
mod general_parser;

fn main() -> std::io::Result<()> {
    let file = std::fs::File::open("./example.simc")?;
    simc::parser::parse_from_file(file).dump();
    Ok(())
}