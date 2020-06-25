// use std::fmt;
use prettytable::{Table, Row, Cell};
use std::collections::HashMap;
use crate::grammar::letter::Letter;

pub fn print_go_function(data: &Vec<HashMap<Letter, usize>>) {
    let mut table = Table::new();

    let letters: Vec<Letter> = data[0].keys().cloned().collect();

    let mut header: Vec<Cell> = Vec::new();
    header.push(Cell::new(""));
    for i in &letters {
        header.push(Cell::new(i.get_str()));
    }

    table.add_row(Row::new(header));

    for i in 0..data.len() {
        let mut row: Vec<Cell> = Vec::new();
        row.push(Cell::new(&format!("{}", i)));
        for letter in &letters {
            if data[i].contains_key(letter) {
                row.push(Cell::new(&data[i].get(letter).unwrap().to_string()));
            } else {
                row.push(Cell::new(""));
            }
        }
        table.add_row(Row::new(row));
    }

    table.printstd();
}

