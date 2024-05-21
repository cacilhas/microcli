use std::{error, fmt};

#[derive(Debug, PartialEq)]
pub enum ParamError {
    InvalidParameter(String),
}

impl fmt::Display for ParamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParamError::InvalidParameter(name) => write!(f, "invalid parameter {}", name),
        }
    }
}

impl error::Error for ParamError {}
