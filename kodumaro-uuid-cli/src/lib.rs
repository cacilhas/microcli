extern crate uuid;
mod errors;

pub use errors::Error;
use mac_address::get_mac_address;
use uuid::Uuid;

pub fn get_v1() -> anyhow::Result<String> {
    match get_mac_address()? {
        Some(addr) => Ok(format!("{}", Uuid::now_v1(&addr.bytes()))),
        None => Err(Error::Missing("mac address".to_owned()).into()),
    }
}

pub fn get_v3(namespace: Uuid, name: String) -> anyhow::Result<String> {
    Ok(format!("{}", Uuid::new_v3(&namespace, name.as_bytes())))
}

pub fn get_v4() -> anyhow::Result<String> {
    Ok(format!("{}", Uuid::new_v4()))
}

pub fn get_v5(namespace: Uuid, name: String) -> anyhow::Result<String> {
    Ok(format!("{}", Uuid::new_v5(&namespace, name.as_bytes())))
}

pub fn get_v6(node_id: String) -> anyhow::Result<String> {
    if node_id.len() < 6 {
        return Err(Error::WrongLength {
            expected: 6,
            got: node_id.len(),
        }
        .into());
    }
    let node_id: &[u8; 6] = &node_id.as_bytes()[0..6].try_into()?;
    Ok(format!("{}", Uuid::now_v6(node_id)))
}

pub fn get_v7() -> anyhow::Result<String> {
    Ok(format!("{}", Uuid::now_v7()))
}

pub fn get_v8(metadata: String) -> anyhow::Result<String> {
    let metadata = metadata.as_bytes();
    let mut buf = [0_u8; 16];
    let length = metadata.len();
    let length = if length > 16 { 16 } else { length };
    for i in 0..length {
        buf[i] = metadata[i];
    }
    Ok(format!("{}", Uuid::new_v8(buf)))
}
