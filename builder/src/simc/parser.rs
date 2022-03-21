use std::io::{BufRead};
use crate::simc::character_info::*;

pub fn parse_from_file(file: std::fs::File) -> CharacterInfo {
    let lines = std::io::BufReader::new(file).lines();
    let mut result = CharacterInfo::new();

    for line in lines {
        if let Ok(line) = line {
            result.update_from_line(line);
        }
    }
    result
}