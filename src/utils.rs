use std::{collections::HashMap, fs::File, io::{BufRead, BufReader}};

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
