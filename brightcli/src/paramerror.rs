use std::{error, fmt};

#[derive(Debug, PartialEq)]
pub enum ParamError {
    InvalidParameter(String),
    TooManyArgs,
}

impl fmt::Display for ParamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParamError::InvalidParameter(name) => write!(f, "invalid parameter {name}"),
            ParamError::TooManyArgs => write!(f, "too many parameters"),
        }
    }
}

impl error::Error for ParamError {}
