//! Util to watch power supply.
//!
//! # Build and install
//!
//! ```sh
//! cargo install powersupplywatch
//! ```
//!
//! # Use
//!
//! ```sh
//! powersupplywatch --help
//! ```

mod error;
mod params;
mod result;

use crate::params::Params;
use crate::result::Result;
use inotify::{Inotify, WatchMask};
use std::{fs, process::Command, thread::sleep, time::Duration};

fn main() -> Result<()> {
    let params = Params::parse()?;
    let mut notif = Inotify::init()?;
    notif
        .watches()
        .add(&params.power_supply, WatchMask::ACCESS)?;

    let mut buf = [0u8; 4096];
    let mut current = read_content(&params.power_supply)?;
    loop {
        match notif.read_events_blocking(&mut buf) {
            Ok(_) => match read_content(&params.power_supply) {
                Ok(content) => {
                    if content != current {
                        current = content;
                        match content {
                            0 => play(&params.unplug),
                            _ => play(&params.plugin),
                        }
                    } else {
                        sleep(Duration::from_millis(100));
                    }
                }
                Err(err) => eprintln!("error reading {}: {}", params.power_supply, err),
            },
            Err(err) => eprintln!("error reading {}: {}", params.power_supply, err),
        }
    }
}

fn read_content(file: &str) -> Result<i32> {
    let content: i32 = fs::read_to_string(file)?.trim().parse()?;
    Ok(content)
}

// TODO: find some decent crate that manages and play any audio file
fn play(file: &str) {
    let _ = Command::new("play").arg(file).output();
}
