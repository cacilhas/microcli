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
    let mut reboot = Button::new(
        0, 0,
        btsize, 0,
        "⏼ Reboot",
    );
    reboot.set_color(resources.reboot_bg_color);
    reboot.set_label_color(resources.reboot_fg_color);
    reboot.set_callback(move |_| {
        let mut i3 = I3::connect().unwrap();
        i3.run_command("exec --no-startup-id reboot").unwrap();
    });
    Ok(reboot)
}
