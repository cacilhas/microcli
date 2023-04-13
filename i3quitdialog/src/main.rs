mod resources;
mod users;

// Buttons
mod cancelbt;
mod exitbt;
mod haltbt;
mod rebootbt;

use crate::resources::Resources;
use crate::users::User;

use eframe::egui;


#[cfg(any(
    target_os = "dragonfly",
    target_os = "freebsd",
    target_os = "linux",
    target_os = "netbsd",
    target_os = "openbsd",
))]
fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(340.0, 75.0)),
        resizable: false,
        run_and_return: false,
        ..Default::default()
    };

    eframe::run_native(
        "i3 Dialog Quit",
        options,
        Box::new(|cc| Box::new(App::new(cc))),
    ).unwrap();
}


#[derive(Default)]
struct App {
    resources: Resources,
    user: User,
}


impl App {

    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let app = Self::default();
        let mut visuals = egui::Visuals::default();
        visuals.override_text_color = Some(app.resources.foreground);
        visuals.window_fill = app.resources.background;
        cc.egui_ctx.set_visuals(visuals);
        app
    }
}


impl eframe::App for App {

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Do you really want to exit i3?");

            if self.user.is_power_user() {
                ui.horizontal(|ui| {
                    haltbt::create(&self.resources, ui);
                    rebootbt::create(&self.resources, ui);
                });
            }

            ui.horizontal(|ui| {
                exitbt::create(&self.resources, ui);
                cancelbt::create(&self.resources, ui, frame);
            });
        });
    }
}
