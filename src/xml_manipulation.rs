use crate::{
    types::{AppState, Contacts},
    utils::{clean_symbols, map_to_evc},
};
use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
};

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

pub fn generate_xml_new(app: &mut AppState, contacts_list: Contacts) {
    // Genreate Header
    let mut output = app.output.clone().unwrap();
    let output_name = PathBuf::from(app.out_file_name.clone());
    output.push(output_name);

    let mut buf: Vec<String> = vec!["<?xml version=\"1.0\" encoding=\"UTF-16\"?>".to_string()];
    let filters: Vec<String> = app
        .filters
        .split(',')
        .map(|s| s.trim().to_string())
        .collect();

    // Create main Structure
    let field1_opening = format!("<{0}>", app.field1);
    let field1_closing = format!("</{0}>", app.field1);
    let field2_opening = format!("    <{0}>", app.field2);
    let field2_closing = format!("    </{0}>", app.field2);
    let field3_opening = format!("        <{0}>", app.field3);
    let field3_closing = format!("        </{0}>", app.field3);

    buf.push(field1_opening);
    buf.push(field2_opening);

    // Loop
    for contacts in contacts_list {
        buf.push(field3_opening.clone());

        // Enter Actuall information
        let mut company_block: Vec<String> = vec![];
        let mut contact_block: Vec<String> = vec![];
        for contact in contacts {
            if filters.contains(&map_to_evc(contact.0.clone(), app)) {
                // Add values to company block
                company_block.push(create_tag(app, contact.0, clean_symbols(contact.1), 3));
                // company_block.push(format!(
                //     "              <{0}>{1}</{0}>",
                //     map_to_evc(contact.0, app),
                //     contact.1
                // ));
            } else {
                contact_block.push(create_tag(app, contact.0, clean_symbols(contact.1), 3));
                // contact_block.push(format!(
                //     "            <{0}>{1}</{0}>",
                //     map_to_evc(contact.0, app),
                //     contact.1
                // ))
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
}

pub fn generate_xml(app: &mut AppState, contacts_list: Contacts) {
    let mut output = app.output.clone().unwrap();
    let output_name = PathBuf::from(app.out_file_name.clone());

    output.push(output_name);

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
    for contacts in contacts_list {
        buf.push(field3_opening.clone());

        // Enter Actuall information
        let mut company_block: Vec<String> = vec![];
        let mut contact_block: Vec<String> = vec![];
        for contact in contacts {
            if filters.contains(&map_to_evc(contact.0.clone(), app)) {
                // Add values to company block
                company_block.push(format!(
                    "              <{0}>{1}</{0}>",
                    map_to_evc(contact.0, app),
                    clean_symbols(contact.1)
                ));
            } else {
                contact_block.push(format!(
                    "            <{0}>{1}</{0}>",
                    map_to_evc(contact.0, app),
                    clean_symbols(contact.1)
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
            Ok(_) => (),
            Err(e) => eprintln!("Failed {e}"),
        },
        Err(err) => {
            eprintln!("Failed because {err}");
        }
    }
}
