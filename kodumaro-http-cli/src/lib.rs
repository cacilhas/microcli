#![feature(const_refs_to_static)]

mod cli;

use std::{fs, io::{self, IsTerminal}, path::Path};

pub use cli::*;
use eyre::Result;
use json_color::Colorizer;
use reqwest::{redirect::Policy, Request, RequestBuilder};
use serde_json::Value;


pub async fn perform(cli: Cli) -> Result<()> {
    let request: Request = (&cli).try_into()?;
    let payload = match cli.payload() {
        Ok(payload) => Some(payload),
        Err(None) => None,
        Err(Some(err)) => return Err(err),
    };

    let output = match cli.output() {
        Some(output) => Some(output),
        None => {
            if cli.download() {
                let path = Path::new(cli.url().path());
                path.file_name().map(|path| path.to_string_lossy().to_string())
            } else {
                None
            }
        }
    };

    let policy: Policy = (&cli).into();
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

    if cli.verbose() {
        let request = builder.build()?;

        eprintln!("\x1b[34;1m{:?} \x1b[33;1m{}\x1b[0m", request.method(), request.url());
        for (name, value) in request.headers().iter() {
            eprintln!("\x1b[1m{}:\x1b[0m \x1b[33m{}\x1b[0m", name, value.to_str()?);
        }
        match payload {
            Some(payload) => eprintln!("{}", serde_json::to_string(&payload)?),
            _ => (),
        }
        eprintln!("");

        builder = RequestBuilder::from_parts(client, request);
    }

    let response = builder.send().await?;

    if cli.verbose() {
        let status = response.status();
        match status.as_u16() / 100 {
            2 => eprintln!("\x1b[32;1m{}\x1b[0m", status),
            1|3 => eprintln!("\x1b[33;1m{}\x1b[0m", status),
            _ => eprintln!("\x1b[31;1m{}\x1b[0m", status),
        }
        for (name, value) in response.headers().iter() {
            eprintln!("\x1b[1m{}:\x1b[0m \x1b[33m{}\x1b[0m", name, value.to_str()?);
        }
    }

    let content_type = response.headers()
        .get(reqwest::header::CONTENT_TYPE)
        .map(|value| value.to_str().unwrap_or_default())
        .unwrap_or("text/html");

    eprintln!("");

    match output {
        Some(file) => fs::write(file, response.text().await?)?,

        None => {
            if content_type.contains("json") {
                let body: Value = response.json().await?;

                if io::stdout().is_terminal() {
                    let colorizer = Colorizer::arbitrary();
                    match colorizer.colorize_json_str(&serde_json::to_string(&body)?) {
                        Ok(body) => println!("{}", body),
                        Err(_) => print!("{}", body),
                    }
                } else {
                    print!("{}", body);
                }

            } else {
                let body = response.text().await?;
                println!("{}", body);
            }
        }
    }

    Ok(())
}
