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

mod notify;
mod paramerror;

use crate::notify::{notify, Icon};
use crate::paramerror::ParamError;
use evdev::{Device, InputEventKind, SwitchType};
use std::{env, error::Error, process::Command};
use ParamError::*;

#[cfg(any(
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "linux",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "openbsd",
))]
fn main() -> Result<(), Box<dyn Error>> {
    let mut args = env::args();
    let _ = args.next();
    let file = args.next().ok_or(WrongBlock)?;
    let command = args.next().ok_or(WrongBlock)?;
    let params: Vec<String> = args.collect();
    let mut device = Device::open(file)?;
    let mut state = 0;

    loop {
        let events = device
            .fetch_events()?
            .filter(|event| event.kind() == InputEventKind::Switch(SwitchType::SW_LID));
        for event in events {
            let value = event.value();
            if value == 1 && state == 0 {
                let mut cmd = Command::new(&command);
                for param in &params {
                    cmd.arg(param);
                }
                match cmd.spawn() {
                    Ok(_) => continue,
                    Err(err) => {
                        eprintln!("{:#?}", err);
                        notify(format!("{:?}", err), Icon::Info);
                    }
                }
            } else {
                state = value;
            }
        }
    }
}
