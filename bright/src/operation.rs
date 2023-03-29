use std::{
    error,
    fmt,
    fs,
    path::{Display, Path},
};


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
                _   => Err(ParamError::InvalidParameter(args[1].clone())),
            },
            _ => Err(ParamError::TooManyArgs),
        }
    }

    pub fn apply(&self, backlight: &Display) -> Result<(), Box<dyn error::Error>> {
        let dir = backlight.to_string();

        let file = Path::new(&dir).join("brightness");
        let current = file_to_int(&file)?;

        let max_file = Path::new(&dir).join("max_brightness");
        let max = file_to_int(&max_file)
            .or_else(move |_| -> FileToIntResult { Ok(i16::MAX) })?;

        let step = max / 10;

        match self {
            Operation::SHOW => println!("{current}"),

            Operation::INC => {
                let desired = *vec![current + step, max]
                    .iter()
                    .min()
                    .unwrap()
                    as u8;
                let raw = format!("{desired}\n");
                fs::write(&file, raw)?;
                println!("{desired}");
            },

            Operation::DEC => {
                let desired = *vec![current - step, 0_i16]
                    .iter()
                    .max()
                    .unwrap()
                    as u8;
                let raw = format!("{desired}\n");
                fs::write(&file, raw)?;
                println!("{desired}");
            },
        };

        Ok(())
    }
}


type FileToIntResult = Result<i16, Box<dyn error::Error>>;

fn file_to_int(file: &Path) -> FileToIntResult {
    let content = fs::read_to_string(file)?;
    let res = content.trim().parse::<u16>()?; // avoid negative numbers
    Ok(res as i16)
}


#[derive(Debug, PartialEq)]
pub enum ParamError {
    InvalidParameter(String),
    TooManyArgs,
}

impl fmt::Display for ParamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParamError::InvalidParameter(name) => write!(f, "invalid parameter {}", name),
            ParamError::TooManyArgs => write!(f, "too many parameters"),
        }
    }
}

impl error::Error for ParamError {}
