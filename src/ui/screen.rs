use egui::{Color32, CornerRadius, Margin, Stroke};

use crate::ui::Launcher;

impl Launcher {
    pub(crate) fn window_frame(&self) -> egui::Frame {
        egui::Frame::new()
            .fill(
                Color32::from_hex("#1a1a1a")
                    .expect("Failed to convert the hex color value to the color"),
            )
            .inner_margin(Margin::same(10))
            .stroke(Stroke::new(2.0, Color32::WHITE))
            .corner_radius(CornerRadius::same(20))
    }

    pub(crate) fn ui_elements(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                let search_bar = ui.text_edit_singleline(&mut self.query);
                if self.first_frame {
                    search_bar.request_focus();
                    self.first_frame = false;
                }
                let search_button = ui.button("Search");
                if search_bar.lost_focus() || search_button.clicked() {
                    if self.query != "".to_string() {
                        self.result = Some(format!("{}", self.query));
                        self.query = "".into();
                    }
                }
                if ui.button("Clear").clicked() {
                    self.result = None;
                    self.query = "".into();
                }
            });
            if let Some(result) = &self.result {
                ui.label(format!("You typed:\t{}", result));
            }
            self.app_scrollable_list(ui);

            self.handle_if_error_execution(ui);
        });
    }
}
