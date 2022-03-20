pub fn get_key_value<'a>(line: &'a String) -> Result<(&'a str, &'a str), &'static str> {
    if let Some(delim_index) = line.chars().position(|c| c == '=') {
        Ok((&line[..delim_index], &line[delim_index + 1..]))
    }
    else {
        Err("no delim")
    }
}