pub mod lr1_item;
pub mod lr1_item_set;
pub mod lr1_item_set_family;


// use self::lr1_item::LR1Item;
use self::lr1_item_set::LR1ItemSet;
use self::lr1_item_set_family::LR1ItemSetFamily;
use crate::lr_table::{GotoTableItem, ActionTableItem, LRTable};
use crate::grammar::letter::Letter;
use crate::grammar::Grammar;
use std::collections::{HashMap, HashSet};
use std::iter;


fn is_lr1_grammar_helper(data: &LR1ItemSetFamily) -> bool {
    for (i, item_set) in data.item_sets.iter().enumerate() {
        let mut h: HashSet<Letter> = HashSet::new();
        let mut step_set: HashSet<Letter> = HashSet::new();

        for item in item_set.items.iter() {
            // reducible item
            if item.y.len() == 0 {
                if h.contains(&item.expected) {
                    // println!("aaa: {} {:?}", i, h);
                    return false;
                } else {
                    h.insert(item.expected.clone());
                }
            }

            // step item
            if item.y.len() >= 1 && item.y[0].is_terminal() {
                step_set.insert(item.y[0].clone());
            }
        }

        for v in step_set.iter() {
            if h.contains(v) {
                return false;
            }
        }
    }

    true
}

pub fn is_lr1_grammar(g: &Grammar) -> bool {
    let data = LR1ItemSetFamily::from_grammar(g);
    is_lr1_grammar_helper(&data)
}

pub fn lr_table_from_grammar_with_seed(g: &Grammar, seed: u64) -> Result<LRTable, String> {
    let data = LR1ItemSetFamily::from_grammar_with_seed(g, seed);
    let start = &g.origin;

    if !is_lr1_grammar_helper(&data) {
        return Err(String::from("not an LR(1) grammar"));
    }

    let mut goto: Vec<HashMap<Letter, GotoTableItem>> = Vec::new();
    let mut action: Vec<HashMap<Letter, ActionTableItem>> = Vec::new();

    let non_terminals = &g.non_terminals;
    let terminals = &g.terminals;
    // terminals.sort();

    // iterate over all states
    for state in 0..data.item_sets.len() {
        let item_set = &data.item_sets[state];
        let mut goto_item: HashMap<Letter, GotoTableItem> = HashMap::new();
        let mut action_item: HashMap<Letter, ActionTableItem> = HashMap::new();

        // goto table
        for nt in non_terminals {
            if !data.go_function[state].get(nt).unwrap().is_err() {
                goto_item.insert(nt.clone(), GotoTableItem::Step(data.go_function[state].get(nt).unwrap().value()));
            } else {
                goto_item.insert(nt.clone(), GotoTableItem::Err);
            }
        }
        goto.push(goto_item);


        for item in item_set.items.iter() {
            // reducible item
            if item.y.len() == 0 {
                if item.left == *start && item.expected.is_end_sign() {
                    action_item.insert(Letter::EndSign, ActionTableItem::Accept);
                } else {
                    action_item.insert(item.expected.clone(), ActionTableItem::Reduce(item.rule()));
                }
            }

            // step item
            if item.y.len() >= 1 && item.y[0].is_terminal() {
                action_item.insert(item.y[0].clone(), ActionTableItem::Step(
                    data.go_function[state].get(&item.y[0]).unwrap().value()
                ));
            }
        }

        for i in terminals.iter().chain(iter::once(&Letter::EndSign)) {
            if !action_item.contains_key(i) {
                action_item.insert(i.clone(), ActionTableItem::Err);
            }
        }

        action.push(action_item);
    }

    Ok(LRTable {
        goto, action
    })
}

pub fn lr_table_from_grammar(g: &Grammar) -> Result<LRTable, String> {
    lr_table_from_grammar_with_seed(g, 0)
}