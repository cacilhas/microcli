//! UUID generator ([RFC 4122](https://www.rfc-editor.org/rfc/rfc4122)).
//!
//! This is a command line tool, **do not** install it using `cargo add`!!
//!
//! # Installation guide
//!
//! You need to enable `uuid_unstable` configuration flag:
//!
//! ```sh
//! RUSTFLAGS='--cfg uuid_unstable' cargo install kodumaro-uuid-cli
//! ```
//!
//! It’s gonna create a `~/.cargo/bin/uuid` executable.
//!
//! # Usage
//!
//! ```sh
//! Usage: uuid [COMMAND]
//!
//! Commands:
//!   nil   generates nil UUID
//!   v1    generates UUIDv1, time-based UUID
//!   v3    generates UUIDv3, name-based MD5 UUID
//!   v4    generates UUIDv4, random UUID
//!   v5    generates UUIDv5, name-based SHA1 UUID
//!   v6    generates UUIDv6, field-compatible version of UUIDv1
//!   v7    generates UUIDv7, Unix Epoch timestamp-based UUID
//!   v8    generates UUIDv8, vendor-specific UUID
//!   help  Print this message or the help of the given subcommand(s)
//!
//! Options:
//!   -h, --help  Print help
//! ```
//!
//! ## Examples
//!
//! ```sh
//! $ uuid
//! urn:uuid:4db78d44-e170-42a3-bf93-418b9baeae2b
//!
//! $ uuid help
//! UUID generator (RFC 4122), see <https://www.rfc-editor.org/rfc/rfc4122>
//!
//! It generates UUID versions 1, 3, 4, 5, 6, 7 and 8, and nil UUID.
//!
//! It returns the URN; if you want to emulate the same behaviour as uuigen (plain
//! UUID), set the env-var:
//!
//! export UUID_MODE=uuidgen
//!
//!
//! Usage: uuid [COMMAND]
//!
//! Commands:
//!   nil
//!           generates nil UUID
//!   v1
//!           generates UUIDv1, time-based UUID
//!   v3
//!           generates UUIDv3, name-based MD5 UUID
//!   v4
//!           generates UUIDv4, random UUID
//!   v5
//!           generates UUIDv5, name-based SHA1 UUID
//!   v6
//!           generates UUIDv6, field-compatible version of UUIDv1
//!   v7
//!           generates UUIDv7, Unix Epoch timestamp-based UUID
//!   v8
//!           generates UUIDv8, vendor-specific UUID
//!   help
//!           Print this message or the help of the given subcommand(s)
//!
//! Options:
//!   -h, --help
//!           Print help (see a summary with '-h')
//!
//! $ uuid nil
//! urn:uuid:00000000-0000-0000-0000-000000000000
//!
//! $ uuid v1
//! urn:uuid:xxxxxxxx-xxxx-1xxx-xxxx-xxxxxxxxxxxx
//!
//! $ uuid v3 $(uuid v7) test
//! urn:uuid:5604097f-ffa0-3934-9635-cb03308240fe
//!
//! $ uuid v5 $(uuid v7) test
//! urn:uuid:d8beedbe-ca82-57ef-8dc1-ca501caeb151
//!
//! $ uuid v6 blabla
//! urn:uuid:xxxxxxxx-xxxx-6xxx-xxxx-xxxxxxxxxxxx
//!
//! $ uuid v7
//! urn:uuid:018800be-993e-7990-b64a-900ba7dd54e3
//!
//! $ uuid v8 'Some long data!'
//! urn:uuid:536f6d65-206c-8f6e-a720-646174612100
//!
//! $ UUID_MODE=uuidgen uuid v7
//! 01880c8f-d233-7be3-b1f5-95ea2650457f
//!
//! $ uuid help v8
//! generates UUIDv8, vendor-specific UUID
//!
//! Usage: uuid v8 <METADATA>
//!
//! Arguments:
//!   <METADATA>  vendor’s metadata to be encoded into the UUIDv8, up to 16 bytes
//!
//! Options:
//!   -h, --help  Print help
//! ```
//!
//! # License
//!
//! - [The 3-Clause BSD License](https://opensource.org/license/bsd-3-clause/)

extern crate uuid;
mod errors;

use std::error::Error;

pub use errors::UUIDError;
use mac_address::get_mac_address;
use uuid::Uuid;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub fn get_v1() -> Result<String> {
    let addr = get_mac_address()?.ok_or_else(|| UUIDError::Missing("mac address".to_owned()))?;
    Ok(format!("{}", Uuid::now_v1(&addr.bytes())))
}

pub fn get_v3(namespace: Uuid, name: String) -> Result<String> {
    Ok(format!("{}", Uuid::new_v3(&namespace, name.as_bytes())))
}

pub fn get_v4() -> Result<String> {
    Ok(format!("{}", Uuid::new_v4()))
}

pub fn get_v5(namespace: Uuid, name: String) -> Result<String> {
    Ok(format!("{}", Uuid::new_v5(&namespace, name.as_bytes())))
}

pub fn get_v6(node_id: String) -> Result<String> {
    if node_id.len() < 6 {
        return Err(UUIDError::WrongLength {
            expected: 6,
            got: node_id.len(),
        }
        .into());
    }
    let node_id: &[u8; 6] = &node_id.as_bytes()[0..6].try_into()?;
    Ok(format!("{}", Uuid::now_v6(node_id)))
}

pub fn get_v7() -> Result<String> {
    Ok(format!("{}", Uuid::now_v7()))
}

pub fn get_v8(metadata: String) -> Result<String> {
    let metadata = metadata.as_bytes();
    let mut buf = [0_u8; 16];
    let length = metadata.len();
    let length = if length > 16 { 16 } else { length };
    for i in 0..length {
        buf[i] = metadata[i];
    }
    Ok(format!("{}", Uuid::new_v8(buf)))
}
