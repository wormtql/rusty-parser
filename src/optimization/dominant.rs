use crate::utils::Graph;

use std::collections::HashSet;
use std::fs;

use prettytable::{Cell, Row, Table};

pub fn graph_from_file(file: &str) -> Graph<(), ()> {
    let contents = fs::read_to_string(file).unwrap();
    let mut ans: Graph<(), ()> = Graph::new();

    let lines: Vec<&str> = contents.lines().collect();
    for i in 0..lines.len() {
        ans.add_node(());
    }

    for (index, line) in lines.iter().enumerate() {
        let temp: Vec<&str> = line.split_whitespace().collect();
        let count: usize = temp[0].parse().unwrap();

        for i in 1..=count {
            let to: usize = temp[i].parse().unwrap();
            ans.add_edge(index, to, ());
        }
    }

    ans
}

pub fn dominant(g: &Graph<(), ()>) -> Vec<HashSet<usize>> {
    let node_count = g.head.len();
    let mut ans: Vec<HashSet<usize>> = vec![HashSet::new(); node_count];

    let whole: HashSet<usize> = (0..node_count).collect();
    // initialize
    ans[0].insert(0);
    for i in 1..node_count {
        for j in 0..node_count {
            ans[i].insert(j);
        }
    }

    let mut flag = true;
    while flag {
        flag = false;

        for node in 1..node_count {
            let mut newd: HashSet<usize> = whole.clone();
            let mut e = g.parent[node];
            while e != -1 {
                let edge = &g.edges[e as usize];
                let from = edge.from as usize;
                newd = newd.intersection(&ans[from]).cloned().collect();
                e = edge.next_rev;
            }

            newd.insert(node);
            if newd != ans[node] {
                ans[node] = newd;
                flag = true;
            }
        }
    }

    ans
}

pub fn get_table(data: &Vec<HashSet<usize>>) -> Table {
    let mut table = Table::new();

    for i in 0..data.len() {
        let mut row: Vec<Cell> = Vec::new();
        row.push(Cell::new(i.to_string().as_str()));

        row.push(Cell::new(&format!("{:?}", data[i])));
        table.add_row(Row::new(row));
    }

    table
}

pub fn calc_loop(g: &Graph<(), ()>, dom: &Vec<HashSet<usize>>) -> Vec<(usize, usize)> {
    let mut ans = Vec::new();

    for i in 0..g.head.len() {
        let mut e = g.head[i];
        while e != -1 {
            let edge = &g.edges[e as usize];
            let to = edge.to as usize;
            
            if dom[i].contains(&to) {
                ans.push((i, to));
            }
            
            e = edge.next;
        }
    }

    ans
}