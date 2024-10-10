use std::{env, fs, path::PathBuf};

use clap::Parser;
use serde::Deserialize;

use crate::{cli_bool::CliBool, Cli};


#[derive(Debug, Deserialize)]
pub struct Settings {

    auth: Option<String>,
    download: Option<bool>,
    follow: Option<bool>,
    fail: Option<bool>,
    max_redirects: Option<usize>,
    verbose: Option<bool>,
    verify: Option<CliBool>,

    #[serde(skip)]
    helper: Option<SettingsHelper>,
}

#[derive(Clone, Debug, Parser)]
#[command(ignore_errors = true)]
struct SettingsHelper {

    #[arg(short = 'C', long)]
    config: Option<String>,

    #[arg(short, long)]
    auth: Option<String>,

    #[arg(long)]
    max_redirects: Option<usize>,

    #[arg(long)]
    verify: Option<CliBool>,
}

impl Settings {

    pub fn new() -> Option<Self> {
        dbg!("Settings::new");
        Settings::from_file(fs::read_to_string(&config_file()).ok()?)
    }

    fn from_file(filename: impl ToString) -> Option<Self> {
        match SettingsHelper::try_parse() {
            Ok(helper) => {
                dbg!(&helper);
                let filename = helper.clone().config.unwrap_or(filename.to_string());
                let mut settings: Self = toml::de::from_str(&filename).ok()?;
                settings.helper = Some(helper);
                Some(settings)
            }

            Err(err) => {
                dbg!(&err);
                eprintln!("{}", err);
                None
            }
        }
    }

    pub fn process(&self, cli: &mut Cli) {
        if let Some(helper) = &self.helper {
            let args = cli.verb.args_mut();

            if helper.auth.is_none() {
                if let Some(auth) = &self.auth {
                    let _ = args.auth.insert(auth.to_owned());
                }
            }

            if let Some(download) = &self.download {
                args.download = args.download || *download;
            }

            if let Some(follow) = &self.follow {
                args.follow = args.follow || *follow;
            }

            if let Some(fail) = &self.fail {
                args.fail = args.fail || *fail;
            }

            if helper.max_redirects.is_none() {
                if let Some(max_redirects) = &self.max_redirects {
                    args.max_redirects = *max_redirects;
                }
            }

            if let Some(verbose) = &self.verbose {
                args.verbose = args.verbose || *verbose;
            }

            if helper.verify.is_none() {
                if let Some(verify) = &self.verify {
                    args.verify = verify.clone();
                }
            }
        }
    }
}


pub fn config_file() -> PathBuf {
    config_dir().join("kodumaro-http.toml")
}

fn config_dir() -> PathBuf {
    #[cfg(target_os = "windows")]
    let config_dir = env::var("APPDATA");
    #[cfg(not(target_os = "windows"))]
    let config_dir = env::var("XDG_CONFIG_DIR");

    match config_dir {
        Ok(dir) => PathBuf::from(dir),

        #[cfg(target_os = "windows")]
        Err(_) => PathBuf::from(&format!("{}\\AppData\\Local", env!["USERPROFILE"])),

        #[cfg(not(target_os = "windows"))]
        Err(_) => PathBuf::from(&format!("{}/.config", env!["HOME"])),
    }
}
