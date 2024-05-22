use std::{error::Error, fmt};

#[derive(Debug)]
pub enum ParamError {
    WrongBlock,
}

impl fmt::Display for ParamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongBlock => f.write_str("expected lid event block"),
        }
    }
}

impl Error for ParamError {}
