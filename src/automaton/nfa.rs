use crate::grammar::letter::Letter;
use crate::utils;

use super::dfa::DFA;

use std::fs;
use std::fmt;
use std::collections::HashMap;

use prettytable::{Cell, Table, Row};

pub struct NFA {
    pub start: usize,
    pub end: Vec<usize>,
    pub letters: Vec<Letter>,
    
    pub edges: Vec<Vec<(Letter, usize)>>,
}

impl fmt::Display for NFA {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "letters:\n").unwrap();
        for i in self.letters.iter() {
            write!(f, "{} ", i).unwrap();
        }
        write!(f, "\n").unwrap();

        for (index, edge) in self.edges.iter().enumerate() {
            write!(f, "{}:\n", index).unwrap();
            for (letter, to) in edge.iter() {
                write!(f, "    {} -> {}\n", letter.get_str(), to).unwrap();
            }
        }
        Ok(())
    }
}

impl NFA {
    pub fn from_file(file: &str) -> NFA {
        let s = fs::read_to_string(file).unwrap();

        let lines: Vec<&str> = s.lines().collect();

        let start_node: usize = lines[0].parse().unwrap();
        let end_node: Vec<usize> = lines[1].split_whitespace().map(|x| x.parse().unwrap()).collect();
        let letter_set: Vec<Letter> = lines[2].split_whitespace()
            .map(|x| Letter::Terminal(String::from(x))).collect();

        let mut edges: Vec<Vec<(Letter, usize)>> = Vec::new();
        for i in 3..lines.len() {
            let temp: Vec<&str> = lines[i].split_whitespace().collect();

            let edge_count: usize = temp[0].parse().unwrap();
            let mut edge: Vec<(Letter, usize)> = Vec::new();
            for j in 0..edge_count {
                let letter = if temp[j * 2 + 1] == "." {
                    Letter::Empty
                } else {
                    Letter::Terminal(String::from(temp[j * 2 + 1]))
                };
                let to: usize = temp[j * 2 + 2].parse().unwrap();
                edge.push((letter, to));
            }

            edges.push(edge);
        }

        NFA {
            start: start_node,
            end: end_node,
            letters: letter_set,
            edges
        }
    }

    pub fn epsilon_closure(&self, seed: &Vec<usize>) -> Vec<usize> {
        let mut vis: Vec<bool> = vec![false; self.edges.len()];
        let mut queue: Vec<usize> = Vec::new();
        for &i in seed.iter() {
            vis[i] = true;
            queue.push(i);
        }

        let mut ans = seed.clone();
        while queue.len() != 0 {
            let p = queue.pop().unwrap();

            for (letter, to) in self.edges[p].iter() {
                if letter.is_empty() && !vis[*to] {
                    ans.push(*to);
                    queue.push(*to);
                    vis[*to] = true;
                }
            }
        }
        //println!("{:?}", ans);

        ans.sort();
        ans
    }

    pub fn step(&self, seed: &Vec<usize>, le: Letter) -> Vec<usize> {
        let mut ans: Vec<usize> = Vec::new();
        let mut vis: Vec<bool> = vec![false; self.edges.len()];

        for &i in seed.iter() {
            for (letter, to) in self.edges[i].iter() {
                if le == *letter && !vis[*to] {
                    ans.push(*to);
                    vis[*to] = true;
                }
            }
        }
        //println!("{:?}", ans);

        self.epsilon_closure(&ans)
    }

    pub fn to_dfa_with_process(&self) -> (DFA, Table) {
        let mut table = Table::new();
        let mut temp: Vec<Cell> = Vec::new();
        temp.push(Cell::new(""));
        for i in self.letters.iter() {
            temp.push(Cell::new(i.to_string().as_str()))
        }
        table.add_row(Row::new(temp));

        let mut queue: Vec<Vec<usize>> = Vec::new();
        let mut vis: HashMap<Vec<usize>, usize> = HashMap::new();

        let mut edges: Vec<Vec<(Letter, usize)>> = Vec::new();
        let mut end: Vec<usize> = Vec::new();
        let mut node_count = 1;
        edges.push(Vec::new());
        
        let initial_set = self.epsilon_closure(&vec![self.start]);
        if initial_set.iter().any(|x| self.end.contains(x)) {
            end.push(0);
        }
        queue.push(initial_set.clone());
        vis.insert(initial_set, 0);

        while queue.len() != 0 {
            let p = queue.remove(0);
            let current_number = *vis.get(&p).unwrap();

            let mut temp: Vec<Cell> = Vec::new();
            
            if end.contains(&current_number) {
                temp.push(Cell::new((utils::vec_to_set_string(&p) + "*").as_str()));
            } else {
                temp.push(Cell::new(utils::vec_to_set_string(&p).as_str()));
            }

            for letter in self.letters.iter() {
                let to = self.step(&p, letter.clone());

                temp.push(Cell::new(utils::vec_to_set_string(&to).as_str()));

                if to.len() == 0 {
                    continue;
                }

                if !vis.contains_key(&to) {
                    vis.insert(to.clone(), node_count);
                    node_count += 1;
                    edges.push(Vec::new());
                    if to.len() != 0 {
                        queue.push(to.clone());
                    }
                }

                let number = *vis.get(&to).unwrap();
                if to.iter().any(|x| self.end.contains(x)) {
                    end.push(number);
                }

                edges[current_number].push((letter.clone(), number));
            }

            table.add_row(Row::new(temp));
        }

        let dfa = DFA {
            start: 0,
            end: end,
            letters: self.letters.clone(),
            edges: edges
        };

        (dfa, table)
    }
}