#![feature(const_refs_to_static)]

mod cli;
mod output_format;
mod styles;

use std::{
    fs,
    io::{self, IsTerminal, Write},
};

pub use cli::*;
use crossterm::style::{
    Color,
    Print,
    ResetColor,
    SetForegroundColor,
    SetStyle,
};
use eyre::{eyre, Result};
use output_format::{add_ext, format_by_ext};
use reqwest::{redirect::Policy, Request, RequestBuilder};
use serde_json::Value;
use styles::*;


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

    let builder = RequestBuilder::from_parts(client.clone(), request);
    let builder = match payload.clone() {
        Some(Value::String(payload)) => builder
            .header(reqwest::header::CONTENT_LENGTH, payload.len())
            .body(payload),
        Some(payload) => builder
            .header(reqwest::header::CONTENT_TYPE, "application/json")
            .header(reqwest::header::CONTENT_LENGTH, serde_json::to_string(&payload)?.len())
            .json(&payload),
        None => builder,
    };

    let mut stdout = io::stdout();
    let mut stderr = io::stderr();

    if cli.verbose() {
        if let Some(builder) = builder.try_clone() {
            let request = builder.build()?;

            crossterm::execute!(
                stderr,
                SetStyle(*METHOD_STYLE),
                Print(request.method()),
                Print(" "),
                SetStyle(*DEFAULT_STYLE),
                SetStyle(*URL_STYLE),
                Print(request.url().to_string()),
                SetStyle(*DEFAULT_STYLE),
                Print("\n"),
            )?;
            for (name, value) in request.headers().iter() {
                let value = value.to_str()?;
                crossterm::execute!(
                    stderr,
                    SetStyle(*HEADER_NAME_STYLE),
                    Print(name),
                    Print(": "),
                    SetStyle(*DEFAULT_STYLE),
                    SetStyle(*HEADER_VALUE_STYLE),
                    Print(value),
                    SetStyle(*DEFAULT_STYLE),
                    Print("\n"),
                )?;
            }
            if let Some(payload) = payload {
                eprintln!();
                format_by_ext(&serde_json::to_string(&payload)?, ".json", &mut stderr)?;
                eprintln!();
            }
            eprintln!();
        }
    }

    let response = builder.send().await?;
    let status = response.status();

    if cli.verbose() {
        draw_line(&mut stderr)?;

        match status.as_u16() / 100 {
            2 => crossterm::execute!(
                stderr,
                SetStyle(*STATUS_SUCCESS_STYLE),
                Print(status),
                SetStyle(*DEFAULT_STYLE),
                Print("\n"),
            )?,
            4|5 => crossterm::execute!(
                stderr,
                SetStyle(*STATUS_FAILURE_STYLE),
                Print(status),
                SetStyle(*DEFAULT_STYLE),
                Print("\n"),
            )?,
            _ => crossterm::execute!(
                stderr,
                SetStyle(*STATUS_OTHER_STYLE),
                Print(status),
                SetStyle(*DEFAULT_STYLE),
                Print("\n"),
            )?,
        }
        for (name, value) in response.headers().iter() {
            let value = value.to_str()?;
            crossterm::execute!(
                stderr,
                SetStyle(*HEADER_NAME_STYLE),
                Print(name),
                Print(": "),
                SetStyle(*DEFAULT_STYLE),
                SetStyle(*HEADER_VALUE_STYLE),
                Print(value),
                SetStyle(*DEFAULT_STYLE),
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
                    format_by_ext(&body, &filename, &mut stdout)?;

                } else {
                    println!("{}", body);
                }
            }
        }
    }

    Ok(())
}


fn draw_line(writer: &mut impl Write) -> Result<()> {
    let width = match crossterm::terminal::size() {
        Ok((width, _)) => width,
        Err(_) => 80u16,
    };
    let line = "─".repeat(width as usize);
    crossterm::execute!(
        writer,
        SetForegroundColor(Color::Black),
        Print(line),
        ResetColor,
    )?;
    Ok(())
}
