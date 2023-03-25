use std::{
    io,
    process,
};
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
    I3,
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
    let resources = get_resources();

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
        "Exit",
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
        "Halt",
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
        "Reboot",
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
        "Cancel",
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


fn get_resources() -> Resources {
    let mut resources = Resources {
        background: Color::Dark3,
        foreground: Color::White,
        exit_fg_color: Color::White,
        exit_bg_color: Color::DarkYellow,
        cancel_fg_color: Color::Cyan,
        cancel_bg_color: Color::DarkGreen,
        halt_fg_color: Color::Yellow,
        halt_bg_color: Color::DarkRed,
        reboot_fg_color: Color::Magenta,
        reboot_bg_color: Color::DarkMagenta,
    };

    legacy_resources(&mut resources);
    set_resources(&mut resources);

    resources
}

fn set_resources(resources: &mut Resources) {
    let app_name = "i3quitdialog";

    match get_color_parameter(&app_name, "Title.foreground") {
        Some(color) => resources.foreground = color,
        None => (),
    }

    match get_color_parameter(&app_name, "Title.background") {
        Some(color) => resources.background = color,
        None => (),
    }

    match get_color_parameter(&app_name, "ExitButton.color") {
        Some(color) => {
            resources.exit_bg_color = color;
            resources.exit_fg_color = contrast(&color);
        },
        None => (),
    }

    match get_color_parameter(&app_name, "CancelButton.color") {
        Some(color) => {
            resources.cancel_bg_color = color;
            resources.cancel_fg_color = contrast(&color);
        },
        None => (),
    }

    match get_color_parameter(&app_name, "HaltButton.color") {
        Some(color) => {
            resources.halt_bg_color = color;
            resources.halt_fg_color = contrast(&color);
        },
        None => (),
    }

    match get_color_parameter(&app_name, "RebootButton.color") {
        Some(color) => {
            resources.reboot_bg_color = color;
            resources.reboot_fg_color = contrast(&color);
        },
        None => (),
    }
}

fn legacy_resources(resources: &mut Resources) {
    let app_name = "I3QuitDialog";

    match get_color_parameter(&app_name, "foreground") {
        Some(color) => resources.foreground = color,
        None => match get_color_parameter(&app_name, "Foreground") {
            Some(color) => resources.foreground = color,
            None => (),
        },
    }

    match get_color_parameter(&app_name, "background") {
        Some(color) => resources.background = color,
        None => match get_color_parameter(&app_name, "Background") {
            Some(color) => resources.background = color,
            None => (),
        },
    }

    match get_color_parameter(&app_name, "exitColor") {
        Some(color) => {
            resources.exit_bg_color = color;
            resources.exit_fg_color = contrast(&color);
        },
        None => (),
    }

    match get_color_parameter(&app_name, "cancelColor") {
        Some(color) => {
            resources.cancel_bg_color = color;
            resources.cancel_fg_color = contrast(&color);
        },
        None => (),
    }

    match get_color_parameter(&app_name, "haltColor") {
        Some(color) => {
            resources.halt_bg_color = color;
            resources.halt_fg_color = contrast(&color);
        },
        None => (),
    }

    match get_color_parameter(&app_name, "rebootColor") {
        Some(color) => {
            resources.reboot_bg_color = color;
            resources.reboot_fg_color = contrast(&color);
        },
        None => (),
    }
}


fn get_color_parameter(app_name: &str, parameter: &str) -> Option<Color> {
    let output = process::Command::new("xrdb")
        .arg( "-get")
        .arg(format!("{}.{}", app_name, parameter))
        .output();
    let output = match output {
        Ok(output) => output,
        Err(_) => return None,
    };
    if !output.status.success() {
        return None;
    }
    let res = match String::from_utf8(output.stdout) {
        Ok(output) => output,
        Err(_) => return None,
    };
    let res = res.trim();

    if res.starts_with("#") {
        match Color::from_hex_str(res) {
            Ok(color) => return Some(color),
            Err(_) => return None,
        }
    }

    match res.to_lowercase().as_str() {
        "dark3" => Some(Color::Dark3),
        "dark2" => Some(Color::Dark2),
        "dark1" => Some(Color::Dark1),
        "light1" => Some(Color::Light1),
        "light2" => Some(Color::Light2),
        "light3" => Some(Color::Light3),
        "black" => Some(Color::Black),
        "red" => Some(Color::Red),
        "green" => Some(Color::Green),
        "yellow" => Some(Color::Yellow),
        "blue" => Some(Color::Blue),
        "magenta" => Some(Color::Magenta),
        "cyan" => Some(Color::Cyan),
        "darkred" => Some(Color::DarkRed),
        "darkgreen" => Some(Color::DarkGreen),
        "darkyellow" => Some(Color::DarkYellow),
        "darkblue" => Some(Color::DarkBlue),
        "darkmagenta" => Some(Color::DarkMagenta),
        "darkcyan" => Some(Color::DarkCyan),
        "white" => Some(Color::White),

        color => match color.parse::<u8>() {
            Ok(color) => Some(Color::by_index(color)),
            Err(_) => None,
        },
    }
}


fn contrast(color: &Color) -> Color {
    let (mut r, mut  g, mut b) = color.to_rgb();
    if is_bright(color) {
        r /= 2;
        g /= 2;
        b /= 2;
    } else {
        r = 0xff - (0xff - r) / 2;
        g = 0xff - (0xff - g) / 2;
        b = 0xff - (0xff - b) / 2;
    }
    Color::from_rgb(r, g, b)
}


fn is_bright(color: &Color) -> bool {
    let (r, g, b) = color.to_rgb();
    vec![r, g, b].iter().fold(0_u8, |acc, &v| acc.max(v)) >= 0xa0
}


struct Resources {
    background: Color,
    foreground: Color,
    exit_fg_color: Color,
    exit_bg_color: Color,
    cancel_fg_color: Color,
    cancel_bg_color: Color,
    halt_fg_color: Color,
    halt_bg_color: Color,
    reboot_fg_color: Color,
    reboot_bg_color: Color,
}
