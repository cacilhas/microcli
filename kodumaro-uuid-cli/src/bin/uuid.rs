extern crate kodumaro_uuid_cli as uuid_cli;

use std::env;

use clap::{Parser, Subcommand};
use eyre::Result;
use uuid::Uuid;

#[derive(Debug, Parser)]
#[command(
    author, about, version,
    name = "uuid",
    long_about = include_str!("long-help.txt"),
)]
struct Cli {
    #[command(subcommand)]
    version: Option<Version>,
}

#[derive(Debug, Subcommand)]
enum Version {
    #[command(about = "generates nil UUID")]
    Nil,
    #[command(about = "generates UUIDv1, time-based UUID")]
    V1,
    #[command(about = "generates UUIDv3, name-based MD5 UUID")]
    V3 {
        #[arg(name = "NAMESPACE", help = "any UUID, preferable v1, v4, or v7")]
        ns: Uuid,
        #[arg(help = "arbitrary qualifier")]
        name: String,
    },
    #[command(about = "generates UUIDv4, random UUID")]
    V4,
    #[command(about = "generates UUIDv5, name-based SHA1 UUID")]
    V5 {
        #[arg(name = "NAMESPACE", help = "any UUID, preferable v1, v4, or v7")]
        ns: Uuid,
        #[arg(help = "arbitrary qualifier")]
        name: String,
    },
    #[command(about = "generates UUIDv6, field-compatible version of UUIDv1")]
    V6 {
        #[arg(name = "NODE ID", help = "a 6-byte long node identifier")]
        node_id: String,
    },
    #[command(about = "generates UUIDv7, Unix Epoch timestamp-based UUID")]
    V7,
    #[command(about = "generates UUIDv8, vendor-specific UUID")]
    V8 {
        #[arg(help = "vendor’s metadata to be encoded into the UUIDv8, up to 16 bytes")]
        metadata: String,
    },
}

fn main() -> Result<()> {
    let command = Cli::parse();
    match command.version.unwrap_or(Version::V4) {
        Version::Nil             => display(uuid::Uuid::nil()),
        Version::V1              => display(uuid_cli::get_v1()?),
        Version::V3 { ns, name } => display(uuid_cli::get_v3(ns, name)?),
        Version::V4              => display(uuid_cli::get_v4()?),
        Version::V5 { ns, name } => display(uuid_cli::get_v5(ns, name)?),
        Version::V6 { node_id }  => display(uuid_cli::get_v6(node_id)?),
        Version::V7              => display(uuid_cli::get_v7()?),
        Version::V8 { metadata } => display(uuid_cli::get_v8(metadata)?),
    }

    Ok(())
}


fn display(msg: impl ToString) {
    let prefix = match env::var("UUID_MODE") {
        Ok(mode) if mode == "uuidgen" => "",
        _ => "urn:uuid:",
    };

    println!("{}{}", prefix, msg.to_string());
}
