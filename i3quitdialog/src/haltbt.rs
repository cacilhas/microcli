use i3_ipc::{
    Connect,
    I3,
};
use eframe::{
    egui,
    egui::widgets::Button,
};

use crate::resources::Resources;


pub fn create(resources: &Resources, ui: &mut egui::Ui) {
    let text: egui::RichText = "Halt".into();
    let halt = Button::new(text.color(resources.halt_fg_color))
        .fill(resources.halt_bg_color)
        .min_size(eframe::epaint::Vec2 { x: 122.0, y: 24.0 });

    if ui.add(halt).clicked() {
        let mut i3 = I3::connect().unwrap();
        i3.run_command("exec --no-startup-id halt -p").unwrap();
    }
}
