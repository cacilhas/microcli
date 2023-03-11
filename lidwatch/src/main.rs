use std::env;
use std::process::Command;
use evdev::{Device, InputEventKind, SwitchType};


#[cfg(any(
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "linux",
    target_os = "macos",
    target_os = "netbsd",
    target_os = "openbsd",
))]
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        panic!("expected lid event block")
    }
    let file: &String = &args[1];
    let command: &String = &args[2];
    let params = &args[3..];
    let mut _device = Device::open(file).unwrap();
    let mut state = 0_i32;

    loop {
        let events = _device.fetch_events().unwrap().filter(|event| event.kind() == InputEventKind::Switch(SwitchType::SW_LID));
        for event in events {
            let value = event.value();
            if value == 1 && state == 0 {
                let mut cmd = Command::new(command);
                for param in params {
                    cmd.arg(param);
                }
                match cmd.spawn() {
                    Ok(_)  => (),
                    Err(err) => eprintln!("{:#?}", err),
                }
            } else {
                state = value;
            }
        }
    }
}
