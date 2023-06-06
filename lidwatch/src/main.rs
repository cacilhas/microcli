//! Util to watch LID switching.
//!
//! # Build and install
//!
//! ```sh
//! cargo install lidwatch
//! ```
//!
//! # Use
//!
//! ```sh
//! lidwatch /dev/input/<lid event> <command to be executed when LID is closed>
//! ```

use evdev::{Device, InputEventKind, SwitchType};
use std::{env, fmt, process::Command};

use ParamError::*;

#[cfg(any(
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "linux",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "openbsd",
))]
fn main() -> anyhow::Result<()> {
    let args = env::args().collect::<Vec<String>>();
    if args.len() < 3 {
        WrongBlock.throw()?;
    }
    let file = &args[1];
    let command = &args[2];
    let params = &args[3..];
    let mut device = Device::open(file)?;
    let mut state: i32 = 0;

    loop {
        let events = device
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
                    Ok(_) => continue,
                    Err(err) => {
                        eprintln!("{err:#?}");
                        notify(&format!("{err:?}"));
                    }
                }
            } else {
                state = value;
            }
        }
    }
}

#[derive(Debug, thiserror::Error)]
enum ParamError {
    WrongBlock,
}

impl fmt::Display for ParamError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongBlock => write!(f, "expected lid event block"),
        }
    }
}

impl ParamError {
    fn throw(self) -> Result<(), ParamError> {
        Err(self)
    }
}

fn notify(msg: &str) {
    let mut cmd = Command::new("notify-send");
    cmd.args(["--app-name=lidwatch", "-t", "5000", &format!("'{msg}'")]);
    match cmd.spawn() {
        Ok(_) => (),
        Err(err) => eprintln!("{err:#?}"),
    }
}
