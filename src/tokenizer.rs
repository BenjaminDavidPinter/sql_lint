use crate::select::SelectStatement;
use regex::Regex;
use std::fs;

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
    let select_regex = Regex::new(r"select\b(?P<columns>(.|\n)*?)from\b(?P<table>.*)").unwrap();
    let mut selects: Vec<SelectStatement> = vec![];
    for cap in select_regex.captures_iter(sql) {
        let regex_column_result: Vec<&str> = cap["columns"].split(',').collect();
        let regex_table_result: String = cap["table"].to_string();

        let mut select_table_schema: String = String::new();
        let mut select_table_alias: String = String::new();
        let mut select_table: String;

        if regex_table_result.contains("as") {
            let local_regex_table_result = regex_table_result.clone();
            let split_table_string: Vec<&str> = local_regex_table_result.split("as").collect();
            select_table = split_table_string[0].trim().to_string();
            select_table_alias = split_table_string[1].trim().to_string();
        } else {
            select_table = regex_table_result.trim().to_string();
        }

        if select_table.contains('.') {
            let local_select_table = select_table.clone();
            let schema_table: Vec<&str> = local_select_table.split('.').collect();
            select_table = schema_table[1].to_string();
            select_table_schema = schema_table[0].to_string();
        }

        let mut new_select = SelectStatement {
            columns: vec![],
            table: select_table,
            alias: select_table_alias,
            schema: select_table_schema,
        };

        for c in regex_column_result {
            new_select.columns.push(c.trim().to_string());
        }

        selects.push(new_select);
    }

    selects
}
