use egui_file::FileDialog;
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;

pub struct AppState {
    pub field1: String,
    pub field2: String,
    pub field3: String,

    pub child_block: String,
    pub filters: String,

    pub out_file_name: String,

    pub output: Option<PathBuf>,
    pub input: Option<PathBuf>,
    pub config_path: Option<PathBuf>,

    pub worksheet_name: String,

    pub open_input_dialog: Option<FileDialog>,
    pub open_config_dialog: Option<FileDialog>,
    pub open_output_dialog: Option<FileDialog>,
}

pub type Contacts = Vec<HashMap<String, String>>;

impl Default for AppState {
    fn default() -> Self {
        let default_document_path: PathBuf = if std::env::consts::OS == "windows" {
            PathBuf::from("C:\\\\")
        } else {
            let mut path = PathBuf::new();
            let home = env::var("HOME").unwrap();
            path.push(home);
            path.push("Documents");
            path
        };
        Self {
            out_file_name: "CONTACTS.xml".to_string(),
            field1: "data".to_string(),
            field2: "contacts".to_string(),
            field3: "contact".to_string(),
            child_block: "company".to_string(),
            filters: "companyCode, companyName".to_string(),
            output: Some(default_document_path.clone()),
            input: Some(default_document_path.clone()),
            config_path: Some(default_document_path),
            worksheet_name: "Daten".to_string(),
            open_input_dialog: None,
            open_config_dialog: None,
            open_output_dialog: None,
        }
    }
}
