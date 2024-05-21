use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct MinMaxError;

impl Error for MinMaxError {}

impl Display for MinMaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("error extracting integer value")
    }
}
