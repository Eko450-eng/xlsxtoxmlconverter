use std::{
    collections::HashMap,
    env,
    fs::{self, File},
    io::{BufRead, BufReader},
    path::PathBuf,
};

use crate::types::AppState;

#[derive(Clone)]
pub struct TemplateConfig {
    pub kv_list: HashMap<String, String>,
    pub input: PathBuf,
    pub output: PathBuf,
    pub name: String,
    pub first_group: String,
    pub second_group: String,
    pub third_group: String,
    pub fourth_group: String,
    pub filters: String,
    pub sheet: String,
}

pub fn generate_config(app: &mut AppState) -> TemplateConfig {
    let mut kv_list: HashMap<String, String> = HashMap::new();
    let config_file = app.config_path.clone().unwrap();
    let mut config: TemplateConfig = TemplateConfig {
        kv_list: HashMap::new(),
        input: PathBuf::new(),
        output: PathBuf::new(),
        name: String::new(),
        first_group: String::new(),
        second_group: String::new(),
        third_group: String::new(),
        fourth_group: String::new(),
        filters: String::new(),
        sheet: String::new(),
    };

    if let Ok(file) = File::open(config_file) {
        let config_file = BufReader::new(file);
        for line in config_file.lines().map_while(Result::ok) {
            if line.starts_with(':') {
                if line.starts_with(":sheet") {
                    let parts: Vec<&str> = line.split('=').collect();
                    config.sheet = parts[1].trim().to_string();
                }
                if line.starts_with(":in") {
                    let parts: Vec<&str> = line.split('=').collect();
                    config.input = PathBuf::from(parts[1].trim());
                }
                if line.starts_with(":out") {
                    let parts: Vec<&str> = line.split('=').collect();
                    config.output = PathBuf::from(parts[1].trim());
                }
                if line.starts_with(":name") {
                    let parts: Vec<&str> = line.split('=').collect();
                    config.name = parts[1].trim().to_string();
                }
                if line.starts_with(":first") {
                    let parts: Vec<&str> = line.split('=').collect();
                    config.first_group = parts[1].trim().to_string();
                }
                if line.starts_with(":second") {
                    let parts: Vec<&str> = line.split('=').collect();
                    config.second_group = parts[1].trim().to_string();
                }
                if line.starts_with(":third") {
                    let parts: Vec<&str> = line.split('=').collect();
                    config.third_group = parts[1].trim().to_string();
                }
                if line.starts_with(":fourth") {
                    let parts: Vec<&str> = line.split('=').collect();
                    config.fourth_group = parts[1].trim().to_string();
                }
                if line.starts_with(":filters") {
                    let parts: Vec<&str> = line.split('=').collect();
                    config.filters = parts[1].trim().to_string();
                }
            } else {
                let parts: Vec<&str> = line.split('=').collect();

                if parts.len() == 2 {
                    let key = parts[0].trim();
                    let value = parts[1].trim();
                    kv_list.insert(key.to_string(), value.to_string());
                }
            }
        }
    }

    config 
}

pub fn generate_defaults(app: &mut AppState) -> Result<(), String> {
    // Create File structure
    let mut path = get_default_documents_path();
    path.push("evc");

    let default_config_text = "Mandant Nr. = companyCode
        Mandant Name = companyName
        Typ = contactType
        Kontakt Nr. = contactNumber
        Kreditoren Nr. = contactVendorNumber
        Kunden Nr. = contactCustomerNumber
        Name 1 = contactNameA
        Name 2 = contactNameB
        Name 3 = contactNameC
        Strasse = contactAddressStreet
        PLZ = contactAddressZipCode
        Ort = contactAddressCity
        Land = contactAddressCountry
        Land Kürzel = contactAddressCountryShort
        Telefon = contactTelephoneNumber
        Fax = contactFaxNumber
        Email = contactEmail
        BLZ = contactBankSWIFT
        Institut = contactBank
        BIC = contactBankSWIFT
        IBAN = contactBankIBAN
        ILN = contactILN
        Steuer Nr. = contactTAXidNumber
        Ust. ID = contactVATidNumber
        Matchcode = contactMatchcode";

    match fs::create_dir_all(path.clone()) {
        Ok(()) => {
            let mut file_path = PathBuf::new();
            file_path.push(path.clone());
            path.push("config");
            match fs::write(path.clone(), default_config_text) {
                Ok(_) => {
                    app.config_path = Some(path);
                    Ok(())
                }
                Err(e) => Err(format!("Failed creating the default_config due to {e}")),
            }
        }
        Err(e) => Err(format!("Failed creating default file Structure due to {e}")),
    }
}

pub fn get_default_documents_path() -> PathBuf {
    if std::env::consts::OS == "windows" {
        // PathBuf::from("C:\\\\")
        let mut path = PathBuf::new();
        let home = env::var("USERPROFILE").unwrap();
        path.push(home);
        path.push("Documents");
        path
    } else {
        let mut path = PathBuf::new();
        let home = env::var("HOME").unwrap();
        path.push(home);
        path.push("Documents");
        path
    }
}

pub fn clean_symbols(input: String) -> String {
    #[allow(unused)]
    let mut result = String::new();
    result = input.replace("&", "&amp;");
    result = result.replace(">", "&gt;");
    result = result.replace("<", "&lt;");
    result = result.replace("\"", "&quot;");
    result = result.replace("²", "&#178;");
    result = result.replace("³", "&#179;");
    result = result.replace("ß", "&#223;");
    result = result.replace("Ä", "&#196;");
    result = result.replace("Ö", "&#246;");
    result = result.replace("Ü", "&#220;");
    result = result.replace("ä", "&#228;");
    result = result.replace("ö", "&#246;");
    result = result.replace("ü", "&#252;");
    result = result.replace("'", "&apos;");
    result = result.replace("°", "&#176;");
    result = result.replace("®", "&#174;");
    result = result.replace("©", "&#169;");
    result = result.replace("–", "&173;");
    result = result.replace("é", "&#233;");
    result = result.replace("è", "&#232;");
    result = result.replace("á", "&#225;");
    result = result.replace("à", "&#224;");
    result = result.replace("´", "&#180;");
    result = result.replace("Ç", "&#199;");
    result = result.replace("ç", "&#231;");
    result = result.replace("»", "&187;");
    result = result.replace("«", "&#171;");
    result = result.replace("Ž", "&#381;");
    result = result.replace("ž", "&#392;");
    result = result.replace("æ", "&230;");
    result = result.replace("Ý", "&#221;");
    result = result.replace("ý", "&#253;");
    result = result.replace("ÿ", "&#255;");
    result = result.replace("Ÿ", "&#376;");
    result = result.replace("Ó", "&#211;");
    result = result.replace("ó", "&#243;");
    result = result.replace("Ò", "&#210;");
    result = result.replace("ó", "&#242;");
    result
}
