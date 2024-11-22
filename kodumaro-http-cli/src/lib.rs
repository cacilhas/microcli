#![feature(const_refs_to_static)]

mod cli;
mod output_format;
mod styles;

use std::{fs::File, io::{self, IsTerminal, Stdout, Write}};

pub use cli::*;
use crossterm::style::{
    Color,
    Print,
    ResetColor,
    SetForegroundColor,
    SetStyle,
};
use eyre::{eyre, Result};
use futures::StreamExt;
use indicatif::{ProgressBar, ProgressState, ProgressStyle};
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
    let total_size: u64 = response.content_length().unwrap_or(0);
    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})"
        )?.with_key(
            "eta",
            |state: &ProgressState, w: &mut dyn ::std::fmt::Write|
                write!(w, "{:.1}s", state.eta().as_secs_f64()).unwrap()
        )
    );

    let content_type = response.headers()
        .get(reqwest::header::CONTENT_TYPE)
        .map(|value| value.to_str().unwrap_or_default())
        .unwrap_or("text/plain")
        .to_string();

    let mut out: Box<dyn Write> = match cli.output() {
        Some(file) => Box::new(File::create(file)?),
        None => Box::new(Buffer::new(
            &mut stdout,
            cli.url().path().to_lowercase(),
            content_type,
        )),
    };

    let mut downloaded: u64 = 0;
    let mut stream = response.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item?;
        let chunk = chunk.into_iter().collect::<Vec<u8>>();
        write!(out, "{}", String::from_utf8_lossy(&chunk))?;
        downloaded = total_size.min(downloaded + chunk.len() as u64);
        pb.set_position(downloaded);
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

#[derive(Debug)]
struct Buffer<'a> {
    buf: String,
    stdout: &'a mut Stdout,
    is_terminal: bool,
    filename: String,
    content_type: String,
}

impl<'a> Buffer<'a> {

    fn new(
        stdout: &'a mut Stdout,
        filename: impl ToString,
        content_type: impl ToString,
    ) -> Self {
        let is_terminal = stdout.is_terminal();
        Self {
            buf: String::new(),
            stdout, is_terminal,
            filename: filename.to_string(),
            content_type: content_type.to_string(),
        }
    }
}

impl Write for Buffer<'_> {

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }

    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let size = buf.len();
        let chunk = String::from_utf8_lossy(buf);
        if self.is_terminal {
            self.buf += &chunk;
        } else {
            write!(self.stdout, "{}", chunk).unwrap();
        }
        Ok(size)
    }
}

impl Drop for Buffer<'_> {

    fn drop(&mut self) {
        if self.is_terminal {
            let filename = add_ext(&self.filename, &self.content_type);
            format_by_ext(&self.buf, &filename, self.stdout).unwrap();
        }
    }
}
