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

pub fn calc_use_def_from_file(file: &str) -> List {
    let contents = fs::read_to_string(file).unwrap();

    let mut ans: List = Vec::new();
    let mut counter = 0;
    let mut temp: Vec<Vec<(usize, String, Vec<String>)>> = Vec::new();
    temp.push(Vec::new());

    for i in contents.trim().lines() {
        if i == "" {
            counter += 1;
            temp.push(Vec::new());
        } else {
            let t1: Vec<&str> = i.split(":").collect();
            let point_name = String::from(t1[0].trim());
            let point_name: usize = (&point_name[1..]).parse().unwrap();

            let t2: Vec<&str> = t1[1].split("=").collect();
            let def_var = String::from(t2[0].trim());
            let use_var: Vec<String> = t2[1].trim().split_whitespace().map(|x| String::from(x)).collect();

            temp[counter].push((point_name, def_var, use_var));
        }
    }

    for (index, block) in temp.iter().enumerate() {
        let mut def: HashSet<String> = HashSet::new();

        let mut use_set: HashSet<(String, usize)> = HashSet::new();
        let mut def_set: HashSet<(String, usize)> = HashSet::new();

        for (i, (point, left, uses)) in block.iter().enumerate() {
            // update USE set
            for u in uses.iter() {
                if !def.contains(u) {
                    use_set.insert((u.clone(), *point));
                }
            }

            // update DEF set
            for j in i + 1..block.len() {
                for k in 0..block[j].2.len() {
                    if block[j].2[k] == *left {
                        def_set.insert((left.clone(), block[j].0));
                    }
                }
            }
            for j in 0..temp.len() {
                if j != index {
                    for k in 0..temp[j].len() {
                        for u in 0..temp[j][k].2.len() {
                            if temp[j][k].2[u] == *left {
                                def_set.insert((left.clone(), temp[j][k].0));
                            }
                        }
                    }
                }
            }

            def.insert(left.clone());
        }

        ans.push((use_set, def_set));
    }

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

pub fn get_table(v: &List, col1: &str, col2: &str) -> Table {
    let mut table = Table::new();

    let mut header: Vec<Cell> = Vec::new();
    header.push(Cell::new(""));
    header.push(Cell::new(col1));
    header.push(Cell::new(col2));
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