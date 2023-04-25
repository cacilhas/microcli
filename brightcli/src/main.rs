extern crate anyhow;

mod operation;
mod paramerror;

use std::{
    env,
    fs,
};

use operation::Operation;


#[cfg(target_os = "linux")]
fn main() {
    let args: Vec<String> = env::args().collect();
    let operation = Operation::from_args(&args).unwrap();
    let paths = fs::read_dir("/sys/class/backlight").unwrap();

    for path in paths {
        match path {
            Ok(entry) =>
                match operation.apply(&entry.path().display()) {
                    Ok(value) => println!("{value}"),
                    Err(err) => eprintln!("{err}"),
                },
            Err(_) => continue,
        };
    };
}
