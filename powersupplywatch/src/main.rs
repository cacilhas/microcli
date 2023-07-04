use std::{fmt::Display, fs, path::Path};

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
                        dbg!(content);
                        current = content;
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

#[derive(Debug, StructOpt)]
#[structopt(name = "powersupplywatch")]
struct Params {
    #[structopt(
        short,
        long = "power-supply",
        default_value = "/sys/class/power_supply/AC0/online"
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
        if !Path::new(&params.power_supply).exists() {
            return Err(Error::NotFound(params.power_supply).into());
        }
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
