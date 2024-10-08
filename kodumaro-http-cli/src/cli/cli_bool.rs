use std::str::FromStr;

use eyre::eyre;


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
        if ["y", "yes", "true", "1"].contains(&value.to_lowercase().as_str()) {
            return Ok(Self::Yes);
        }
        if ["n", "no", "false", "0", ""].contains(&value.to_lowercase().as_str()) {
            return Ok(Self::No);
        }

        Err(eyre!("could not parse {}", value))
    }
}

impl ToString for CliBool {

    fn to_string(&self) -> String {
        format!("{:?}", self).to_lowercase()
    }
}
