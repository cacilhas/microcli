extern crate clap;
extern crate kodumaro_uuid_cli as uuid_cli;
extern crate uuid;

use clap::{Parser, Subcommand};
use uuid::Uuid;

#[derive(Debug, Parser)]
#[command(
    author = "Montegasppα Cacilhας <montegasppa@cacilhas.info>",
    about = "UUID generator (RFC 4122), see <https://www.rfc-editor.org/rfc/rfc4122>",
    name = "uuid",
    long_about = None,
)]
struct Cli {
    #[command(subcommand)]
    version: Option<Version>,
}

#[derive(Debug, Subcommand)]
enum Version {
    #[command(about = "generate nil UUID")]
    NIL,
    #[command(about = "generate UUIDv1, time-based UUID")]
    V1,
    #[command(about = "generate UUIDv3, name-based MD5 UUID")]
    V3 {
        #[arg(name = "NAMESPACE", help = "any UUID, preferable v1, v4, or v7")]
        ns: Uuid,
        #[arg(help = "arbitrary qualifier")]
        name: String,
    },
    #[command(about = "generate UUIDv4, random UUID")]
    V4,
    #[command(about = "generate UUIDv5, name-based SHA1 UUID")]
    V5 {
        #[arg(name = "NAMESPACE", help = "any UUID, preferable v1, v4, or v7")]
        ns: Uuid,
        #[arg(help = "arbitrary qualifier")]
        name: String,
    },
    #[command(about = "generate UUIDv6, field-compatible version of UUIDv1")]
    V6 {
        #[arg(name = "NODE ID", help = "a 6-byte long node identifier")]
        node_id: String,
    },
    #[command(about = "generate UUIDv7, Unix Epoch timestamp-based UUID")]
    V7,
    #[command(about = "generate UUIDv8, vendor-specific UUID")]
    V8 {
        #[arg(help = "vendor’s metadata to be encoded into the UUIDv8, up to 16 bytes")]
        metadata: String,
    },
}

fn main() -> anyhow::Result<()> {
    let command = Cli::parse();
    match command.version.unwrap_or(Version::V4) {
        Version::NIL => println!("urn:uuid:{}", uuid::Uuid::nil()),

        Version::V1 => println!("urn:uuid:{}", uuid_cli::get_v1()?),

        Version::V3 { ns, name } => println!("urn:uuid:{}", uuid_cli::get_v3(ns, name)?),

        Version::V4 => println!("urn:uuid:{}", uuid_cli::get_v4()?),

        Version::V5 { ns, name } => println!("urn:uuid:{}", uuid_cli::get_v5(ns, name)?),

        Version::V6 { node_id } => println!("urn:uuid:{}", uuid_cli::get_v6(node_id)?),

        Version::V7 => println!("urn:uuid:{}", uuid_cli::get_v7()?),

        Version::V8 { metadata } => println!("urn:uuid:{}", uuid_cli::get_v8(metadata)?),
    }

    Ok(())
}
