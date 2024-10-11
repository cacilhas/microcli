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
//! ```sh
//! Usage: trim [OPTIONS] [FILE]
//!
//! Arguments:
//!   [FILE]  input file, defaults to STDIN
//!
//! Options:
//!   -l, --left         trim only leading
//!   -r, --right        trim only trailing
//!   -c, --char <CHAR>  character to be removed, defaults to whitespaces
//!   -h, --help         Print help
//! ```
//!
//! ## Examples
//!
//! ```sh
//! echo '   hello world   ' | trim
//! ```


use eyre::Result;


pub fn trim(input: impl ToString, trim_left: bool, trim_right: bool, c: Option<char>) -> Result<String> {
    let mut output = input.to_string();
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_both() -> Result<()> {
        let output = trim("  hello world   ", true, true, None)?;
        assert_eq!("hello world", output);
        Ok(())
    }

    #[test]
    fn trim_left() -> Result<()> {
        let output = trim("  hello world   ", true, false, None)?;
        assert_eq!("hello world   ", output);
        Ok(())
    }

    #[test]
    fn trim_right() -> Result<()> {
        let output = trim("  hello world   ", false, true, None)?;
        assert_eq!("  hello world", output);
        Ok(())
    }

    #[test]
    fn trim_char_both() -> Result<()> {
        let output = trim("!! hello world !!!", true, true, Some('!'))?;
        assert_eq!(" hello world ", output);
        Ok(())
    }

    #[test]
    fn trim_char_left() -> Result<()> {
        let output = trim("!! hello world !!!", true, false, Some('!'))?;
        assert_eq!(" hello world !!!", output);
        Ok(())
    }

    #[test]
    fn trim_char_right() -> Result<()> {
        let output = trim("!! hello world !!!", false, true, Some('!'))?;
        assert_eq!("!! hello world ", output);
        Ok(())
    }
}
