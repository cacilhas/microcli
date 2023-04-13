use i3_ipc::{
    Connect,
    I3,
};
use eframe::{
    egui,
    egui::widgets::Button,
};

use crate::resources::Resources;


pub fn create(resources: &Resources, ui: &mut egui::Ui, force: bool) {
    let text: egui::RichText = "Exit".into();
    let exit = Button::new(
        text
            .color(resources.exit_fg_color)
            .size(16.0)
            .strong()
            .underline()
    )
        .fill(resources.exit_bg_color)
        .min_size(eframe::epaint::Vec2 { x: 122.0, y: 24.0 });

    if force || ui.add(exit).clicked() {
        let mut i3 = I3::connect().unwrap();
        i3.run_command("exit").unwrap();
    }
}

