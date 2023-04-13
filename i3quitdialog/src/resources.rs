use std::process;
use eframe::egui::Color32 as Color;


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
            background: Color::DARK_GRAY,
            foreground: Color::WHITE,
            exit_fg_color: Color::WHITE,
            exit_bg_color: Color::GOLD,
            cancel_fg_color: Color::from_rgb(0, 0xff, 0xff), // Cyan
            cancel_bg_color: Color::DARK_GREEN,
            halt_fg_color: Color::YELLOW,
            halt_bg_color: Color::DARK_RED,
            reboot_fg_color: Color::from_rgb(0xff, 0, 0xff), // Magenta
            reboot_bg_color: Color::from_rgb(0x60, 0, 0x60), // DarkMagenta
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
        let res = &res[1..];
        return match res.len() {
            3 => {
                let r = format!("{}{}", &res[0..1], &res[0..1]);
                let g = format!("{}{}", &res[1..2], &res[1..2]);
                let b = format!("{}{}", &res[2..3], &res[2..3]);
                let r = match hex::decode(r) {
                    Ok(r) => r[0],
                    Err(_) => return None,
                };
                let g = match hex::decode(g) {
                    Ok(g) => g[0],
                    Err(_) => return None,
                };
                let b = match hex::decode(b) {
                    Ok(b) => b[0],
                    Err(_) => return None,
                };
                Some(Color::from_rgb(r, g, b))
            },

            6 => match hex::decode(res) {
                    Ok(res) => Some(Color::from_rgb(res[0], res[1], res[2])),
                    Err(_) => None,
                },

            _ => None,
        }
    }

    match res.to_lowercase().as_str() {
        "black"       => Some(Color::BLACK),
        "red"         => Some(Color::RED),
        "green"       => Some(Color::GREEN),
        "yellow"      => Some(Color::YELLOW),
        "blue"        => Some(Color::BLUE),
        "magenta"     => Some(Color::from_rgb(0xff, 0, 0xff)),
        "cyan"        => Some(Color::from_rgb(0, 0xff, 0xff)),
        "darkred"     => Some(Color::DARK_RED),
        "darkgreen"   => Some(Color::DARK_GREEN),
        "darkyellow"  => Some(Color::GOLD),
        "darkblue"    => Some(Color::DARK_BLUE),
        "darkmagenta" => Some(Color::from_rgb(0x60, 0, 0x60)),
        "darkcyan"    => Some(Color::from_rgb(0, 0x60, 0x60)),
        "white"       => Some(Color::WHITE),

        _ => None,
    }
}


fn contrast(color: &Color) -> Color {
    let (mut r, mut g, mut b, _) = color.to_tuple();
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
    color.to_array().iter().take(3).fold(0_u8, |acc, &v| acc.max(v)) >= 0xa0
}
