use calamine::{open_workbook, Reader, Xlsx};
use chrono::{DateTime, Duration, Local};

use crate::types::{AppState, Contacts};
use std::collections::HashMap;

pub fn read_excel(app: &mut AppState) -> Result<Contacts, Box<dyn std::error::Error>> {
    let start_time: DateTime<Local> = Local::now();
    let path = app.input.clone().unwrap();
    println!("Read excel: {:?}", path);
    let mut workbook: Xlsx<_> = open_workbook(path)?;
    let range = workbook.worksheet_range(&app.worksheet_name).unwrap();
    let mut contacts: Contacts = vec![];

    if let Some(first_row) = range.rows().next() {
        for (i, row) in range.rows().enumerate() {
            if i != 0 {
                let mut row_data: HashMap<String, String> = HashMap::new();
                for (header, cell) in first_row.iter().zip(row.iter()) {
                    row_data.insert(header.to_string(), cell.to_string());
                }
                contacts.push(row_data);
            }
        }
    }

    let end_time: DateTime<Local> = Local::now();
    let duration: Duration = end_time.signed_duration_since(start_time);
    println!("Read Successfull in {:?}", duration);
    Ok(contacts)
}
