use std::fs::read_to_string;

use eyre::Result;

pub fn parse_string(value: impl ToString) -> Result<String> {
    let value = value.to_string();
    if value.starts_with("@") {
        Ok(read_to_string(value[1..].to_string())?.trim().to_owned())
    } else {
        Ok(value)
    }
}
