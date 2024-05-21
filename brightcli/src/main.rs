mod minmaxerror;
mod operation;
mod paramerror;

use std::{env, error::Error, fs};

use operation::Operation;

#[cfg(target_os = "linux")]
fn main() -> Result<(), Box<dyn Error>> {
    let operation = Operation::from_args(&mut env::args())?;
    let paths = fs::read_dir("/sys/class/backlight")?;

    for path in paths {
        match path {
            Ok(entry) => match operation.apply(&entry.path().display()) {
                Ok(value) => println!("{value}"),
                Err(err) => eprintln!("{err}"),
            },
            Err(_) => continue,
        };
    }

    Ok(())
}
