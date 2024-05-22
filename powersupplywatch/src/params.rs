use crate::error::Error;
use crate::result::Result;
use std::path::Path;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "powersupplywatch")]
pub struct Params {
    #[structopt(
        short,
        long = "power-supply",
        default_value = "/sys/class/power_supply/AC0"
    )]
    pub power_supply: String,
    #[structopt(short, long, default_value = "/usr/share/sounds/freedesktop/stereo")]
    pub sounds: String,
    #[structopt(short = "i", long, default_value = "power-plug.oga")]
    pub plugin: String,
    #[structopt(short, long, default_value = "power-unplug.oga")]
    pub unplug: String,
}

impl Params {
    pub fn parse() -> Result<Self> {
        let mut params = Params::from_args();
        let mut power_supply = Path::new(&params.power_supply).to_owned();
        if !power_supply.ends_with("/online") {
            power_supply = power_supply.join("online");
        }
        if !power_supply.exists() {
            Error::not_found(&params.power_supply)?;
        }
        params.power_supply = power_supply
            .to_str()
            .ok_or(Error::PathParsingError)?
            .to_owned();
        let sounds = Path::new(&params.sounds);
        let plugin = &params.plugin;
        let unplug = &params.unplug;
        let mut plugin = Path::new(plugin).to_owned();
        let mut unplug = Path::new(unplug).to_owned();
        if plugin.is_relative() {
            plugin = sounds.join(plugin);
        }
        if unplug.is_relative() {
            unplug = sounds.join(unplug);
        }
        params.plugin = plugin.to_str().ok_or(Error::PathParsingError)?.to_owned();
        params.unplug = unplug.to_str().ok_or(Error::PathParsingError)?.to_owned();
        if !plugin.exists() {
            Error::not_found(&params.plugin)?;
        }
        if !unplug.exists() {
            Error::not_found(&params.unplug)?;
        }
        Ok(params)
    }
}
