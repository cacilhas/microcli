#![feature(const_refs_to_static)]

mod cli;
mod output_format;

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
use eyre::{eyre, Result};
use output_format::{add_ext, format_by_ext};
use reqwest::{redirect::Policy, Request, RequestBuilder};
use serde_json::Value;


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
    let status = response.status();

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
    eprintln!();

    if cli.fail() {
        let code = status.as_u16();
        if code >= 400 && code <= 599 {
            return Err(eyre!("{}", status));
        }
    }

    let content_type = response.headers()
        .get(reqwest::header::CONTENT_TYPE)
        .map(|value| value.to_str().unwrap_or_default())
        .unwrap_or("text/plain")
        .to_string();

    match cli.output() {
        Some(file) => fs::write(file, response.text().await?)?,

        None => {
            if let Ok(body) = response.text().await {
                let filename = cli.url().path().to_lowercase();

                if io::stdout().is_terminal() {
                    let filename = add_ext(&filename, &content_type);
                    format_by_ext(&body, &filename)?;

                } else {
                    println!("{}", body);
                }
            }
        }
    }

    Ok(())
}
