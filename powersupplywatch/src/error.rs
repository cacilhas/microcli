use crate::result::Result;
use std::{error, fmt::Display};

#[derive(Debug)]
pub enum Error {
    NotFound(String),
    PathParsingError,
}

impl Error {
    pub fn not_found(value: impl Into<String>) -> Result<()> {
        Err(Self::NotFound(value.into()).into())
    }
}

impl error::Error for Error {}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(msg) => write!(f, "{} not found", msg),
            Self::PathParsingError => f.write_str("path parsing error"),
        }
    }
}
