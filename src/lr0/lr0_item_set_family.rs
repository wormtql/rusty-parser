use super::lr0_item_set::LR0ItemSet;
use crate::grammar::Grammar;
use crate::grammar::letter::Letter;
use std::collections::HashMap;
// use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt;

use rand::thread_rng;
use rand::{Rng, SeedableRng};
use rand::seq::SliceRandom;
use rand::rngs::StdRng;

use prettytable::{Table, Row, Cell};

pub enum GoFunctionItem {
    Err,
    Step(usize)
}

impl GoFunctionItem {
    pub fn is_err(&self) -> bool {
        match self {
            GoFunctionItem::Err => true,
            _ => false
        }
    }

    pub fn value(&self) -> usize {
        if let GoFunctionItem::Step(x) = self {
            *x
        } else {
            panic!("error")
        }
    }
}

impl fmt::Display for GoFunctionItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let GoFunctionItem::Step(x) = self {
            write!(f, "{}", x).unwrap();
        }
        Ok(())
    }
}

impl fmt::Debug for GoFunctionItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

pub struct LR0ItemSetFamily {
    pub item_sets: Vec<LR0ItemSet>,
    pub go_function: Vec<HashMap<Letter, GoFunctionItem>>,
}

impl fmt::Display for LR0ItemSetFamily {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LR(0) standard item set family:\n").unwrap();

        let table1 = self.displayable_sets_table();
        write!(f, "{}", table1).unwrap();

        let table2 = self.displayable_go_table();
        
        write!(f, "GO function:\n").unwrap();
        write!(f, "{}", table2).unwrap();

        Ok(())
    }
}

impl LR0ItemSetFamily {
    pub fn from_grammar_with_seed(g: &Grammar, seed: u64) -> LR0ItemSetFamily {

        // final item sets
        let mut item_sets: Vec<LR0ItemSet> = Vec::new();
        // final go function
        let mut go_function: Vec<HashMap<Letter, GoFunctionItem>> = Vec::new();

        // every LR(0) item
        let whole = LR0ItemSet::from_grammar(g);

        // initial state
        let initial_set = LR0ItemSet::from_grammar_initial(g).closure(&whole);
        // item_sets.push(initial_set);

        // every signs
        let mut signs: Vec<Letter> = Vec::new();
        signs.extend_from_slice(&g.terminals);
        signs.extend_from_slice(&g.non_terminals);
        signs.sort();

        // rng
        // let mut rng = rand::thread_rng();
        let mut rng: StdRng = SeedableRng::seed_from_u64(seed);

        // bfs search

        // item set and its corresponding state id
        let mut vis: HashMap<LR0ItemSet, usize> = HashMap::new();
        // bfs queue, storing item set
        let mut q: Vec<LR0ItemSet> = Vec::new();
        q.push(initial_set.clone());
        item_sets.push(initial_set.clone());
        vis.insert(initial_set, 0);
        while !q.is_empty() {
            let ran: usize = rng.gen_range(0, q.len());
            // let p = q.pop_front().unwrap();

            // random choose
            let p = q.remove(ran);

            let mut shuffle = &mut signs[..];
            shuffle.shuffle(&mut rng);
            for letter in shuffle {
                // calc GO(p, letter)
                let temp = p.go(&whole, letter);

                if !temp.is_empty() {
                    if !vis.contains_key(&temp) {
                        q.push(temp.clone());
                        item_sets.push(temp.clone());
                        vis.insert(temp, item_sets.len() - 1);
                    }
                }
            }
        }

        // calc go function
        for state in 0..item_sets.len() {
            let mut go: HashMap<Letter, GoFunctionItem> = HashMap::new();

            let item_set = &item_sets[state];
            for letter in signs.iter() {
                let temp = item_set.go(&whole, letter);
                if !temp.is_empty() {
                    go.insert(letter.clone(), GoFunctionItem::Step(*vis.get(&temp).unwrap()));
                } else {
                    go.insert(letter.clone(), GoFunctionItem::Err);
                }
            }

            go_function.push(go);
        }

        LR0ItemSetFamily {
            item_sets,
            go_function
        }
    }

    pub fn from_grammar(g: &Grammar) -> LR0ItemSetFamily {
        LR0ItemSetFamily::from_grammar_with_seed(g, 0)
    }

    pub fn displayable_sets_table(&self) -> Table {
        let mut table = Table::new();
        let c = 5;

        for i in 0..self.item_sets.len() / c {
            let mut num: Vec<Cell> = Vec::new();
            let mut row: Vec<Cell> = Vec::new();
            for j in i * c..i * c + c {
                num.push(Cell::new(&j.to_string()));
                row.push(Cell::new(&self.item_sets[j].to_string()));
            }
            table.add_row(Row::new(num));
            table.add_row(Row::new(row));
        }

        if self.item_sets.len() % c == 0 {
            return table;
        }

        let mut num: Vec<Cell> = Vec::new();
        let mut row: Vec<Cell> = Vec::new();
        let temp = self.item_sets.len() / c * c;
        for i in temp..self.item_sets.len() {
            num.push(Cell::new(&i.to_string()));
            row.push(Cell::new(&self.item_sets[i].to_string()));
        }
        table.add_row(Row::new(num));
        table.add_row(Row::new(row));

        table
    }

    pub fn displayable_go_table(&self) -> Table {
        let mut table = Table::new();

        let mut letters: Vec<_> = self.go_function[0].keys().cloned().collect();
        letters.sort();

        let mut header: Vec<Cell> = Vec::new();
        header.push(Cell::new(""));
        for letter in letters.iter() {
            header.push(Cell::new(&letter.get_str()));
        }
        table.add_row(Row::new(header));
        
        for (index, i) in self.go_function.iter().enumerate() {
            let mut row: Vec<Cell> = Vec::new();
            row.push(Cell::new(&index.to_string()));
            for letter in letters.iter() {
                row.push(Cell::new(&i.get(letter).unwrap().to_string()));
            }
            table.add_row(Row::new(row));
        }

        table
    }
}