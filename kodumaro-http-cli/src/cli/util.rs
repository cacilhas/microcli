use std::fs::read_to_string;

use eyre::Result;

pub fn parse_string(value: impl ToString) -> Result<String> {
    let value = value.to_string();
    if let Some(value) = value.strip_prefix('@') {
        Ok(read_to_string(value)?.trim().to_owned())
    } else {
        Ok(value)
    }
}
