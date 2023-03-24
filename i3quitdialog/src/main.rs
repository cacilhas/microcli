use std::error;
use fltk::{
    app::{App, screen_size},
    button::Button,
    group,
    enums::*,
    frame::Frame,
    prelude::*,
    window::Window,
};
use i3_ipc::{
    Connect,
    I3
};
use users::{
    UsersCache,
    Users,
};


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

    let app = App::default();
    let mut win = Window::new(
        winx, winy,
        winsize.0, winsize.1,
        "i3 Quit Dialog",
    );
    win.set_color(Color::Dark3);
    let mut title = Frame::new(
        0, 0,
        win.width(), 30,
        "Do you really want to exit i3?",
    );
    title.set_label_color(Color::White);
    title.set_label_size(24);
    let mut hpack = group::Pack::new(
        0, title.height(),
        win.width(), 30,
        "",
    );

    let btsize = win.width() / 4;
    create_exit_button(btsize).unwrap();
    if is_power_user() {
        create_halt_button(btsize).unwrap();
        create_reboot_button(btsize).unwrap();
    }
    create_cancel_button(btsize, Box::new(move |_| app.quit())).unwrap();

    hpack.end();
    hpack.set_type(group::PackType::Horizontal);
    win.end();
    let height = title.height() + hpack.height();
    win.set_size(winsize.0, height);
    win.show();

    app.run().unwrap();
}


fn create_exit_button(btsize: i32) -> Result<Button, Box<dyn error::Error>> {
    let mut exit = Button::new(
        0, 0,
        btsize, 0,
        "Exit",
    );
    exit.set_color(Color::DarkYellow);
    exit.set_label_color(Color::White);
    exit.set_callback(move |_| {
        let mut i3 = I3::connect().unwrap();
        i3.run_command("exit").unwrap();
    });
    Ok(exit)
}


fn create_halt_button(btsize: i32) -> Result<Button, Box<dyn error::Error>> {
    let mut halt = Button::new(
        0, 0,
        btsize, 0,
        "Halt",
    );
    halt.set_color(Color::DarkRed);
    halt.set_label_color(Color::Yellow);
    halt.set_callback(move |_| {
        let mut i3 = I3::connect().unwrap();
        i3.run_command("exec --no-startup-id halt -p").unwrap();
    });
    Ok(halt)
}


fn create_reboot_button(btsize: i32) -> Result<Button, Box<dyn error::Error>> {
    let mut reboot = Button::new(
        0, 0,
        btsize, 0,
        "Reboot",
    );
    reboot.set_color(Color::DarkMagenta);
    reboot.set_label_color(Color::Magenta);
    reboot.set_callback(move |_| {
        let mut i3 = I3::connect().unwrap();
        i3.run_command("exec --no-startup-id reboot").unwrap();
    });
    Ok(reboot)
}


fn create_cancel_button(btsize: i32, cb: Box<dyn FnMut(&mut Button)>) -> Result<Button, Box<dyn error::Error>> {
    let mut cancel = Button::new(
        0, 0,
        btsize, 0,
        "Cancel",
    );
    cancel.set_color(Color::DarkGreen);
    cancel.set_label_color(Color::Cyan);
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
        Some(groups) => {
            for group in groups.iter() {
                if group.name() == "power" {
                    return true;
                }
            }
            false
        },
        None => false,
    }
}
