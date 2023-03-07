use fltk::{prelude::*, enums::*, *, app::screen_size};
use i3_ipc::{Connect, I3};


fn main() {
    let (width, height) = screen_size();
    let winx = ((width as i32)-300) / 2;
    let winy = ((height as i32)-300) / 2;

    let app = app::App::default();
    let mut win = window::Window::new(
        winx, winy,
        340, 1,
        "i3 Quit Dialog",
    );
    win.set_color(Color::DarkCyan);
    let mut vpack = group::Pack::new(
        0, 0,
        win.width(), win.height(),
        "",
    );
    let mut title = frame::Frame::new(
        0, 0,
        win.width(), 30,
        "Do you really want to exit i3?",
    );
    title.set_label_size(24);
    let mut hpack = group::Pack::new(
        0, 0,
        win.width(), 30,
        "",
    );
    let btsize = win.width() / 4;
    let mut exit = button::Button::new(
        0, 0,
        btsize, 0,
        "Exit",
    );
    exit.set_color(Color::Yellow);
    let mut halt = button::Button::new(
        0, 0,
        btsize, 0,
        "Halt",
    );
    halt.set_color(Color::Red);
    let mut reboot = button::Button::new(
        0, 0,
        btsize, 0,
        "Reboot",
    );
    reboot.set_color(Color::DarkMagenta);
    let mut cancel = button::Button::new(
        0, 0,
        btsize, 0,
        "Cancel",
    );
    cancel.set_color(Color::Green);
    hpack.end();
    hpack.set_type(group::PackType::Horizontal);
    vpack.end();
    vpack.set_type(group::PackType::Vertical);
    win.end();
    win.show();

    halt.set_callback(move |_| {
        let mut i3 = I3::connect().unwrap();
        i3.run_command("exec --no-startup-id sudo halt -p").unwrap();
    });
    reboot.set_callback(move |_| {
        let mut i3 = I3::connect().unwrap();
        i3.run_command("exec --no-startup-id sudo reboot").unwrap();
    });
    exit.set_callback(move |_| {
        let mut i3 = I3::connect().unwrap();
        i3.run_command("exit").unwrap();
    });
    cancel.set_callback(move |_| app.quit());

    app.run().unwrap();
}
