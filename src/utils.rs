use std::{
    collections::HashMap,
    env,
    error::Error,
    fs::{self, File},
    io::{BufRead, BufReader},
    path::PathBuf,
};

use crate::types::AppState;

pub fn generate_config(app: &mut AppState) -> Vec<HashMap<String, String>> {
    let mut kv_list: Vec<HashMap<String, String>> = vec![];
    let config_file = app.config_path.clone().unwrap();

    if let Ok(file) = File::open(config_file) {
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

pub fn map_to_evc(input_value: String, app: &mut AppState) -> String {
    let map = generate_config(app);
    let mut rv = "NO_VALUE_MAPPED_IN_CONFIG".to_string();

    for i in map {
        if let Some(value) = i.get(&input_value) {
            rv = value.to_string()
        }
    }
    rv
}

pub fn generate_defaults(app: &mut AppState) -> Result<(), String> {
    // Create File structure
    let mut path = get_default_documents_path();
    path.push("evc");

    let default_config_text="Mandant Nr. = companyCode
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
