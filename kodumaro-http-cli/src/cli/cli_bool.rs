use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

use eyre::eyre;

const TRUE_ARRAY: [&str; 4] = ["y", "yes", "true", "1"];
const FALSE_ARRAY: [&str; 5] = ["n", "no", "false", "0", ""];

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum CliBool {
    #[default]
    Yes,
    No,
}

impl From<bool> for CliBool {
    fn from(value: bool) -> Self {
        if value { Self::Yes } else { Self::No }
    }
}

impl From<CliBool> for bool {
    fn from(value: CliBool) -> Self {
        value == CliBool::Yes
    }
}

impl FromStr for CliBool {
    type Err = eyre::Error;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let lowercase = value.to_lowercase();
        if TRUE_ARRAY.contains(&lowercase.as_str()) {
            return Ok(Self::Yes);
        }
        if FALSE_ARRAY.contains(&lowercase.as_str()) {
            return Ok(Self::No);
        }

        Err(eyre!("could not parse {}", value))
    }
}

impl Display for CliBool {

    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Yes => write!(f, "yes"),
            Self::No => write!(f, "no"),
        }
    }
}
