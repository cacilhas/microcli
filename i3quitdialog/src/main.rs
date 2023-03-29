mod resources;

use std::io;
use fltk::{
    app::{App, screen_size},
    button::Button,
    group,
    frame::Frame,
    prelude::*,
    window::Window,
};
use i3_ipc::{
    Connect,
    I3,
};
use users::{
    UsersCache,
    Users,
};

use crate::resources::Resources;


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

    if is_power_user() {
        let mut hpack1 = group::Pack::new(
            0, y,
            win.width(), 30,
            "",
        );
        create_halt_button(&resources, btsize).unwrap();
        create_reboot_button(&resources, btsize).unwrap();
        hpack1.end();
        hpack1.set_type(group::PackType::Horizontal);
        y += hpack1.height();
    }

    let mut hpack2 = group::Pack::new(
        0, y,
        win.width(), 30,
        "",
    );

    let mut exit = create_exit_button(&resources, btsize).unwrap();
    create_cancel_button(&resources, btsize, move |_| app.quit()).unwrap();

    hpack2.end();
    hpack2.set_type(group::PackType::Horizontal);

    win.end();
    let height = y + hpack2.height();
    win.set_size(winsize.0, height);
    win.show();

    exit.take_focus().unwrap();
    app.run().unwrap();
}


fn create_exit_button(resources: &Resources, btsize: i32) -> io::Result<Button> {
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


fn create_halt_button(resources: &Resources, btsize: i32) -> io::Result<Button> {
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


fn create_reboot_button(resources: &Resources, btsize: i32) -> io::Result<Button> {
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


fn create_cancel_button<C: FnMut(&mut Button) + 'static>(resources: &Resources, btsize: i32, cb: C) -> io::Result<Button> {
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


fn is_power_user() -> bool {
    let cache = UsersCache::new();
    let uid = cache.get_current_uid();
    let user = match cache.get_user_by_uid(uid) {
        Some(user) => user,
        None => return false,
    };

    match user.groups() {
        Some(groups) => groups.iter().any(|group| group.name() == "power"),
        None => false,
    }
}
