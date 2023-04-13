use eframe::{
    egui,
    egui::widgets::Button,
};

use crate::resources::Resources;


pub fn create(resources: &Resources, ui: &mut egui::Ui, frame: &mut eframe::Frame) {
    let text: egui::RichText = "Cancel".into();
    let cancel = Button::new(text.color(resources.cancel_fg_color))
        .fill(resources.cancel_bg_color)
        .min_size(eframe::epaint::Vec2 { x: 122.0, y: 24.0 });

    if ui.add(cancel).clicked() {
        frame.close();
    }
}

