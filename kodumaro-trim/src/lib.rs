use eyre::Result;


pub fn trim(input: impl Into<String>, trim_left: bool, trim_right: bool, c: Option<char>) -> Result<String> {
    let mut output = input.into();
    if trim_left {
        output = ltrim(output, c)?;
    }
    if trim_right {
        output = rtrim(output, c)?;
    }
    Ok(output)
}

fn ltrim(input: String, c: Option<char>) -> Result<String> {
    match c {
        Some(c) => Ok(input.trim_start_matches(c).to_string()),
        None => Ok(input.trim_start().to_string()),
    }
}

fn rtrim(input: String, c: Option<char>) -> Result<String> {
    match c {
        Some(c) => Ok(input.trim_end_matches(c).to_string()),
        None => Ok(input.trim_end().to_string()),
    }
}
