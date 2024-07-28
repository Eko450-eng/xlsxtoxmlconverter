#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
mod types;
mod utils;
mod xlsx_maniulation;
mod xml_manipulation;

use eframe::App;
use egui::{Align, Layout};
use egui_file::FileDialog;
use types::AppState;
use xlsx_maniulation::read_excel;
use xml_manipulation::generate_xml;

impl App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                ui.horizontal(|ui| {
                    ui.label("Sheet Name?");
                    ui.text_edit_singleline(&mut self.worksheet_name);
                });

                // Groups
                ui.horizontal(|ui| {
                    ui.label("First group - Data");
                    ui.text_edit_singleline(&mut self.field1);
                });
                ui.horizontal(|ui| {
                    ui.label("Second group - Contacts");
                    ui.text_edit_singleline(&mut self.field2);
                });
                ui.horizontal(|ui| {
                    ui.label("Third group - Contact");
                    ui.text_edit_singleline(&mut self.field3);
                });
                ui.horizontal(|ui| {
                    ui.label("Forth group - Company");
                    ui.text_edit_singleline(&mut self.child_block);
                });

                //Paths
                ui.horizontal(|ui| {
                    if (ui.button(format!(
                        "input :{}",
                        self.input.clone().unwrap().to_string_lossy()
                    )))
                    .clicked()
                    {
                        let mut dialog_input = FileDialog::open_file(self.input.clone());
                        dialog_input.open();
                        self.open_input_dialog = Some(dialog_input);
                    }
                    if let Some(dialog) = &mut self.open_input_dialog {
                        if dialog.show(ctx).selected() {
                            if let Some(file) = dialog.path() {
                                self.input = Some(file.to_path_buf());
                            }
                        }
                    }
                });

                ui.horizontal(|ui| {
                    if (ui.button(format!(
                        "Config :{}",
                        self.config_path.clone().unwrap().to_string_lossy()
                    )))
                    .clicked()
                    {
                        let mut dialog_config = FileDialog::open_file(self.config_path.clone());
                        dialog_config.open();
                        self.open_config_dialog = Some(dialog_config);
                    }
                    if let Some(dialog) = &mut self.open_config_dialog {
                        if dialog.show(ctx).selected() {
                            if let Some(file) = dialog.path() {
                                self.config_path = Some(file.to_path_buf());
                            }
                        }
                    }
                });

                ui.horizontal(|ui| {
                    if (ui.button(format!(
                        "output :{}",
                        self.output.clone().unwrap().to_string_lossy()
                    )))
                    .clicked()
                    {
                        let mut dialog_out = FileDialog::select_folder(self.output.clone());
                        dialog_out.open();
                        self.open_output_dialog = Some(dialog_out);
                    }
                    if let Some(dialog) = &mut self.open_output_dialog {
                        if dialog.show(ctx).selected() {
                            if let Some(file) = dialog.path() {
                                self.output = Some(file.to_path_buf());
                            }
                        }
                    }
                    ui.horizontal(|ui| {
                        ui.label("Output File Name");
                        ui.text_edit_singleline(&mut self.out_file_name);
                    });
                });

                let submit_btn = ui.button("Save and generate");

                if submit_btn.clicked() {
                    if let Ok(values) = read_excel(self) {
                        generate_xml(self, values)
                    }
                }
            });
        });
    }
}

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let config = AppState::default();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_app_id("Scout")
            .with_inner_size([1080.0, 720.0]),
        ..Default::default()
    };

    eframe::run_native("Scout", options, Box::new(|_cc| Ok(Box::new(config))))
}
