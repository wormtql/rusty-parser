use std::fs;
use std::collections::HashSet;

use crate::utils;
use crate::utils::Graph;

use prettytable::{Cell, Row, Table};

type List = Vec<(HashSet<usize>, HashSet<usize>)>;

pub fn load_gen_kill_from_file(file: &str) -> List {
    let contents = fs::read_to_string(file).unwrap();

    let mut ans: List = Vec::new();

    for line in contents.lines() {
        let temp: Vec<&str> = line.split(",").collect();
        let gen: HashSet<usize> = temp[0].split_whitespace().map(|x| x.parse().unwrap()).collect();
        let kill: HashSet<usize> = temp[1].split_whitespace().map(|x| x.parse().unwrap()).collect();

        ans.push((gen, kill));
    }

    ans
}

pub fn calc_with_process(g: &Graph<(), ()>, gk: &List) -> (List, Table) {
    let mut ans: List = Vec::new();
    let mut table = Table::new();

    let mut row1: Vec<Cell> = Vec::new();
    let mut row2: Vec<Cell> = Vec::new();
    let mut header: Vec<Cell> = Vec::new();
    header.push(Cell::new(""));
    row1.push(Cell::new("IN"));
    row2.push(Cell::new("OUT"));

    for i in 0..gk.len() {
        header.push(Cell::new(&format!("B{}", i + 1)));
        row1.push(Cell::new("{}"));
        row2.push(Cell::new(&utils::hash_set_to_sorted_str(&gk[i].0, ",")));
        ans.push((HashSet::new(), gk[i].0.clone()));
    }
    table.add_row(Row::new(header));
    table.add_row(Row::new(row1));
    table.add_row(Row::new(row2));

    let mut flag = true;
    while flag {
        flag = false;
        let mut row1: Vec<Cell> = Vec::new();
        let mut row2: Vec<Cell> = Vec::new();
        row1.push(Cell::new("IN"));
        row2.push(Cell::new("OUT"));

        for i in 0..gk.len() {
            let mut new_in = HashSet::new();

            let mut e = g.parent[i];
            while e != -1 {
                let edge = &g.edges[e as usize];
                let from = edge.from as usize;

                new_in = new_in.union(&ans[from].1).cloned().collect();
                e = edge.next_rev;
            }

            if new_in != ans[i].0 {
                flag = true;
                ans[i].0 = new_in;
                ans[i].1 = ans[i].0.difference(&gk[i].1).cloned().collect();
                ans[i].1 = ans[i].1.union(&gk[i].0).cloned().collect();

                row1.push(Cell::new(&utils::hash_set_to_sorted_str(&ans[i].0, ",")));
                row2.push(Cell::new(&utils::hash_set_to_sorted_str(&ans[i].1, ",")));
            } else {
                row1.push(Cell::new("no change"));
                row2.push(Cell::new("no change"));
            }
        }

        table.add_row(Row::new(row1));
        table.add_row(Row::new(row2));
    }

    (ans, table)
}

pub fn get_table(v: &List) -> Table {
    let mut table = Table::new();

    let mut header: Vec<Cell> = Vec::new();
    header.push(Cell::new(""));
    header.push(Cell::new("IN"));
    header.push(Cell::new("OUT"));
    table.add_row(Row::new(header));

    for i in 0..v.len() {
        let mut row: Vec<Cell> = Vec::new();

        row.push(Cell::new(&format!("B{}", i + 1)));
        row.push(Cell::new(&utils::hash_set_to_sorted_str(&v[i].0, ",")));
        row.push(Cell::new(&utils::hash_set_to_sorted_str(&v[i].1, ",")));

        table.add_row(Row::new(row));
    }

    table
}