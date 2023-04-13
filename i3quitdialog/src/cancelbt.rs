use eframe::{
    egui,
    egui::widgets::Button,
    epaint::Stroke,
};

use crate::resources::Resources;


pub fn create(resources: &Resources, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
    let cancel = Button::new("⎋ Cancel")
        .fill(resources.cancel_bg_color)
        .min_size(eframe::epaint::Vec2 { x: 112.0, y: 24.0 })
        .stroke(Stroke::new(0.0, resources.cancel_fg_color));

    if ui.add(cancel).clicked() {
        frame.close();
    }
}

