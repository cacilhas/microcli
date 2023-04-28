use std::{
    fs,
    path::{Display, Path},
};

use crate::paramerror::ParamError;

#[derive(Debug, Default, PartialEq)]
pub enum Operation {
    INC,
    DEC,
    #[default]
    SHOW,
}

impl Operation {
    pub fn from_args(args: &Vec<String>) -> Result<Operation, ParamError> {
        match args.len() {
            1 => Ok(Operation::default()),
            2 => match &*args[1] {
                "+" => Ok(Operation::INC),
                "-" => Ok(Operation::DEC),
                _ => Err(ParamError::InvalidParameter(args[1].clone())),
            },
            _ => Err(ParamError::TooManyArgs),
        }
    }

    pub fn apply(&self, backlight: &Display) -> anyhow::Result<i16> {
        let dir = backlight.to_string();

        let file = Path::new(&dir).join("brightness");
        let current = file_to_int(&file)?;

        let max_file = Path::new(&dir).join("max_brightness");
        let max = file_to_int(&max_file).unwrap_or(i16::MAX);

        let step = max / 10;

        let res = match self {
            Operation::SHOW => current,

            Operation::INC => {
                let desired = *vec![current + step, max].iter().min().unwrap();
                let raw = format!("{desired}\n");
                fs::write(&file, raw)?;
                desired
            }

            Operation::DEC => {
                let desired = *vec![current - step, 0_i16].iter().max().unwrap();
                let raw = format!("{desired}\n");
                fs::write(&file, raw)?;
                desired
            }
        };

        Ok(res)
    }
}

type FileToIntResult = anyhow::Result<i16>;

fn file_to_int(file: &Path) -> FileToIntResult {
    let content = fs::read_to_string(file)?;
    let res = content.trim().parse::<u16>()?; // avoid negative numbers
    Ok(res as i16)
}
