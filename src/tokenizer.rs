use regex::Regex;
use std::fs;
use crate::select::SelectStatement;

pub fn tokenize_file(file_location: &str) {
    let file_contents: String;
    let read_result = fs::read_to_string(file_location);
    match read_result {
        Ok(content) => file_contents = content,
        Err(e) => panic!("{:#?}", e),
    }

    let selects = tokenize_selects(&file_contents);
    println!("{:#?}", selects);
}

fn tokenize_selects(sql: &str) -> Vec<SelectStatement> {
    let select_regex = Regex::new(r"select(?P<columns>.*)from").unwrap();
    let mut selects: Vec<SelectStatement> = vec!();
    for cap in select_regex.captures_iter(sql) {
        let individual_columns: Vec<&str> = cap["columns"].split(',').collect();
        let mut new_select = SelectStatement { columns : vec!() };
        for c in individual_columns {
            new_select.columns.push(c.trim().to_string());
        }
        selects.push(new_select);
    }

    selects
}


