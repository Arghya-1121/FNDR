pub mod screen;
use eframe::App;
use egui::{Id, LayerId, Popup, PopupAnchor, ScrollArea, scroll_area::ScrollAreaOutput};

use crate::service::{
    app_launch::app_exec,
    search::{ApplicationDatabase, Applications},
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
            app_data: ApplicationDatabase::app_data(),
            error_exec: None,
        }
    }
}

impl App for Launcher {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default()
            .frame(self.window_frame())
            .show(ui, |ui| self.ui_elements(ui));
    }
}

impl Launcher {
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
