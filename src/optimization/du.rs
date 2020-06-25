use std::fs;
use std::collections::{HashSet};

use crate::utils;
use crate::utils::Graph;

use prettytable::{Cell, Row, Table};

type List = Vec<(HashSet<(String, usize)>, HashSet<(String, usize)>)>;

pub fn load_use_def_from_file(file: &str) -> List {
    let contents = fs::read_to_string(file).unwrap();

    let mut ans: List = Vec::new();

    for line in contents.lines() {
        let mut use_set: HashSet<(String, usize)> = HashSet::new();
        let mut def_set: HashSet<(String, usize)> = HashSet::new();
        let temp1: Vec<&str> = line.split(",").collect();
        
        if temp1[0].trim() != "" {
            for i in temp1[0].split(";") {
                let temp2: Vec<&str> = i.split(":").collect();
                let letter = String::from(temp2[0].trim());
                for j in temp2[1].split_whitespace() {
                    use_set.insert((letter.clone(), j.parse().unwrap()));
                }
            }
        }
        
        if temp1[1] != "" {
            for i in temp1[1].split(";") {
                let temp2: Vec<&str> = i.split(":").collect();
                let letter = String::from(temp2[0].trim());
                for j in temp2[1].split_whitespace() {
                    def_set.insert((letter.clone(), j.parse().unwrap()));
                }
            }
        }

        ans.push((use_set, def_set));
    }

    // println!("{:?}", ans);
    ans
}

pub fn calc_with_process(g: &Graph<(), ()>, use_def: &List) -> (List, Table) {
    let mut table = Table::new();
    let mut ans: List = Vec::new();

    let block_count = use_def.len();

    let mut row1: Vec<Cell> = Vec::new();
    let mut row2: Vec<Cell> = Vec::new();
    let mut header: Vec<Cell> = Vec::new();
    header.push(Cell::new(""));
    row1.push(Cell::new("OUT_L"));
    row2.push(Cell::new("IN_L"));

    for i in 0..block_count {
        ans.push((use_def[i].0.clone(), HashSet::new()));
        row1.push(Cell::new("{}"));
        row2.push(Cell::new(&format_helper(&ans[i].0)));
        header.push(Cell::new(&format!("B{}", i + 1)))
    }
    table.add_row(Row::new(header));
    table.add_row(Row::new(row1));
    table.add_row(Row::new(row2));

    let mut flag = true;
    while flag {
        flag = false;
        let mut row1: Vec<Cell> = Vec::new();
        let mut row2: Vec<Cell> = Vec::new();
        row1.push(Cell::new("OUT_L"));
        row2.push(Cell::new("IN_L"));

        for i in (0..block_count).rev() {
            let mut new_out: HashSet<(String, usize)> = HashSet::new();

            let mut e = g.head[i];
            while e != -1 {
                let edge = &g.edges[e as usize];
                let to = edge.to as usize;

                new_out = new_out.union(&ans[to].0).cloned().collect();
                e = edge.next;
            }

            if new_out != ans[i].1 {
                ans[i].1 = new_out;
                ans[i].0 = ans[i].1.difference(&use_def[i].1).cloned().collect();
                ans[i].0 = ans[i].0.union(&use_def[i].0).cloned().collect();
                flag = true;
            }
        }

        for i in 0..block_count {
            row1.push(Cell::new(&format_helper(&ans[i].1)));
            row2.push(Cell::new(&format_helper(&ans[i].0)));
        }

        table.add_row(Row::new(row1));
        table.add_row(Row::new(row2));
    }

    (ans, table)
}

fn format_helper(v: &HashSet<(String, usize)>) -> String {
    if v.len() == 0 {
        return String::from("{}");
    }

    let mut temp: Vec<_> = v.iter().collect();
    temp.sort();

    let mut ans = String::from("{");
    ans.push_str(&format!("{}{}", temp[0].0, temp[0].1));

    for i in temp.iter().skip(1) {
        ans.push_str(&format!(",{}{}", i.0, i.1));
    }
    ans.push_str("}");

    ans
}

pub fn get_table(v: &List) -> Table {
    let mut table = Table::new();

    let mut header: Vec<Cell> = Vec::new();
    header.push(Cell::new(""));
    header.push(Cell::new("IN_L"));
    header.push(Cell::new("OUT_L"));
    table.add_row(Row::new(header));

    for i in 0..v.len() {
        let mut row: Vec<Cell> = Vec::new();

        row.push(Cell::new(&format!("B{}", i + 1)));
        row.push(Cell::new(&format_helper(&v[i].0)));
        row.push(Cell::new(&format_helper(&v[i].1)));

        table.add_row(Row::new(row));
    }

    table
}