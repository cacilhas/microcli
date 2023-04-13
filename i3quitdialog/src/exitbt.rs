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
    let exit = Button::new("↩ Exit")
        .fill(resources.exit_bg_color)
        .stroke(Stroke::new(0.0, resources.exit_fg_color));

    if ui.add(exit).clicked() {
        let mut i3 = I3::connect().unwrap();
        i3.run_command("exit").unwrap();
    }
}

