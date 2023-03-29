mod operation;

use std::{
    error,
    env,
    fs,
};

use operation::Operation;


#[cfg(target_os = "linux")]
fn main() -> Result<(), Box<dyn error::Error>> {
    let args: Vec<String> = env::args().collect();
    let operation = Operation::from_args(&args)?;
    let paths = fs::read_dir("/sys/class/backlight")?;

    for path in paths {
        match path {
            Ok(entry) => operation.apply(&entry.path().display())?,
            Err(_) => {},
        };
    };

    Ok(())
}
