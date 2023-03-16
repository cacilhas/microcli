use fltk::{prelude::*, enums::*, *, app::screen_size};
use i3_ipc::{Connect, I3};


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

    let app = app::App::default();
    let mut win = window::Window::new(
        winx, winy,
        winsize.0, winsize.1,
        "i3 Quit Dialog",
    );
    win.set_color(Color::Dark3);
    let mut title = frame::Frame::new(
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
    let mut exit = button::Button::new(
        0, 0,
        btsize, 0,
        "Exit",
    );
    exit.set_color(Color::DarkYellow);
    exit.set_label_color(Color::Yellow);
    let mut halt = button::Button::new(
        0, 0,
        btsize, 0,
        "Halt",
    );
    halt.set_color(Color::DarkRed);
    halt.set_label_color(Color::Red);
    let mut reboot = button::Button::new(
        0, 0,
        btsize, 0,
        "Reboot",
    );
    reboot.set_color(Color::DarkMagenta);
    reboot.set_label_color(Color::Magenta);
    let mut cancel = button::Button::new(
        0, 0,
        btsize, 0,
        "Cancel",
    );
    cancel.set_color(Color::DarkGreen);
    cancel.set_label_color(Color::Green);
    hpack.end();
    hpack.set_type(group::PackType::Horizontal);
    win.end();
    let height = title.height() + hpack.height();
    win.set_size(winsize.0, height);
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
