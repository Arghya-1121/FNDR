use eframe::App;
use egui::{
    Color32, CornerRadius, Id, LayerId, Margin, Popup, PopupAnchor, ScrollArea, Stroke,
    scroll_area::ScrollAreaOutput,
};

use crate::service::{
    app_launch::app_exec,
    search::{Application_Database, Applications},
};

pub struct Launcher {
    query: String,
    result: Option<String>,
    first_frame: bool,
    app_data: Vec<Applications>,
    error_exec: Option<(String, String)>,
}

impl Default for Launcher {
    fn default() -> Self {
        Self {
            query: String::new(),
            result: None,
            first_frame: true,
            app_data: Application_Database::app_data(),
            error_exec: None,
        }
    }
}

impl Launcher {
    fn window_frame(&self) -> egui::Frame {
        egui::Frame::new()
            .fill(
                Color32::from_hex("#1a1a1a")
                    .expect("Failed to convert the hex color value to the color"),
            )
            .inner_margin(Margin::same(10))
            .stroke(Stroke::new(2.0, Color32::WHITE))
            .corner_radius(CornerRadius::same(20))
    }

    fn ui_elements(&mut self, ui: &mut egui::Ui) {
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

    fn app_scrollable_list(&mut self, ui: &mut egui::Ui) -> ScrollAreaOutput<()> {
        ScrollArea::vertical().show(ui, |ui| {
            for app in &self.app_data {
                if ui.button(&app.name).clicked() {
                    let error_exec = app_exec(&app).to_string();
                    if !error_exec.trim().is_empty() {
                        self.error_exec = Some((app.name.clone(), error_exec));
                    } else {
                        ui.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                };
                ui.add_space(10.0);
            }
        })
    }

    fn handle_if_error_execution(&mut self, ui: &mut egui::Ui) {
        if let Some(error_exec) = &self.error_exec {
            let error_popup = Popup::new(
                "com.kolkode.fndr.error".into(),
                ui.ctx().clone(),
                PopupAnchor::PointerFixed,
                LayerId {
                    order: egui::Order::Foreground,
                    id: Id::new("foreground"),
                },
            );

            error_popup.show(|ui| {
                ui.heading(format!(
                    "There are some error occurred while executing: {}",
                    error_exec.0
                ));
                ui.label(&error_exec.1);
                if ui.button("Close").clicked() {
                    ui.send_viewport_cmd(egui::ViewportCommand::Close);
                }
            });
        };
    }
}

impl App for Launcher {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(self.window_frame())
            .show(ui, |ui| self.ui_elements(ui));
    }
}
