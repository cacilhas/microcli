use std::{
    error,
    env,
    fmt,
    fs,
    path::{Display, Path},
};

use Operation::*;
use ParamError::*;


#[cfg(target_os = "linux")]
fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();
    let param = get_operation(&args)?;
    let paths = fs::read_dir("/sys/class/backlight")?;

    for path in paths {
        match path {
            Ok(entry) => apply_to_backlight(&entry.path().display(), &param)?,
            Err(_) => {},
        };
    };

    Ok(())
}

fn get_operation(args: &Vec<String>) -> Result<Operation, ParamError> {
    match args.len() {
        1 => Ok(SHOW),
        2 => match &*args[1] {
            "+" => Ok(INC),
            "-" => Ok(DEC),
            other => Err(InvalidParameter(String::from(other))),
        },
        _ => Err(TooManyArgs),
    }
}

fn apply_to_backlight(backlight: &Display, operation: &Operation) -> Result<(), Box<dyn error::Error>> {
    let dir = backlight.to_string();
    let file = Path::new(&dir).join("max_brightness");
    let max = file_to_int(&file)?;
    let file = Path::new(&dir).join("brightness");
    let current = file_to_int(&file)?;
    let step = max / 10;

    match operation {
        SHOW => println!("{:#?}", current),

        INC => {
            let desired = *vec![(current as i16) + (step as i16), max as i16]
                .iter()
                .min()
                .unwrap()
                as u8;
            let raw = format!("{}\n", desired);
            fs::write(&file, raw)?;
            println!("{:#?}", desired);
        },

        DEC => {
            let desired = *vec![(current as i16) - (step as i16), 0_i16]
                .iter()
                .max()
                .unwrap()
                as u8;
            let raw = format!("{}\n", desired);
            fs::write(&file, raw)?;
            println!("{:#?}", desired);
        },
    };

    Ok(())
}

fn file_to_int(file: &Path) -> Result<u8, Box<dyn error::Error>> {
    let content = fs::read_to_string(file)?;
    Ok(content.trim().parse::<u8>()?)
}


#[derive(Debug)]
enum Operation {
    INC,
    DEC,
    SHOW,
}


#[derive(Debug)]
enum ParamError {
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
