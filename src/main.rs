#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
mod types;
mod utils;
mod xlsx_maniulation;
mod xml_manipulation;

use std::fs;

use eframe::App;
use egui::{Align, Align2, Direction, Key, Layout, Modifiers};
use egui_code_editor::{CodeEditor, Syntax};
use egui_file::FileDialog;
use egui_toast::{Toast, ToastKind, ToastOptions, Toasts};
use types::AppState;
use utils::generate_defaults;
use xlsx_maniulation::read_excel;
use xml_manipulation::generate_xml;

impl App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut toast = Toasts::new()
            .anchor(Align2::RIGHT_TOP, (-10.0, -10.0))
            .direction(Direction::BottomUp);
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(Layout::top_down(Align::Center), |ui| {
                ui.horizontal(|ui| {
                    ui.label("Sheet Name?")
                    .on_hover_ui(|ui| {
                        ui.label("The name of the Excel sheet that contains the data");
                    });
                    ui.text_edit_singleline(&mut self.worksheet_name);
                });

                // Groups
                ui.horizontal(|ui| {
                    ui.label("First group - Data")
                    .on_hover_ui(|ui| {
                        ui.label("The Parent box in the EVC this is typically <data>");
                    });
                    ui.text_edit_singleline(&mut self.field1);
                });
                ui.horizontal(|ui| {
                    ui.label("Second group - Contacts")
                    .on_hover_ui(|ui| {
                        ui.label("The Parent box in the EVC this is typically <contacts>");
                    });
                    ui.text_edit_singleline(&mut self.field2);
                });
                ui.horizontal(|ui| {
                    ui.label("Third group - Contact")
                    .on_hover_ui(|ui| {
                        ui.label("The Parent box in the EVC this is typically <contact>");
                    });
                    ui.text_edit_singleline(&mut self.field3);
                });
                ui.horizontal(|ui| {
                    ui.label("Forth group - Company")
                    .on_hover_ui(|ui| {
                        ui.label("The Parent box in the EVC this is typically <company> and contains the companyNumber and companyName");
                    });
                    ui.text_edit_singleline(&mut self.child_block);
                });

                ui.horizontal(|ui| {
                    ui.label("Filters for Fourth group")
                    .on_hover_ui(|ui| {
                        ui.label("Set which fields should be put inside the fourth group");
                        ui.label("Seperated with ','");
                        ui.label("Will use mapped Values - not the names set inside the XLSX");
                    });
                    ui.text_edit_singleline(&mut self.filters);
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
                    if ui
                        .button(format!(
                            "Config :{}",
                            self.config_path.clone().unwrap().to_string_lossy()
                        ))
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


                    if ui.button("Edit config").clicked() {
                        if let Some(config_path) = &self.config_path {
                            if config_path.is_file() {
                                self.show_editor = true;
                            } else {
                                toast.add(Toast {
                                    text: "No Config selected!".into(),
                                    kind: ToastKind::Error,
                                    options: ToastOptions::default()
                                        .duration_in_seconds(2.0)
                                        .show_progress(true),
                                    ..Default::default()
                                });
                                self.show_editor = false;
                            }
                            if let Ok(c) = fs::read_to_string(config_path) {
                                self.config_content = c;
                            };
                        }
                    }

                    if ui.button("Create default config at .../Documents/evc").clicked(){
                        match generate_defaults(self) {
                            Ok(())=>println!("Createdd"),
                            Err(e) => eprintln!("{e}"),
                        };
                    };
                });

                toast.show(ctx);

                if self.show_editor {
                    if ui.input(|i| i.to_owned().consume_key(Modifiers::CTRL, Key::S)) {
                        match fs::write(
                            self.config_path.clone().unwrap(),
                            self.config_content.clone(),
                        ) {
                            Ok(_) => {}
                            Err(e) => {
                                println!("Saving failed: {:?}", e)
                            }
                        }
                    };

                    CodeEditor::default()
                        .id_source("code_editor")
                        .with_rows(12)
                        .with_fontsize(12.0)
                        .with_syntax(Syntax::default())
                        .with_numlines(true)
                        .show(ui, &mut self.config_content);
                }

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
            egui::TopBottomPanel::top("menu_bar").show_inside(ui, |ui| {
                ui.label("Ctrl-S to save in editor");
                ui.label("Hover over Text for more information");
            });
        });
    }
}

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let config = AppState::default();
    let icon_bytes: &[u8] = include_bytes!("resources/icon.png");
    let icon_data = eframe::icon_data::from_png_bytes(icon_bytes);

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_icon(icon_data.unwrap())
            .with_app_id("xtox")
            .with_inner_size([1080.0, 720.0]),
        ..Default::default()
    };

    eframe::run_native("xtox", options, Box::new(|_cc| Ok(Box::new(config))))
}
