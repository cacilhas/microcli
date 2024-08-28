//! Kodumaro Trim intends to be a shell in-pipe tool.
//!
//! # Installation guide
//!
//! ```sh
//! cargo install kodumaro-trim
//! ```
//!
//! # Usage
//!
//! Usage: ktrim [OPTIONS] [FILE]
//!
//! Arguments:
//!   [FILE]  input file, defaults to STDIN
//!
//! Options:
//!   -l, --left         trim only leading
//!   -r, --right        trim only trailing
//!   -c, --char <CHAR>  character to remove, defaults to whitespaces
//!   -h, --help         Print help
//!
//! ## Examples
//!
//! ```sh
//! echo '   hello world   ' | ktrim
//! ```


use std::{fs::File, io::{self, BufRead, BufReader}};

use clap::{ArgAction, Parser};
use eyre::Result;
use kodumaro_trim::trim;


#[derive(Debug, Parser)]
struct Arguments {
    #[arg(short, long, action = ArgAction::SetTrue, help = "trim only leading")]
    left: bool,
    #[arg(short, long, action = ArgAction::SetTrue, help = "trim only trailing")]
    right: bool,
    #[arg(short, long, help = "character to remove, defaults to whitespaces")]
    char: Option<char>,
    #[arg(help = "input file, defaults to STDIN")]
    file: Option<String>,
}

pub fn main() -> Result<()> {
    let parameters = Arguments::parse().init();

    match parameters.file {
        Some(file) => {
            let file = &file;
            let file = File::open(file)?;
            let reader = BufReader::new(file);
            for line in reader.lines() {
                let current = trim(line?, parameters.left, parameters.right, parameters.char)?;
                if !current.is_empty() {
                    println!("{}", current);
                }
            }
        }
        None => {
            let stdin = io::stdin();
            let reader = stdin.lock();
            for line in reader.lines() {
                let current = trim(line?, parameters.left, parameters.right, parameters.char)?;
                if !current.is_empty() {
                    println!("{}", current);
                }
            }
        }
    }

    Ok(())
}

impl Arguments {
    fn init(mut self) -> Self {
        if !(self.left || self.right) {
            self.left = true;
            self.right = true;
        }
        self
    }
}
