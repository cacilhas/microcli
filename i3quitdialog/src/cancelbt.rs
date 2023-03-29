use std::io;

use fltk::{
    button::Button,
    prelude::*,
};

use crate::resources::Resources;


pub fn create<C>(resources: &Resources, btsize: i32, cb: C) -> io::Result<Button>
where
    C: FnMut(&mut Button) + 'static,
{
    let mut cancel = Button::new(
        0, 0,
        btsize, 0,
        "⎋ Cancel",
    );
    cancel.set_color(resources.cancel_bg_color);
    cancel.set_label_color(resources.cancel_fg_color);
    cancel.set_callback(cb);
    Ok(cancel)
}
