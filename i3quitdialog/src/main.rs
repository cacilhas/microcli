mod resources;
mod users;

// Buttons
mod cancelbt;
mod exitbt;
mod haltbt;
mod rebootbt;

use fltk::{
    app::{App, screen_size},
    group,
    frame::Frame,
    prelude::*,
    window::Window,
};

use crate::resources::Resources;
use crate::users::User;


#[cfg(any(
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "linux",
    target_os = "netbsd",
    target_os = "openbsd",
))]
fn main() {
    let winsize = (340, 75);
    let (width, height) = screen_size();
    let winx = ((width as i32)-winsize.0) / 2;
    let winy = ((height as i32)-winsize.1) / 2;
    let resources = Resources::default();

    let app = App::default();
    let mut win = Window::new(
        winx, winy,
        winsize.0, winsize.1,
        "i3 Quit Dialog",
    );
    win.set_color(resources.background);
    let mut title = Frame::new(
        0, 0,
        win.width(), 30,
        "Do you really want to exit i3?",
    );
    title.set_label_color(resources.foreground);
    title.set_label_size(24);
    let btsize = win.width() / 2;
    let mut y = title.height();
    let user = User::default();

    if user.is_power_user() {
        let mut hpack1 = group::Pack::new(
            0, y,
            win.width(), 30,
            "",
        );
        haltbt::create(&resources, btsize).unwrap();
        rebootbt::create(&resources, btsize).unwrap();
        hpack1.end();
        hpack1.set_type(group::PackType::Horizontal);
        y += hpack1.height();
    }

    let mut hpack2 = group::Pack::new(
        0, y,
        win.width(), 30,
        "",
    );

    let mut exit = exitbt::create(&resources, btsize).unwrap();
    cancelbt::create(&resources, btsize, move |_| app.quit()).unwrap();

    hpack2.end();
    hpack2.set_type(group::PackType::Horizontal);

    win.end();
    let height = y + hpack2.height();
    win.set_size(winsize.0, height);
    win.show();

    exit.take_focus().unwrap();
    app.run().unwrap();
}
