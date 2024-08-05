use chrono::{DateTime, Duration, Local};

use crate::{
    types::{AppState, Contacts},
    utils::{clean_symbols, generate_config},
};
use std::{
    fs::{self, File},
    io::{BufWriter, Write},
    path::{Path, PathBuf},
};

pub fn parse_content(app: &mut AppState, contacts_list: Contacts, file_raw: &mut File) {
    let start_time: DateTime<Local> = Local::now();
    let filters: Vec<String> = app
        .filters
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    let mut file = BufWriter::new(file_raw);

    let _ = file.write("<?xml version=\"1.0\" encoding=\"UTF-16\"?>".as_bytes());
    let _ = file.write(format!("<{0}>\n", app.field1).as_bytes());
    let _ = file.write(format!("    <{0}>\n", app.field2).as_bytes());

    let config = generate_config(app);
    let conf = config.kv_list;
    let empty_string = " ".to_string();

    // Loop
    for contacts in contacts_list {
        let _ = file.write(format!("        <{0}>\n", app.field3).as_bytes());

        // Enter Actuall information
        let mut company_block = Vec::new();
        let mut contact_block = Vec::new();

        for contact in contacts {
            // if filters.contains(&map_to_evc(contact.0.to_owned(), app)) {
            let cleaned_string = clean_symbols(contact.1);
            let tt = conf.clone();
            let t = tt.get(&contact.0);
            if filters.contains(&contact.0) {
                // Add values to company block
                let _ = writeln!(
                    &mut company_block,
                    "            <{0}>{1}</{0}>",
                    t.unwrap_or(&empty_string),
                    cleaned_string
                );
            } else {
                let _ = writeln!(
                    &mut contact_block,
                    "            <{0}>{1}</{0}>",
                    t.unwrap_or(&empty_string),
                    cleaned_string
                );
            }
        }

        let _ = file.write(format!("            <{}>\n", app.child_block).as_bytes());

        for company_data in company_block {
            let _ = file.write(std::slice::from_ref(&company_data));
        }
        let _ = file.write(format!("            </{}>\n", app.child_block).as_bytes());

        for contact_data in contact_block {
            let _ = file.write(std::slice::from_ref(&contact_data));
        }

        let _ = file.write(format!("        </{0}>\n", app.field3).as_bytes());
    }
    // Loop end

    let _ = file.write(format!("    </{0}>\n", app.field2).as_bytes());
    let _ = file.write(format!("</{0}>\n", app.field1).as_bytes());

    let end_time: DateTime<Local> = Local::now();
    let duration: Duration = end_time.signed_duration_since(start_time);

    println!("LOOP took {:?}", duration);
}

pub fn write_xml(
    app: &mut AppState,
    contacts_list: Contacts,
    file: &mut File,
) -> Result<String, String> {
    let bom = [0xEF, 0xBB, 0xBF];
    match file.write_all(&bom) {
        Ok(_) => {
            parse_content(app, contacts_list, file);
            Ok("Done".to_string())
        }
        Err(e) => Err(format!("Failed {e}")),
    }
}

pub fn generate_xml(app: &mut AppState, contacts_list: Contacts) -> Result<String, String> {
    let mut output = app.output.clone().unwrap();
    let output_name = PathBuf::from(app.out_file_name.clone());

    output.push(output_name);

    let start_time: DateTime<Local> = Local::now();
    if Path::exists(&output) {
        match fs::remove_file(&output) {
            Ok(_) => match fs::File::create_new(&output) {
                Ok(mut file) => {
                    let end_time: DateTime<Local> = Local::now();
                    let _ = write_xml(app, contacts_list, &mut file);

                    let duration: Duration = end_time.signed_duration_since(start_time);
                    Ok(format!("Finished in {:?} seconds and  {:?} nanoseconds", duration.num_seconds(), duration.num_nanoseconds().unwrap_or(0)))
                }
                Err(e) => Err(format!("Failed creating file {e}")),
            },
            Err(e) => Err(format!("Failed removing file {e}")),
        }
    } else {
        match fs::File::create_new(&output) {
            Ok(mut file) => {
                let end_time: DateTime<Local> = Local::now();
                let _ = write_xml(app, contacts_list, &mut file);

                let duration: Duration = end_time.signed_duration_since(start_time);
                Ok(format!("Finished in {:?} seconds and  {:?} nanoseconds", duration.num_seconds(), duration.num_nanoseconds()))
            }
            Err(e) => Err(format!("Failed creating file {e}")),
        }
    }
}
