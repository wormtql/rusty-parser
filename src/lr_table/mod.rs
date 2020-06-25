use crate::grammar::letter::Letter;
use crate::grammar::rule::Rule;
use crate::token::{TokenStream, Token};
use crate::parse_tree::{ParseTree, NonLeafNode};
use crate::utils;

use std::collections::HashMap;
// use crate::lr0::lr0_item_set_family::LR0ItemSetFamily;
// use crate::grammar::Grammar;
use std::fmt;
use prettytable::{Table, Row, Cell};

use serde::{Serialize, Deserialize};

pub use lalr_table::LALRTable;

mod lalr_table;

#[derive(Debug, Serialize, Deserialize)]
pub enum GotoTableItem {
    Err,
    Step(usize),
}

impl GotoTableItem {
    pub fn get_state(&self) -> usize {
        match self {
            GotoTableItem::Err => panic!("goto table item is err, attempting to get state"),
            GotoTableItem::Step(x) => *x
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ActionTableItem {
    Step(usize),
    Reduce(Rule),
    Accept,
    Err,
}

#[derive(Serialize, Deserialize)]
pub struct LRTable {
    pub goto: Vec<HashMap<Letter, GotoTableItem>>,
    pub action: Vec<HashMap<Letter, ActionTableItem>>
}

impl fmt::Display for LRTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // let goto_table = self.displayable_goto_table();
        // let action_table = self.displayable_action_table();

        // write!(f, "action table:\n{}", action_table.to_string()).unwrap();
        // write!(f, "goto table:\n{}", goto_table.to_string())
        write!(f, "{}", self.displayable_table().to_string())
    }
}

impl LRTable {
    pub fn displayable_table(&self) -> Table {
        let mut header: Vec<Cell> = Vec::new();
        let mut terminals: Vec<Letter> = self.action[0].keys().cloned().collect();
        let mut non_terminals: Vec<Letter> = self.goto[0].keys().cloned().collect();
        let mut table = Table::new();

        terminals.sort();
        non_terminals.sort();

        header.push(Cell::new(""));
        for letter in &terminals {
            header.push(Cell::new(letter.get_str()));
        }
        header.push(Cell::new("|"));
        for letter in &non_terminals {
            header.push(Cell::new(letter.get_str()));
        }
        table.add_row(Row::new(header));

        for i in 0..self.action.len() {
            let mut row: Vec<Cell> = Vec::new();
            row.push(Cell::new(&i.to_string()));
            for letter in &terminals {
                let text = match self.action[i].get(letter).unwrap() {
                    ActionTableItem::Step(s) => s.to_string(),
                    ActionTableItem::Reduce(ref rule) => rule.to_string(),
                    ActionTableItem::Accept => String::from("acc"),
                    ActionTableItem::Err => String::new()
                };
                row.push(Cell::new(&text));
            }
            row.push(Cell::new("|"));

            for letter in &non_terminals {
                let text = match self.goto[i].get(letter).unwrap() {
                    GotoTableItem::Step(s) => s.to_string(),
                    GotoTableItem::Err => String::new()
                };
                row.push(Cell::new(&text));
            }
            table.add_row(Row::new(row));
        }

        table
    }

    pub fn displayable_action_table(&self) -> Table {
        let mut header: Vec<Cell> = Vec::new();
        let letters: Vec<Letter> = self.action[0].keys().cloned().collect();
        let mut table = Table::new();

        header.push(Cell::new(""));
        for letter in &letters {
            header.push(Cell::new(letter.get_str()));
        }
        table.add_row(Row::new(header));

        for i in 0..self.action.len() {
            let mut row: Vec<Cell> = Vec::new();
            row.push(Cell::new(&i.to_string()));
            for letter in &letters {
                let text = match self.action[i].get(letter).unwrap() {
                    ActionTableItem::Step(s) => s.to_string(),
                    ActionTableItem::Reduce(ref rule) => rule.to_string(),
                    ActionTableItem::Accept => String::from("acc"),
                    ActionTableItem::Err => String::new()
                };
                row.push(Cell::new(&text));
            }
            table.add_row(Row::new(row));
            
        }

        table
    }

    pub fn displayable_goto_table(&self) -> Table {
        let mut header: Vec<Cell> = Vec::new();
        let letters: Vec<Letter> = self.goto[0].keys().cloned().collect();
        let mut table = Table::new();

        header.push(Cell::new(""));
        for letter in &letters {
            header.push(Cell::new(letter.get_str()));
        }
        table.add_row(Row::new(header));

        for i in 0..self.goto.len() {
            let mut row: Vec<Cell> = Vec::new();
            row.push(Cell::new(&i.to_string()));
            for letter in &letters {
                let text = match self.goto[i].get(letter).unwrap() {
                    GotoTableItem::Step(s) => s.to_string(),
                    GotoTableItem::Err => String::new()
                };
                row.push(Cell::new(&text));
            }
            table.add_row(Row::new(row));
        }

        table
    }

    pub fn analysis_with_process(&self, token_stream: &TokenStream) -> (Option<ParseTree>, Table) {
        // state stack
        let mut state: Vec<usize> = Vec::new();
        // signs stack
        let mut signs: Vec<Letter> = Vec::new();
        // final parse tree, equals to signs stack, but is a tree
        let mut tree: Vec<ParseTree> = Vec::new();


        let mut letter_stream: Vec<Letter> = token_stream.stream.iter().map(
            |x| if x.ttype != "EOF" { Letter::Terminal(x.ttype.clone()) } else { Letter::EndSign }
        ).collect();


        // table
        let mut table = Table::new();
        
        // initial state
        state.push(0);
        // end sign
        signs.push(Letter::EndSign);

        // i iterate over tokens
        let mut i = 0;
        while i < letter_stream.len() {
            let mut row: Vec<Cell> = Vec::new();
            row.push(Cell::new(&utils::vec_to_string(&state, "")));
            row.push(Cell::new(&utils::vec_to_string(&signs, "")));
            row.push(Cell::new(&utils::vec_to_string(&letter_stream[i..], "")));

            // current state, top of the state stack
            let current_state = match state.last() {
                Some(&x) => x,
                None => panic!("error, this cannot happen"),
            };

            if current_state >= self.action.len() {
                panic!("this cannot happen, in lr_analysis()");
            }

            // current letter
            let input_letter = letter_stream[i].clone();

            match self.action[current_state].get(&input_letter).unwrap() {
                ActionTableItem::Err => {
                    row.push(Cell::new("error"));
                    table.add_row(Row::new(row));
                    return (None, table);
                },
                ActionTableItem::Accept => {
                    row.push(Cell::new("acc"));
                    table.add_row(Row::new(row));
                    break;
                },
                ActionTableItem::Step(next_state) => {
                    // 移进

                    // push to sign stack
                    signs.push(input_letter.clone());
                    // push to state stack
                    state.push(*next_state);
                    // update parse tree stack
                    tree.push(ParseTree::Leaf(token_stream.stream[i].clone()));
                    // iterate next token
                    i += 1;
                    
                    row.push(Cell::new(&format!("S{}", *next_state)));
                }
                ActionTableItem::Reduce(rule) => {
                    // 规约

                    // reduce to node
                    let mut node = NonLeafNode::new(rule.left.get_str());

                    let mut it1 = rule.right.len() - 1;
                    let mut it2 = signs.len() - 1;
                    // contruct new tree and update stack
                    while rule.right[it1] == signs[it2] {
                        node.push_child_front(tree.pop().unwrap());

                        state.pop();
                        signs.pop();

                        if it1 == 0 {
                            break;
                        }

                        it1 -= 1;
                        it2 -= 1;
                    }
                    node.children = node.children.into_iter().rev().collect();
                    

                    // reduce to what sign
                    let reduced = rule.left.clone();
                    // top of the sign stack after refuced
                    let temp_state = *state.last().unwrap();
                    // push new state according to goto table
                    state.push(self.goto[temp_state].get(&reduced).unwrap().get_state());
                    // push reduced sign to sign stack
                    signs.push(reduced);
                    // update tree
                    tree.push(ParseTree::NonLeaf(node));


                    // update row
                    row.push(Cell::new(&rule.to_string()));
                }
            }

            table.add_row(Row::new(row));
        }

        if tree.len() != 1 {
            panic!("this cannot happen, in lr_analysis()");
        }

        (Some(tree.pop().unwrap()), table)
    }

    pub fn analysis(&self, token_stream: &TokenStream) -> Option<ParseTree> {
        // state stack
        let mut state: Vec<usize> = Vec::new();
        // signs stack
        let mut signs: Vec<Letter> = Vec::new();
        // final parse tree, equals to signs stack, but is a tree
        let mut tree: Vec<ParseTree> = Vec::new();


        let mut letter_stream: Vec<Letter> = token_stream.stream.iter().map(
            |x| if x.ttype != "EOF" { Letter::Terminal(x.ttype.clone()) } else { Letter::EndSign }
        ).collect();

        
        // initial state
        state.push(0);
        // end sign
        signs.push(Letter::EndSign);

        // i iterate over tokens
        let mut i = 0;
        while i < letter_stream.len() {
            // current state, top of the state stack
            let current_state = match state.last() {
                Some(&x) => x,
                None => panic!("error, this cannot happen"),
            };

            if current_state >= self.action.len() {
                panic!("this cannot happen, in lr_analysis()");
            }

            // current letter
            let input_letter = letter_stream[i].clone();

            match self.action[current_state].get(&input_letter).unwrap() {
                ActionTableItem::Err => {
                    return None;
                },
                ActionTableItem::Accept => {
                    break;
                },
                ActionTableItem::Step(next_state) => {
                    // 移进

                    // push to sign stack
                    signs.push(input_letter.clone());
                    // push to state stack
                    state.push(*next_state);
                    // update parse tree stack
                    tree.push(ParseTree::Leaf(token_stream.stream[i].clone()));
                    // iterate next token
                    i += 1;
                }
                ActionTableItem::Reduce(rule) => {
                    // 规约

                    // reduce to node
                    let mut node = NonLeafNode::new(rule.left.get_str());

                    let mut it1 = rule.right.len() - 1;
                    let mut it2 = signs.len() - 1;
                    // contruct new tree and update stack
                    while rule.right[it1] == signs[it2] {
                        node.push_child_front(tree.pop().unwrap());

                        state.pop();
                        signs.pop();

                        if it1 == 0 {
                            break;
                        }

                        it1 -= 1;
                        it2 -= 1;
                    }
                    node.children = node.children.into_iter().rev().collect();
                    

                    // reduce to what sign
                    let reduced = rule.left.clone();
                    // top of the sign stack after refuced
                    let temp_state = *state.last().unwrap();
                    // push new state according to goto table
                    state.push(self.goto[temp_state].get(&reduced).unwrap().get_state());
                    // push reduced sign to sign stack
                    signs.push(reduced);
                    // update tree
                    tree.push(ParseTree::NonLeaf(node));
                }
            }
        }

        if tree.len() != 1 {
            panic!("this cannot happen, in lr_analysis()");
        }

        Some(tree.pop().unwrap())
    }
}


