use std::{
    env::Args,
    error::Error,
    fs,
    path::{Display, Path},
};

use crate::{minmaxerror::MinMaxError, paramerror::ParamError};

#[derive(Debug, Default, PartialEq)]
pub enum Operation {
    INC,
    DEC,
    #[default]
    SHOW,
}

impl Operation {
    pub fn from_args(args: &mut Args) -> Result<Operation, ParamError> {
        let _ = args.next(); // discard executable name
        match args.next() {
            None => Ok(Operation::default()),
            Some(value) => match value.as_str() {
                "+" => Ok(Operation::INC),
                "-" => Ok(Operation::DEC),
                val => Err(ParamError::InvalidParameter(val.to_owned())),
            },
        }
    }

    pub fn apply(&self, backlight: &Display) -> Result<i16, Box<dyn Error>> {
        let dir = backlight.to_string();

        let file = Path::new(&dir).join("brightness");
        let current = file_to_int(&file)?;

        let max_file = Path::new(&dir).join("max_brightness");
        let max = file_to_int(&max_file).unwrap_or(i16::MAX);
        let min: i16 = 0;

        let step = max / 10;

        let res = match self {
            Operation::SHOW => current,

            Operation::INC => {
                let desired = *[current + step, max].iter().min().ok_or(MinMaxError)?;
                let raw = format!("{desired}\n");
                fs::write(&file, raw)?;
                desired
            }

            Operation::DEC => {
                let desired = *[current - step, min].iter().max().ok_or(MinMaxError)?;
                let raw = format!("{desired}\n");
                fs::write(&file, raw)?;
                desired
            }
        };

        Ok(res)
    }
}

fn file_to_int(file: &Path) -> Result<i16, Box<dyn Error>> {
    let content = fs::read_to_string(file)?;
    let res = content.trim().parse::<u16>()?; // avoid negative numbers
    Ok(res as i16)
}
