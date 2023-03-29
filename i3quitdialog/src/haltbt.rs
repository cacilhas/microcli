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
    let mut halt = Button::new(
        0, 0,
        btsize, 0,
        "⏻ Halt",
    );
    halt.set_color(resources.halt_bg_color);
    halt.set_label_color(resources.halt_fg_color);
    halt.set_callback(move |_| {
        let mut i3 = I3::connect().unwrap();
        i3.run_command("exec --no-startup-id halt -p").unwrap();
    });
    Ok(halt)
}
