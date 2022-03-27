pub fn get_key_value<'a>(line: &'a String) -> Result<(&'a str, &'a str), &'static str> {
    if let Some(delim_index) = line.chars().position(|c| c == '=') {
        Ok((&line[..delim_index], &line[delim_index + 1..]))
    }
    else {
        Err("no delim")
    }
}

pub fn trim_quotes<'a>(value: &'a str) -> &'a str
{

    if let Some(fc) = value.chars().next() {
        if fc == '"' {
            let nvalue = &value[1..];
            if let Some(end_index) = nvalue.chars().position(|c| c == '"') {
                return &nvalue[..end_index];
            }
        }
    }
    value
}