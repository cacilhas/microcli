use std::env;
use std::fs;
use std::path::{Display, Path};

enum Operation {
    INC,
    DEC,
}

use Operation::*;


#[cfg(target_os = "linux")]
fn main() {
    let args: Vec<String> = env::args().collect();
    let param = match args.len() {
        1 => None,
        2 => match &*args[1] {
            "+" => Some(INC),
            "-" => Some(DEC),
            _   => panic!("Invalid argument {}", args[1]),
        },
        _ => panic!("too many arguments"),
    };
    let paths = fs::read_dir("/sys/class/backlight").unwrap();

    for path in paths {
        match path {
            Ok(entry) => apply_to_backlight(&entry.path().display(), &param),
            Err(_)    => {},
        }
    }
}

fn apply_to_backlight(backlight: &Display, operation: &Option<Operation>) {
    let dir = backlight.to_string();
    let file = Path::new(&dir).join("max_brightness");
    let max = file_to_int(&file);
    let file = Path::new(&dir).join("brightness");
    let current = file_to_int(&file);
    let step = max / 10;

    match operation {
        None => println!("{:#?}", current),

        Some(INC) => {
            let desired = *vec![(current as i16) + (step as i16), max as i16].iter().min().unwrap() as u8;
            let raw = format!("{}\n", desired);
            fs::write(&file, raw).expect("failed to save new brightness");
            println!("{:#?}", desired);
        },

        Some(DEC) => {
            let desired = *vec![(current as i16) - (step as i16), 0_i16].iter().max().unwrap() as u8;
            let raw = format!("{}\n", desired);
            fs::write(&file, raw).expect("failed to save new brightness");
            println!("{:#?}", desired);
        },
    };
}

fn file_to_int(file: &Path) -> u8 {
    let raw = fs::read_to_string(file).expect("unable to read backlight values");
    let raw = raw.trim();
    return raw.parse::<u8>().unwrap();
}
