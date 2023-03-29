use std::io;

use fltk::{
    button::Button,
    prelude::*,
};
use i3_ipc::{
    Connect,
    I3,
};

use crate::resources::Resources;


pub fn create(resources: &Resources, btsize: i32) -> io::Result<Button> {
    let mut exit = Button::new(
        0, 0,
        btsize, 0,
        "↩ Exit",
    );
    exit.set_color(resources.exit_bg_color);
    exit.set_label_color(resources.exit_fg_color);
    exit.set_callback(move |_| {
        let mut i3 = I3::connect().unwrap();
        i3.run_command("exit").unwrap();
    });
    Ok(exit)
}
