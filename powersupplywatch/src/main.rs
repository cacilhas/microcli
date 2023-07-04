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
//! powersupplywatch [--power-supply=</sys/class/power_supply/AC0>] \
//!   [--sounds=</usr/share/sounds/freedesktop/stereo>] \
//!   [--plugin=<power-plug.oga>] \
//!   [--unplug=<power-unplug.oga>]
//! ```

use std::{fmt::Display, fs, path::Path, process::Command};

use inotify::{Inotify, WatchMask};
use structopt::StructOpt;

fn main() -> anyhow::Result<()> {
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
                    }
                }
                Err(err) => eprintln!("error reading {}: {}", params.power_supply, err),
            },
            Err(err) => eprintln!("error reading {}: {}", params.power_supply, err),
        }
    }
}

fn read_content(file: &str) -> anyhow::Result<i32> {
    let content: i32 = fs::read_to_string(file)?.trim().parse()?;
    Ok(content)
}

// TODO: find some decent crate that manages and play any audio file
fn play(file: &str) {
    let _ = Command::new("play").arg(file).output();
}

#[derive(Debug, StructOpt)]
#[structopt(name = "powersupplywatch")]
struct Params {
    #[structopt(
        short,
        long = "power-supply",
        default_value = "/sys/class/power_supply/AC0"
    )]
    power_supply: String,
    #[structopt(short, long, default_value = "/usr/share/sounds/freedesktop/stereo")]
    sounds: String,
    #[structopt(short = "i", long, default_value = "power-plug.oga")]
    plugin: String,
    #[structopt(short, long, default_value = "power-unplug.oga")]
    unplug: String,
}

impl Params {
    fn parse() -> anyhow::Result<Self> {
        let mut params = Params::from_args();
        let mut power_supply = Path::new(&params.power_supply).to_owned();
        if !power_supply.ends_with("/online") {
            power_supply = power_supply.join("online");
        }
        if !power_supply.exists() {
            return Err(Error::NotFound(params.power_supply).into());
        }
        params.power_supply = power_supply.to_str().unwrap().to_owned();
        let sounds = Path::new(&params.sounds);
        let plugin = params.plugin.clone();
        let unplug = params.unplug.clone();
        let mut plugin = Path::new(&plugin).to_owned();
        let mut unplug = Path::new(&unplug).to_owned();
        if plugin.is_relative() {
            plugin = sounds.clone().join(&plugin);
        }
        if unplug.is_relative() {
            unplug = sounds.clone().join(&unplug);
        }
        params.plugin = plugin.to_str().unwrap().to_owned();
        params.unplug = unplug.to_str().unwrap().to_owned();
        if !plugin.exists() {
            return Err(Error::NotFound(params.plugin).into());
        }
        if !unplug.exists() {
            return Err(Error::NotFound(params.unplug).into());
        }
        Ok(params)
    }
}

#[derive(Debug, thiserror::Error)]
enum Error {
    NotFound(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotFound(msg) => write!(f, "{msg} not found"),
        }
    }
}
