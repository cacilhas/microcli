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
    let operation = match Operation::from_args(&args) {
        Ok(operation) => operation,
        Err(err) => panic!("{err}"),
    };
    let paths = match fs::read_dir("/sys/class/backlight") {
        Ok(path) => path,
        Err(err) => panic!("{err}"),
    };

    for path in paths {
        match path {
            Ok(entry) =>
                match operation.apply(&entry.path().display()) {
                    Ok(value) => println!("{value}"),
                    Err(err) => eprintln!("{err}"),
                },
            Err(_) => {},
        };
    };
}
