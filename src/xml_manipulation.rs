use chrono::{DateTime, Duration, Local};

use crate::{
    types::{AppState, Contacts},
    utils::{clean_symbols, map_to_evc},
};
use std::{
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

#[allow(unused)]
fn create_tag(app: &mut AppState, key: String, value: String, indents: i16) -> String {
    let evc_key = map_to_evc(key, app);
    let mut indent_string = vec![];
    for _ in 0..indents {
        indent_string.push("    ")
    }

    let mut r = vec![];

    r.push(format!("{}<{}>", indent_string.join(" "), evc_key));
    r.push(format!("<{}>", value));
    r.push(format!("</{}>", evc_key));

    r.join("")
}

pub fn parse_content(app: &mut AppState, contacts_list: Contacts, file: &mut File) {
    let start_time: DateTime<Local> = Local::now();
    let filters: Vec<String> = app
        .filters
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    let _ = file.write("<?xml version=\"1.0\" encoding=\"UTF-16\"?>".as_bytes());
    let _ = file.write(format!("<{0}>\n", app.field1).as_bytes());
    let _ = file.write(format!("    <{0}>\n", app.field2).as_bytes());

    // Loop
    for contacts in contacts_list {
        let _ = file.write(format!("        <{0}>\n", app.field3).as_bytes());

        // Enter Actuall information
        let mut company_block: Vec<String> = vec![];
        let mut contact_block: Vec<String> = vec![];

        for contact in contacts {
            if filters.contains(&map_to_evc(contact.0.to_owned(), app)) {
                // Add values to company block
                let cleaned_string = clean_symbols(contact.1);
                company_block.push(format!(
                    "              <{0}>{1}</{0}>\n",
                    map_to_evc(contact.0, app),
                    cleaned_string
                ));
            } else {
                let cleaned_string = clean_symbols(contact.1);
                contact_block.push(format!(
                    "            <{0}>{1}</{0}>\n",
                    map_to_evc(contact.0, app),
                    cleaned_string
                ))
            }
        }

        let _ = file.write(format!("            <{}>\n", app.child_block).as_bytes());

        for company_data in company_block {
            let _ = file.write(company_data.as_bytes());
        }
        let _ = file.write(format!("            </{}>\n", app.child_block).as_bytes());

        for contact_data in contact_block {
            let _ = file.write(contact_data.as_bytes());
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
    let start_time: DateTime<Local> = Local::now();
    let mut output = app.output.clone().unwrap();
    let output_name = PathBuf::from(app.out_file_name.clone());

    output.push(output_name);

    if Path::exists(&output) {
        match fs::remove_file(&output) {
            Ok(_) => match fs::File::create_new(&output) {
                Ok(mut file) => {
                    write_xml(app, contacts_list, &mut file);
                    Ok("Success".to_string())
                }
                Err(e) => Err(format!("Failed creating file {e}")),
            },
            Err(e) => Err(format!("Failed removing file {e}")),
        }
    } else {
        match fs::File::create_new(&output) {
            Ok(mut file) => {
                write_xml(app, contacts_list, &mut file);
                Ok("Success".to_string())
            }
            Err(e) => Err(format!("Failed creating file {e}")),
        }
    }
}
