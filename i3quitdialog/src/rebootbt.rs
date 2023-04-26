use eframe::{egui, egui::widgets::Button};
use i3_ipc::{Connect, I3};

use crate::resources::Resources;

pub fn create(resources: &Resources, ui: &mut egui::Ui) {
    let text: egui::RichText = "Reboot".into();
    let reboot = Button::new(text.color(resources.reboot_fg_color))
        .fill(resources.reboot_bg_color)
        .min_size(eframe::epaint::Vec2 { x: 122.0, y: 24.0 });

    if ui.add(reboot).clicked() {
        let mut i3 = I3::connect().unwrap();
        i3.run_command("exec --no-startup-id reboot").unwrap();
    }
}
