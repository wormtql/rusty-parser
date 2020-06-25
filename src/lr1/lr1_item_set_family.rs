use super::lr1_item_set::LR1ItemSet;

use crate::grammar::letter::Letter;
use crate::grammar::Grammar;
use crate::first_follow_set::FirstFollowSet;

use std::collections::{HashMap, VecDeque};
use std::fmt;

use rand::thread_rng;
use rand::{Rng, SeedableRng};
use rand::seq::SliceRandom;
use rand::rngs::StdRng;

use prettytable::{Cell, Row, Table};

pub enum GoFunctionItem {
    Err,
    Step(usize)
}

impl GoFunctionItem {
    pub fn is_err(&self) -> bool {
        if let GoFunctionItem::Err = self {
            true
        } else {
            false
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

pub struct LR1ItemSetFamily {
    pub item_sets: Vec<LR1ItemSet>,
    pub go_function: Vec<HashMap<Letter, GoFunctionItem>>
}

impl fmt::Display for LR1ItemSetFamily {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "LR(1) standard item set family:\n").unwrap();
        let table1 = self.displayable_sets_table();
        let table2 = self.displayable_go_table();

        write!(f, "{}", table1).unwrap();

        write!(f, "GO function:\n").unwrap();
        write!(f, "{}", table2).unwrap();

        Ok(())
    }
}

impl LR1ItemSetFamily {
    pub fn from_grammar_with_seed(g: &Grammar, seed: u64) -> LR1ItemSetFamily {
        let mut item_sets: Vec<LR1ItemSet> = Vec::new();
        let mut go_function: Vec<HashMap<Letter, GoFunctionItem>> = Vec::new();

        // first follow set
        let ffset = FirstFollowSet::from_grammar(g);

        // initial state
        let initial_set = LR1ItemSet::from_grammar_initial(g).closure(&g.rules, &ffset);
        // item_sets.push(initial_set);

        // every signs
        let mut signs: Vec<Letter> = Vec::new();
        signs.extend_from_slice(&g.terminals);
        signs.extend_from_slice(&g.non_terminals);
        signs.sort();

        // rng
        let mut rng: StdRng = SeedableRng::seed_from_u64(seed);

        // bfs search

        // state and its corresponding state id
        let mut vis: HashMap<LR1ItemSet, usize> = HashMap::new();
        // bfs queue, storing state id
        let mut q: Vec<LR1ItemSet> = Vec::new();
        q.push(initial_set.clone());
        vis.insert(initial_set.clone(), 0);
        item_sets.push(initial_set);
        while !q.is_empty() {
            let ran: usize = rng.gen_range(0, q.len());
            let p = q.remove(ran);

            let mut shuffle = &mut signs[..];
            shuffle.shuffle(&mut rng);
            for letter in shuffle {
                // calc GO(p, letter)
                let temp = p.go(&g.rules, letter, &ffset);

                if !temp.is_empty() {
                    if !vis.contains_key(&temp) {
                        item_sets.push(temp.clone());
                        vis.insert(temp.clone(), item_sets.len() - 1);
                        q.push(temp);
                    }
                }
            }
        }

        // calc go function
        for state in 0..item_sets.len() {
            let mut go: HashMap<Letter, GoFunctionItem> = HashMap::new();

            let item_set = &item_sets[state];
            for letter in signs.iter() {
                let temp = item_set.go(&g.rules, letter, &ffset);
                if !temp.is_empty() {
                    go.insert(letter.clone(), GoFunctionItem::Step(*vis.get(&temp).unwrap()));
                } else {
                    go.insert(letter.clone(), GoFunctionItem::Err);
                }
            }

            go_function.push(go);
        }

        LR1ItemSetFamily {
            item_sets,
            go_function
        }
    }

    pub fn from_grammar(g: &Grammar) -> LR1ItemSetFamily {
        LR1ItemSetFamily::from_grammar_with_seed(g, 0)
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