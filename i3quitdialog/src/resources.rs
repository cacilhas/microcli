use std::process;
use fltk::enums::Color;


const APP_NAME: &'static str = "i3quitdialog";


#[derive(Debug)]
pub struct Resources {
    pub background: Color,
    pub foreground: Color,
    pub exit_fg_color: Color,
    pub exit_bg_color: Color,
    pub cancel_fg_color: Color,
    pub cancel_bg_color: Color,
    pub halt_fg_color: Color,
    pub halt_bg_color: Color,
    pub reboot_fg_color: Color,
    pub reboot_bg_color: Color,
}


impl Default for Resources {
    fn default() -> Self {
        let mut res = Self {
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
        set_resources(&mut res);
        res
    }
}


fn set_resources(resources: &mut Resources) {
    match get_color_parameter("Title.foreground") {
        Some(color) => resources.foreground = color,
        None => (),
    }

    match get_color_parameter("Title.background") {
        Some(color) => resources.background = color,
        None => (),
    }

    match get_color_parameter("ExitButton.color") {
        Some(color) => {
            resources.exit_bg_color = color;
            resources.exit_fg_color = contrast(&color);
        },
        None => (),
    }

    match get_color_parameter("CancelButton.color") {
        Some(color) => {
            resources.cancel_bg_color = color;
            resources.cancel_fg_color = contrast(&color);
        },
        None => (),
    }

    match get_color_parameter("HaltButton.color") {
        Some(color) => {
            resources.halt_bg_color = color;
            resources.halt_fg_color = contrast(&color);
        },
        None => (),
    }

    match get_color_parameter("RebootButton.color") {
        Some(color) => {
            resources.reboot_bg_color = color;
            resources.reboot_fg_color = contrast(&color);
        },
        None => (),
    }
}


fn get_color_parameter(parameter: &str) -> Option<Color> {
    let output = process::Command::new("xrdb")
        .arg( "-get")
        .arg(format!("{}.{}", APP_NAME, parameter))
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
