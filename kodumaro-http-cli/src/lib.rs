#![feature(const_refs_to_static)]

mod cli;

use std::{
    fs,
    io::{self, IsTerminal},
};

pub use cli::*;
use crossterm::style::{
    Attribute,
    Color,
    Print,
    ResetColor,
    SetAttribute,
    SetForegroundColor,
};
use eyre::Result;
use json_color::Colorizer;
use reqwest::{redirect::Policy, Request, RequestBuilder};
use serde_json::Value;
use syntect::{
    easy::HighlightLines,
    highlighting::{Style, ThemeSet},
    parsing::SyntaxSet,
    util::{as_24_bit_terminal_escaped, LinesWithEndings},
};


pub async fn perform(cli: impl CLParameters) -> Result<()> {
    let request: Request = cli.request()?;
    let payload = match cli.payload() {
        Ok(payload) => Some(payload),
        Err(None) => None,
        Err(Some(err)) => return Err(err),
    };

    let policy: Policy = cli.policy();
    let client = reqwest::Client::builder()
        .redirect(policy)
        .build()?;

    let mut builder = RequestBuilder::from_parts(client.clone(), request);
    builder = match payload.clone() {
        Some(Value::String(payload)) => builder
            .header(reqwest::header::CONTENT_LENGTH, payload.len())
            .body(payload),
        Some(payload) => builder
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .header(reqwest::header::CONTENT_LENGTH, serde_json::to_string(&payload)?.len())
            .json(&payload),
        None => builder,
    };

    let mut stderr = io::stderr();

    if cli.verbose() {
        let request = builder.build()?;

        crossterm::execute!(
            stderr,
            SetForegroundColor(Color::Blue),
            SetAttribute(Attribute::Bold),
            Print(request.method()),
            ResetColor,
            Print(" "),
            SetForegroundColor(Color::Yellow),
            Print(request.url().to_string()),
            ResetColor,
            Print("\n"),
        )?;
        for (name, value) in request.headers().iter() {
            let value = value.to_str()?;
            crossterm::execute!(
                stderr,
                SetAttribute(Attribute::Bold),
                Print(name),
                Print(": "),
                ResetColor,
                SetForegroundColor(Color::Yellow),
                Print(value),
                ResetColor,
                Print("\n"),
            )?;
        }
        if let Some(payload) = payload {
            eprintln!("{}", serde_json::to_string(&payload)?);
        }
        eprintln!();

        builder = RequestBuilder::from_parts(client, request);
    }

    let response = builder.send().await?;

    if cli.verbose() {
        let width = match crossterm::terminal::size() {
            Ok((width, _)) => width,
            Err(_) => 80u16,
        };
        let line = "─".repeat(width as usize);
        crossterm::execute!(
            stderr,
            SetForegroundColor(Color::Black),
            Print(line),
            ResetColor,
        )?;

        let status = response.status();
        match status.as_u16() / 100 {
            2 => crossterm::execute!(
                stderr,
                SetForegroundColor(Color::Green),
                SetAttribute(Attribute::Bold),
                Print(status),
                ResetColor,
                Print("\n"),
            )?,
            1|3 => crossterm::execute!(
                stderr,
                SetForegroundColor(Color::Yellow),
                SetAttribute(Attribute::Bold),
                Print(status),
                ResetColor,
                Print("\n"),
            )?,
            _ => crossterm::execute!(
                stderr,
                SetForegroundColor(Color::Red),
                SetAttribute(Attribute::Bold),
                Print(status),
                ResetColor,
                Print("\n"),
            )?,
        }
        for (name, value) in response.headers().iter() {
            let value = value.to_str()?;
            crossterm::execute!(
                stderr,
                SetAttribute(Attribute::Bold),
                Print(name),
                Print(": "),
                ResetColor,
                SetForegroundColor(Color::Red),
                Print(value),
                ResetColor,
                Print("\n"),
            )?;
        }
    }

    let content_type = response.headers()
        .get(reqwest::header::CONTENT_TYPE)
        .map(|value| value.to_str().unwrap_or_default())
        .unwrap_or("text/html");

    eprintln!();

    match cli.output() {
        Some(file) => fs::write(file, response.text().await?)?,

        None => {
            if content_type.contains("json") {
                if let Ok(body) = response.json::<Value>().await {

                    if io::stdout().is_terminal() {
                        let colorizer = Colorizer::arbitrary();
                        match colorizer.colorize_json_str(&serde_json::to_string(&body)?) {
                            Ok(body) => println!("{}", body),
                            Err(_) => print!("{}", body),
                        }
                    } else {
                        print!("{}", body);
                    }
                }

            } else {
                if let Ok(body) = response.text().await {
                    if io::stdout().is_terminal() {
                        let ps = SyntaxSet::load_defaults_newlines();
                        let ts = ThemeSet::load_defaults();
                        let syntax = match ps.find_syntax_by_extension("html") {
                            Some(syntax) => syntax,
                            None => {
                                eprintln!("failed to find HTML syntax");
                                println!("{}", body);
                                return Ok(());
                            }
                        };
                        let mut h = HighlightLines::new(syntax, &ts.themes["Solarized (dark)"]);
                        for line in LinesWithEndings::from(&body) {
                            let ranges: Vec<(Style, &str)> =
                                h.highlight_line(line, &ps).unwrap();
                            let escaped = as_24_bit_terminal_escaped(&ranges[..], false);
                            print!("{}", escaped);
                        }
                    } else {
                        println!("{}", body);
                    }
                }
            }
        }
    }

    Ok(())
}
