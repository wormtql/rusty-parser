use crate::grammar::letter::Letter;
use crate::utils;

use std::fmt;
use std::fs;
use std::collections::HashSet;
use std::hash::{Hash, Hasher};

use prettytable::{Cell, Row, Table};


#[derive(Eq, PartialEq, Clone)]
struct MySet<T: Hash + Eq + PartialEq + Clone> {
    set: HashSet<T>
}

impl fmt::Display for MySet<usize> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.set)
    }
}

impl fmt::Debug for MySet<usize> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Hash for MySet<usize> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut temp: Vec<usize> = self.set.iter().cloned().collect();
        temp.sort();
        for i in temp {
            i.hash(state);
        }
    }
}

impl<T: Hash + Eq + PartialEq + Clone> MySet<T> {
    fn wrap(s: HashSet<T>) -> MySet<T> {
        MySet {
            set: s
        }
    }
}


pub struct DFA {
    pub start: usize,
    pub end: Vec<usize>,
    pub letters: Vec<Letter>,

    pub edges: Vec<Vec<(Letter, usize)>>,
}

impl fmt::Display for DFA {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        write!(f, "{}", self.displayable_table())
    }
}

impl DFA {
    pub fn from_file(file: &str) -> DFA {
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
                let letter = Letter::Terminal(String::from(temp[j * 2 + 1]));
                let to: usize = temp[j * 2 + 2].parse().unwrap();
                edge.push((letter, to));
            }

            edges.push(edge);
        }

        DFA {
            start: start_node,
            end: end_node,
            letters: letter_set,
            edges
        }
    }

    pub fn step_single(&self, state: usize, le: Letter) -> Option<usize> {
        for (letter, to) in self.edges[state].iter() {
            if *letter == le {
                return Some(*to);
            }
        }

        None
    }

    pub fn minimize_with_process(&self) -> (DFA, Table, Table) {
        let mut table = Table::new();
        

        let mut split: HashSet<MySet<usize>> = HashSet::new();
        // let mut queue: Vec<MySet<usize>> = Vec::new();

        let s1: HashSet<usize> = self.end.iter().cloned().collect();
        let s2: HashSet<usize> = (0..self.edges.len()).filter(|x| !self.end.contains(x)).collect();
        let s1 = MySet::wrap(s1);
        let s2 = MySet::wrap(s2);

        // queue.push(s1.clone());
        // queue.push(s2.clone());
        split.insert(s1);
        split.insert(s2);

        'bigloop: loop {

            let split_vec: Vec<Vec<usize>> = split.iter().map(|x| x.set.iter().cloned().collect()).collect();
            // println!("{:?}", split_vec);
            let mut new_split = split.clone();

            'everysplit: for p in split.iter() {
                'everyletter: for letter in self.letters.iter() {
                    let mut temp: Vec<HashSet<usize>> = vec![HashSet::new(); split.len() + 1];
                    let last = temp.len() - 1;

                    let mut row: Vec<Cell> = Vec::new();

                    for &state in p.set.iter() {
                        let to = self.step_single(state, letter.clone());

                        if to.is_none() {
                            temp[last].insert(state);
                        } else {
                            let to2 = to.unwrap();
                            // determine which split it is
                            let mut belong = 0;
                            for (index, i) in split_vec.iter().enumerate() {
                                if i.contains(&to2) {
                                    belong = index;
                                    break;
                                }
                            }

                            temp[belong].insert(state);
                        }
                    }

                    let addition_split: Vec<HashSet<usize>> = temp.drain(..).filter(|x| x.len() > 0).collect();
                    let cell1 = format!("{:?} -> {:?}", p.set, &addition_split);
                    row.push(Cell::new(&cell1));
                    // println!("{:?}", addition_split);
                    if addition_split.len() == 1 {
                        // no need to split
                        continue;
                    } else {
                        // need to split
                        new_split.remove(p);
                        for i in addition_split {
                            // queue.push(MySet::wrap(i.clone()));
                            new_split.insert(MySet::wrap(i));
                        }

                        let cell2 = format!("{:?}", new_split);
                        row.push(Cell::new(&cell2));
                        table.add_row(Row::new(row));

                        break;
                    }
                }
            }

            if new_split == split {
                break;
            } else {
                split = new_split;
            }
        }

        // now to contruct new DFA
        let mut end: Vec<usize> = Vec::new();
        let mut edges: Vec<Vec<(Letter, usize)>> = vec![Vec::new(); split.len()];
        let mut start = 0;
        let state: Vec<HashSet<usize>> = split.iter().map(|x| x.set.clone()).collect();

        // start and end state
        for i in 0..state.len() {
            if state[i].iter().any(|x| self.end.contains(x)) {
                end.push(i);
            }
            if state[i].iter().any(|x| self.start == *x) {
                start = i;
            }
        }

        let mut table2 = Table::new();
        let mut header: Vec<Cell> = Vec::new();
        header.push(Cell::new(""));
        for i in self.letters.iter() {
            header.push(Cell::new(&i.to_string()));
        }
        table2.add_row(Row::new(header));

        for i in 0..state.len() {
            let representative = *state[i].iter().next().unwrap();
            let mut row: Vec<Cell> = Vec::new();

            if end.contains(&i) {
                row.push(Cell::new(&format!("{:?}*", state[i])));
            } else {
                row.push(Cell::new(&format!("{:?}", state[i])));
            }
            

            for letter in self.letters.iter() {
                let to = self.step_single(representative, letter.clone());
                if to.is_some() {
                    // to which new state
                    let mut to_new_state = 0;
                    for j in 0..state.len() {
                        if state[j].contains(&to.unwrap()) {
                            to_new_state = j;
                            break;
                        }
                    }

                    row.push(Cell::new(&format!("{:?}", state[to_new_state])));
                    edges[i].push((letter.clone(), to_new_state));
                } else {
                    row.push(Cell::new(""));
                }
            }

            table2.add_row(Row::new(row));
        }

        let dfa = DFA {
            start,
            end,
            edges,
            letters: self.letters.clone()
        };

        (dfa, table, table2)
    }

    pub fn displayable_table(&self) -> Table {
        let mut table = Table::new();
        let mut header: Vec<Cell> = Vec::new();
        header.push(Cell::new(""));
        for i in self.letters.iter() {
            header.push(Cell::new(&i.to_string()));
        }
        table.add_row(Row::new(header));


        for index in 0..self.edges.len() {
            let mut row: Vec<Cell> = Vec::new();

            if self.end.contains(&index) {
                row.push(Cell::new(&(index.to_string() + "*")));
            } else {
                row.push(Cell::new(&index.to_string()));
            }
            

            for letter in self.letters.iter() {
                let mut temp = 10086;
                for (le, to) in self.edges[index].iter() {
                    if letter == le {
                        temp = *to;
                        break;
                    }
                }

                if temp == 10086 {
                    row.push(Cell::new(""));
                } else {
                    row.push(Cell::new(&temp.to_string()));
                }
            }

            table.add_row(Row::new(row));
        }

        table
    }
}