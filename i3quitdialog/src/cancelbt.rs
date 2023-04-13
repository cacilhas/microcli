use eframe::{
    egui,
    egui::widgets::Button,
    epaint::Stroke,
};

use crate::resources::Resources;


pub fn create(resources: &Resources, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
    let cancel = Button::new("⎋ Cancel")
        .fill(resources.cancel_bg_color)
        .stroke(Stroke::new(0.0, resources.cancel_fg_color));

    if ui.add(cancel).clicked() {
        frame.close();
    }
}

