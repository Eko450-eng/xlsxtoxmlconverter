#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::{
    collections::HashMap,
    fs::{self, File},
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
};

use calamine::{open_workbook, Reader, Xlsx};
use eframe::App;
use egui::Ui;

//

struct AppState {
    field1: String,
    field2: String,
    field3: String,
    child_block: String,
    filters: String,
    output: String,
    input: String,
}

#[derive(Debug)]
struct Contacts {
    header: Vec<String>,
    contact: Vec<HashMap<String, String>>,
}

fn read_excel(app: &mut AppState) -> Result<Contacts, Box<dyn std::error::Error>> {
    let path = app.input.clone();
    let mut workbook: Xlsx<_> = open_workbook(path)?;
    let range = workbook.worksheet_range("Daten").unwrap();
    let mut headers: Vec<String> = vec![];
    let mut contacts: Vec<HashMap<String, String>> = vec![];

    if let Some(first_row) = range.rows().next() {
        for (i, row) in range.rows().enumerate() {
            if i == 0 {
                headers = row.iter().map(|cell| cell.to_string()).collect();
            } else {
                // let contact: Vec<String> = row.iter().map(|cell| cell.to_string()).collect();
                let mut row_data: HashMap<String, String> = HashMap::new();
                for (header, cell) in first_row.iter().zip(row.iter()) {
                    row_data.insert(header.to_string(), cell.to_string());
                }
                contacts.push(row_data);
            }
        }
    }

    Ok(Contacts {
        header: headers,
        contact: contacts,
    })
}

fn generate_config() -> Vec<HashMap<String, String>> {
    let mut kv_list: Vec<HashMap<String, String>> = vec![];

    if let Ok(file) = File::open(PathBuf::from("/home/eko/code/apps/evcconvert/src/config")) {
        let config_file = BufReader::new(file);
        for line in config_file.lines() {
            let mut map: HashMap<String, String> = HashMap::new();
            if let Ok(line) = line {
                let parts: Vec<&str> = line.split('=').collect();

                if parts.len() == 2 {
                    let key = parts[0].trim();
                    let value = parts[1].trim();
                    map.insert(key.to_string(), value.to_string());
                    kv_list.push(map);
                }
            }
        }
    }
    kv_list
}

fn map_to_evc(input_value: String) -> String {
    let map = generate_config();
    let mut rv = "NO_VALUE_MAPPED_IN_CONFIG".to_string();

    for i in map {
        if let Some(value) = i.get(&input_value) {
            rv = value.to_string()
        }
    }
    rv
}

fn generate_xml(app: &mut AppState, _ui: &mut Ui, contacts_list: Contacts) {
    let output = PathBuf::from(app.output.clone());
    let mut buf: Vec<String> = vec!["<?xml version=\"1.0\" encoding=\"UTF-16\"?>".to_string()];
    let filters: Vec<String> = app
        .filters
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    let field1_opening = format!("<{0}>", app.field1);
    let field1_closing = format!("</{0}>", app.field1);
    let field2_opening = format!("    <{0}>", app.field2);
    let field2_closing = format!("    </{0}>", app.field2);
    let field3_opening = format!("        <{0}>", app.field3);
    let field3_closing = format!("        </{0}>", app.field3);

    buf.push(field1_opening);
    buf.push(field2_opening);

    // Loop
    for contacts in contacts_list.contact {
        buf.push(field3_opening.clone());

        // Enter Actuall information
        let mut company_block: Vec<String> = vec![];
        let mut contact_block: Vec<String> = vec![];
        for contact in contacts {
            if filters.contains(&map_to_evc(contact.0.clone())) {
                // Add values to company block
                company_block.push(format!(
                    "              <{0}>{1}</{0}>",
                    map_to_evc(contact.0),
                    contact.1
                ));
            } else {
                contact_block.push(format!(
                    "            <{0}>{1}</{0}>",
                    map_to_evc(contact.0),
                    contact.1
                ))
            }
        }

        buf.push(format!("            <{}>", app.child_block));
        for company_data in company_block {
            buf.push(company_data);
        }
        buf.push(format!("            </{}>", app.child_block));

        for contact_data in contact_block {
            buf.push(contact_data);
        }

        buf.push(field3_closing.clone());
    }
    // Loop end
    buf.push(field2_closing);
    buf.push(field1_closing);

    // if Path::new(&output).exists() {
    //     let _ = fs::remove_file(&output);
    // };

    match fs::File::create_new(output) {
        Ok(mut file) => match file.write_all(buf.join("\n").as_bytes()) {
            Ok(f) => println!("Success {:?}", f),
            Err(e) => println!("Failed {e}"),
        },
        Err(err) => {
            println!("Failed because {err}");
        }
    }
}

impl App for AppState {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.label("First Field");
                ui.text_edit_singleline(&mut self.field1);
            });
            ui.horizontal(|ui| {
                ui.label("Child Field");
                ui.text_edit_singleline(&mut self.field2);
            });
            ui.horizontal(|ui| {
                ui.label("Next child Field");
                ui.text_edit_singleline(&mut self.field3);
            });
            ui.horizontal(|ui| {
                ui.label("Filtered Child");
                ui.text_edit_singleline(&mut self.child_block);
            });
            ui.horizontal(|ui| {
                ui.label("Filters (Seperated by comma)");
                ui.text_edit_singleline(&mut self.filters);
            });
            ui.horizontal(|ui| {
                ui.label("Output directory");
                ui.text_edit_singleline(&mut self.output);
            });
            ui.horizontal(|ui| {
                ui.label("Input directory");
                ui.text_edit_singleline(&mut self.input);
            });
            let submit_btn = ui.button("Save and generate");

            if submit_btn.clicked() {
                if let Ok(values) = read_excel(self) {
                    generate_xml(self, ui, values)
                }
            }
        });
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            field1: "data".to_string(),
            field2: "contacts".to_string(),
            field3: "contact".to_string(),
            child_block: "company".to_string(),
            filters: "companyCode, companyName".to_string(),
            output: "CONTACTS_XML.xml".to_string(),
            input: "/home/eko/Downloads/EVIATEC_EVC_Kontakte2XML.xlsx".to_string(),
        }
    }
}

fn _read_config(_app: AppState) {}

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let config = AppState::default();

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_app_id("Scout")
            .with_inner_size([600.0, 600.0]),
        ..Default::default()
    };

    eframe::run_native("Scout", options, Box::new(|_cc| Ok(Box::new(config))))
}
