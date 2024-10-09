use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum UUIDError {
    Missing(String),
    InvalidVersion(String),
    WrongLength { expected: usize, got: usize },
}

impl Display for UUIDError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UUIDError::Missing(value) => f.write_str(&format!("missing {}", value)),
            UUIDError::InvalidVersion(value) => f.write_str(&format!("invalid UUID version {}", value)),
            UUIDError::WrongLength { expected, got } => f.write_str(&format!("wrong length, expected {}, got {}", expected, got)),
        }
    }
}

impl Error for UUIDError {}
