use i3_ipc::{
    Connect,
    I3,
};
use eframe::{
    egui,
    egui::widgets::Button,
    epaint::Stroke,
};

use crate::resources::Resources;


pub fn create(resources: &Resources, ui: &mut egui::Ui) {
    let halt = Button::new("⏻ Halt")
        .fill(resources.halt_bg_color)
        .stroke(Stroke::new(0.0, resources.halt_fg_color));

    if ui.add(halt).clicked() {
        let mut i3 = I3::connect().unwrap();
        i3.run_command("exec --no-startup-id halt -p").unwrap();
    }
}
