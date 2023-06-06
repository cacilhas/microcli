//! A simple quit dialog for [i3wm](https://i3wm.org/).
//!
//! # Build and install
//!
//! ```sh
//! cargo install i3quitdialog
//! ```
//!
//! # Settings
//!
//! The “`Exit`” button works out of the box. In order to allow users to halt and
//! reboot the system, add them to the `power` group.
//!
//! You can just call it:
//!
//! ```sh
//! i3quitdialog
//! ```
//!
//! However, if you are setting a
//! [blocklet](https://vivien.github.io/i3blocks/#_i3blocks_properties) to launch
//! I3 Quit Dialog, prefer using `i3-msg`:
//!
//! ```ini
//! [Logoff]
//! command=i3-msg -q -- exec --no-startup-id i3quitdialog
//! color=#ff0000
//! full_text=⏻
//! text=⏻
//! interval=0
//! #signal=10  # uncomment this line to capture `pkill -SIGRTMIN+10 i3blocks`
//! ```
//!
//! ## `.XResources` support
//!
//! I3 Quit Dialog supports the following keys from `xrdb` (case sensitive):
//!
//! - `i3quitdialog.Title.foreground`: window colour
//! - `i3quitdialog.Title.background`: window background colour
//! - `i3quitdialog.ExitButton.color`: exit button colour
//! - `i3quitdialog.CancelButton.color`: cancel button colour
//! - `i3quitdialog.HaltButton.color`: halt button colour
//! - `i3quitdialog.RebootButton.color`: halt button colour
//!
//! Acceptable values:
//!
//! - Hexadecimal RGB (`#0000ff`)
//! - One of the following strings:
//!     - `Black`
//!     - `Blue`
//!     - `DarkBlue`
//!     - `Cyan`
//!     - `DarkCyan`
//!     - `Green`
//!     - `DarkGreen`
//!     - `Magenta`
//!     - `DarkMagenta`
//!     - `Red`
//!     - `DarkRed`
//!     - `White`
//!     - `Yellow`
//!     - `DarkYellow`

#![allow(non_camel_case_types)]

#[macro_use]
extern crate static_init;

mod resources;
mod users;

// Buttons
mod cancelbt;
mod exitbt;
mod haltbt;
mod rebootbt;

use crate::resources::Resources;
use crate::users::User;

use eframe::{
    egui,
    egui::{FontData, FontDefinitions},
    epaint::FontFamily,
};

#[cfg(any(
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "linux",
    target_os = "netbsd",
    target_os = "openbsd",
))]
fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(268.0, 96.0)),
        resizable: false,
        run_and_return: false,
        ..Default::default()
    };

    eframe::run_native(
        "i3 Dialog Quit",
        options,
        Box::new(|cc| Box::new(App::new(cc))),
    )
    .unwrap();
}

#[derive(Default)]
struct App {
    resources: Resources,
    user: User,
}

impl App {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let app = Self::default();
        app.init(cc);
        app
    }

    fn init(&self, cc: &eframe::CreationContext<'_>) {
        let visuals = egui::Visuals {
            override_text_color: Some(self.resources.foreground),
            panel_fill: self.resources.background,
            ..Default::default()
        };
        cc.egui_ctx.set_visuals(visuals);

        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert("bellota".into(), BELLOTA.to_owned());
        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "bellota".into());
        cc.egui_ctx.set_fonts(fonts);
    }

    fn pressed_keys(ctx: &egui::Context) -> PressedKeys {
        let mut escape = false;
        let mut enter = false;
        let events = ctx.input(|input| input.events.to_owned());
        for event in events.iter() {
            match event {
                #[allow(unused_variables)]
                egui::Event::Key {
                    key,
                    pressed,
                    repeat,
                    modifiers,
                } if *key == egui::Key::Escape => escape = *pressed,
                #[allow(unused_variables)]
                egui::Event::Key {
                    key,
                    pressed,
                    repeat,
                    modifiers,
                } if *key == egui::Key::Enter => enter = *pressed,
                _ => (),
            };
        }

        PressedKeys { enter, escape }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        frame.set_centered();
        frame.set_always_on_top(true);
        let keys = App::pressed_keys(ctx);
        if keys.escape {
            return frame.close();
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Do you really want to exit i3?");

            if self.user.is_power_user() {
                ui.horizontal(|ui| {
                    haltbt::create(&self.resources, ui);
                    rebootbt::create(&self.resources, ui);
                });
            }

            ui.horizontal(|ui| {
                exitbt::create(&self.resources, ui, frame, keys.enter);
                cancelbt::create(&self.resources, ui, frame);
            });
        });
    }
}

struct PressedKeys {
    enter: bool,
    escape: bool,
}

#[dynamic]
static BELLOTA: FontData = {
    let font = include_bytes!("assets/bellota.ttf");
    FontData::from_static(font)
};
