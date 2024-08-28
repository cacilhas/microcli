//! Kodumaro Trim aims to be a shell in-pipe tool for removing leading and trailing characters from strings.
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
//!   -c, --char &lt;CHAR&gt;  character to be removed, defaults to whitespaces
//!   -h, --help         Print help
//!
//! ## Examples
//!
//! ```sh
//! echo '   hello world   ' | ktrim
//! ```


use eyre::Result;


pub fn trim(input: impl Into<String>, trim_left: bool, trim_right: bool, c: Option<char>) -> Result<String> {
    let mut output = input.into();
    if trim_left {
        output = ltrim(output, c)?;
    }
    if trim_right {
        output = rtrim(output, c)?;
    }
    Ok(output)
}

fn ltrim(input: String, c: Option<char>) -> Result<String> {
    match c {
        Some(c) => Ok(input.trim_start_matches(c).to_string()),
        None => Ok(input.trim_start().to_string()),
    }
}

fn rtrim(input: String, c: Option<char>) -> Result<String> {
    match c {
        Some(c) => Ok(input.trim_end_matches(c).to_string()),
        None => Ok(input.trim_end().to_string()),
    }
}
