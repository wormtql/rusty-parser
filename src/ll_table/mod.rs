use crate::grammar::letter::Letter;
use crate::grammar::rule::Rule;
use crate::token::{TokenStream, Token};
use crate::parse_tree::{ParseTree, NonLeafNode};
use crate::utils;

use std::fmt;
use std::collections::HashMap;

use prettytable::{Table, Row, Cell};

#[derive(Debug)]
pub enum LLTableItem {
    Err,
    Action(Rule)
}

impl LLTableItem {
    pub fn is_err(&self) -> bool {
        if let LLTableItem::Err = self {
            true
        } else {
            false
        }
    }

    pub fn value(&self) -> &Rule {
        if let LLTableItem::Action(r) = self {
            r
        } else {
            panic!("error")
        }
    }
}

pub struct LLTable {
    pub data: HashMap<Letter, HashMap<Letter, LLTableItem>>
}

impl fmt::Display for LLTable {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut header: Vec<Cell> = Vec::new();
        let mut table = Table::new();

        let mut terminals: Vec<Letter> = self.data.values().next().unwrap().keys().cloned().collect();
        let mut non_terminals: Vec<Letter> = self.data.keys().cloned().collect();
        terminals.sort();
        non_terminals.sort();

        header.push(Cell::new(""));
        for t in &terminals {
            header.push(Cell::new(&t.to_string()));
        }
        table.add_row(Row::new(header));

        for nt in &non_terminals {
            let mut row: Vec<Cell> = Vec::new();
            row.push(Cell::new(nt.get_str()));
            for t in terminals.iter() {
                let text = match self.data.get(nt).unwrap().get(t).unwrap() {
                    LLTableItem::Err => String::new(),
                    LLTableItem::Action(r) => r.to_string(),
                };
                row.push(Cell::new(&text));
            }
            table.add_row(Row::new(row));
        }

        write!(f, "{}", table)
    }
}

impl LLTable {
    pub fn analysis_with_process(&self, token_stream: &TokenStream, origin: Letter) -> (Option<ParseTree>, Table) {
        let mut table = Table::new();

        // analysis stack
        let mut stack: Vec<Letter> = Vec::new();
        // tree
        let mut tree_stack: Vec<ParseTree> = Vec::new();
        // letter stream
        let letter_stream: Vec<Letter> = token_stream.stream.iter().map(
            |x| if x.ttype != "EOF" { Letter::Terminal(x.ttype.clone()) } else { Letter::EndSign }
        ).collect();

        stack.push(Letter::EndSign);
        stack.push(origin);

        // iterate over token stream
        let mut i = 0;
        while i < letter_stream.len() {
            let mut row: Vec<Cell> = Vec::new();
            row.push(Cell::new(&utils::vec_to_string(&stack, "")));
            row.push(Cell::new(&utils::vec_to_string(&letter_stream[i..], "")));


            let letter1 = stack.last().unwrap();
            let letter2 = &letter_stream[i];

            if letter1.is_non_terminal() {
                let table_item = self.data.get(letter1).unwrap().get(letter2).unwrap();
                match table_item {
                    LLTableItem::Err => {
                        row.push(Cell::new("error"));
                        table.add_row(Row::new(row));
                        return (None, table);
                    },
                    LLTableItem::Action(rule) => {
                        row.push(Cell::new(&rule.to_string()));
    
                        stack.pop();
                        if !rule.is_empty() { 
                            for item in rule.right.iter().rev() {
                                stack.push(item.clone());
                            }
                        }
                    }
                }
            } else if letter1.is_terminal() {
                if letter1 == letter2 {
                    stack.pop();
                    i += 1;
                } else {
                    row.push(Cell::new("error"));
                    table.add_row(Row::new(row));
                    return (None, table);
                }
            } else if letter1.is_end_sign() {
                if letter1 == letter2 {
                    row.push(Cell::new("acc"));
                    i += 1;
                } else {
                    row.push(Cell::new("error"));
                    table.add_row(Row::new(row));
                    return (None, table);
                }
            }

            table.add_row(Row::new(row));
        }

        (None, table)
    }
}