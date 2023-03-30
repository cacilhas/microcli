use std::{
    env,
    error,
    fmt,
    process::Command,
};
use evdev::{Device, InputEventKind, SwitchType};

use ParamError::*;


#[cfg(any(
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "linux",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "openbsd",
))]
fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        WrongBlock.throw()?;
    }
    let file: &String = &args[1];
    let command: &String = &args[2];
    let params = &args[3..];
    let mut _device = Device::open(file)?;
    let mut state = 0_i32;

    loop {
        let events = _device
            .fetch_events()?
            .filter(|event| event.kind() == InputEventKind::Switch(SwitchType::SW_LID));
        for event in events {
            let value = event.value();
            if value == 1 && state == 0 {
                let mut cmd = Command::new(command);
                for param in params {
                    cmd.arg(param);
                }
                match cmd.spawn() {
                    Ok(_)  => continue,
                    Err(err) => eprintln!("{err:#?}"),
                }
            } else {
                state = value;
            }
        }
    }
}


#[derive(Debug)]
enum ParamError {
    WrongBlock
}

impl fmt::Display for ParamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongBlock => write!(f, "expected lid event block"),
        }
    }
}

impl error::Error for ParamError {
}

impl ParamError {
    fn throw(self) -> Result<(), ParamError> {
        Err(self)
    }
}
