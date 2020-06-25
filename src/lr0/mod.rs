pub mod lr0_item;
pub mod lr0_item_set;
pub mod lr0_item_set_family;

use std::collections::{HashMap, HashSet};
use self::lr0_item_set_family::LR0ItemSetFamily;
use crate::grammar::Grammar;
use crate::grammar::rule::Rule;
use crate::grammar::letter::Letter;
use crate::lr_table::{GotoTableItem, ActionTableItem, LRTable};


pub fn is_lr0_grammar(g: &Grammar) -> bool {
    let data = LR0ItemSetFamily::from_grammar(g);

    is_lr0_grammar_helper(&data)
}

fn is_lr0_grammar_helper(data: &LR0ItemSetFamily) -> bool {
    for item_set in data.item_sets.iter() {
        let mut step_set: HashSet<Letter> = HashSet::new();
        // let mut r_set: HashSet<Letter> = HashSet::new();
        let mut r_rule: HashSet<Rule> = HashSet::new();

        for item in item_set.items.iter() {
            if item.y.len() == 0 {
                let rule = item.rule();
                if r_rule.len() >= 1 {
                    return false;
                } else {
                    r_rule.insert(rule);
                }
            } else if item.y.len() >= 1 && item.y[0].is_terminal() {
                step_set.insert(item.y[0].clone());
            }
        }

        if !step_set.is_empty() && !r_rule.is_empty() {
            // step -- reduce conflict
            return false;
        }
    }

    true
}

pub fn lr_table_from_grammar_with_seed(g: &Grammar, seed: u64) -> Result<LRTable, String> {
    let data = LR0ItemSetFamily::from_grammar_with_seed(g, seed);
    let start = &g.origin;

    if !is_lr0_grammar_helper(&data) {
        return Err(String::from("not an LR(0) grammar"));
    }


    let mut goto: Vec<HashMap<Letter, GotoTableItem>> = Vec::new();
    let mut action: Vec<HashMap<Letter, ActionTableItem>> = Vec::new();

    let non_terminals = &g.non_terminals;
    let terminals = &g.terminals;

    let end_letter = Letter::EndSign;

    for state in 0..data.item_sets.len() {
        let item_set = &data.item_sets[state];
        let mut goto_item: HashMap<Letter, GotoTableItem> = HashMap::new();
        let mut action_item: HashMap<Letter, ActionTableItem> = HashMap::new();

        for nt in non_terminals {
            if !data.go_function[state].get(nt).unwrap().is_err() {
                goto_item.insert(nt.clone(), GotoTableItem::Step(data.go_function[state].get(nt).unwrap().value()));
            } else {
                goto_item.insert(nt.clone(), GotoTableItem::Err);
            }
        }
        goto.push(goto_item);


        let mut flag = false;
        for item in &item_set.items {
            if item.y.len() == 0 {
                // reduce
                flag = true;
                if item.left == *start {
                    action_item.insert(end_letter.clone(), ActionTableItem::Accept);
                    for t in terminals.iter() {
                        action_item.insert(t.clone(), ActionTableItem::Err);
                    }
                } else {
                    for t in terminals {
                        action_item.insert(t.clone(), ActionTableItem::Reduce(item.rule()));
                    }
                    action_item.insert(end_letter.clone(), ActionTableItem::Reduce(item.rule()));
                }
                break;
            }
        }

        // step item
        if flag == false {
            for t in terminals {
                if !data.go_function[state].get(t).unwrap().is_err() {
                    action_item.insert(t.clone(), ActionTableItem::Step(data.go_function[state].get(t).unwrap().value()));
                } else {
                    // if !action_item.contains_key(t) {
                        action_item.insert(t.clone(), ActionTableItem::Err);
                    // }
                }
            }
            // if !action_item.contains_key(&end_letter) {
                action_item.insert(end_letter.clone(), ActionTableItem::Err);
            // }
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