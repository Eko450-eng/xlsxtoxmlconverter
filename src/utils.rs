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
    let res = input.replace("&", "&amp;");
    let res1 = res.replace(">", "&gt;");
    let res2 = res1.replace("<", "&lt;");
    let res3 = res2.replace("\"", "&quot;");
    let res4 = res3.replace("²", "&#178;");
    let res5 = res4.replace("³", "&#179;");
    let res6 = res5.replace("ß", "&#223;");
    let res7 = res6.replace("Ä", "&#196;");
    let res8 = res7.replace("Ö", "&#246;");
    let res9 = res8.replace("Ü", "&#220;");
    let res10 = res9.replace("ä", "&#228;");
    let res11 = res10.replace("ö", "&#246;");
    let res12 = res11.replace("ü", "&#252;");
    let res13 = res12.replace("'", "&apos;");
    let res14 = res13.replace("°", "&#176;");
    let res15 = res14.replace("®", "&#174;");
    let res16 = res15.replace("©", "&#169;");
    let res17 = res16.replace("–", "&173;");
    let res18 = res17.replace("é", "&#233;");
    let res19 = res18.replace("è", "&#232;");
    let res20 = res19.replace("á", "&#225;");
    let res21 = res20.replace("à", "&#224;");
    let res22 = res21.replace("´", "&#180;");
    let res23 = res22.replace("Ç", "&#199;");
    let res24 = res23.replace("ç", "&#231;");
    let res25 = res24.replace("»", "&187;");
    let res26 = res25.replace("«", "&#171;");
    let res27 = res26.replace("Ž", "&#381;");
    let res28 = res27.replace("ž", "&#392;");
    let res29 = res28.replace("æ", "&230;");
    let res30 = res29.replace("Ý", "&#221;");
    let res31 = res30.replace("ý", "&#253;");
    let res32 = res31.replace("ÿ", "&#255;");
    let res33 = res32.replace("Ÿ", "&#376;");
    let res34 = res33.replace("Ó", "&#211;");
    let res35 = res34.replace("ó", "&#243;");
    let res36 = res35.replace("Ò", "&#210;");
    res36.replace("ó", "&#242;")
}







