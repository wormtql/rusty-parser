use crate::grammar::Grammar;
use crate::grammar::letter::Letter;
use crate::first_follow_set::FirstFollowSet;
use crate::lr_table::{LRTable, GotoTableItem, ActionTableItem};
use crate::lr0::lr0_item_set_family::LR0ItemSetFamily;
use std::collections::HashSet;
use std::collections::HashMap;
use std::iter;

fn is_slr1_grammar_helper(data: &LR0ItemSetFamily, follow_set: &HashMap<Letter, Vec<Letter>>) -> bool {
    for item_set in &data.item_sets {
        let mut h: HashSet<Letter> = HashSet::new();
        let mut step_set: HashSet<Letter> = HashSet::new();

        for item in &item_set.items {
            if item.y.len() == 0 {
                for i in follow_set.get(&item.left).unwrap() {
                    if h.contains(i) {
                        return false;
                    } else {
                        h.insert(i.clone());
                    }
                }
            } else if item.y.len() >= 1 && item.y[0].is_terminal() {
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

pub fn is_slr1_grammar(g: &Grammar) -> bool {
    let item_set_family = LR0ItemSetFamily::from_grammar(g);
    let ffset = FirstFollowSet::from_grammar(g);

    is_slr1_grammar_helper(&item_set_family, &ffset.follow_set)
}

pub fn lr_table_from_grammar_with_seed(g: &Grammar, seed: u64) -> Result<LRTable, String> {
    // LR(0) item set family and go function
    let item_set_family = LR0ItemSetFamily::from_grammar_with_seed(g, seed);
    println!("{}", item_set_family);
    // first follow set
    let ffset = FirstFollowSet::from_grammar(g);
    let follow_set = ffset.follow_set;

    // if is SLR(1) grammar
    if !is_slr1_grammar_helper(&item_set_family, &follow_set) {
        return Err(String::from("not an SLR(1) grammar"));
    }

    // goto table, index is state id
    let mut goto: Vec<HashMap<Letter, GotoTableItem>> = Vec::new();
    // action table, index is state id
    let mut action: Vec<HashMap<Letter, ActionTableItem>> = Vec::new();

    // all nonterminals
    let non_terminals = &g.non_terminals;
    // all terminals
    let terminals = &g.terminals;

    // iterate over states
    for state in 0..item_set_family.item_sets.len() {
        // current item set
        let item_set = &item_set_family.item_sets[state];
        // goto table item
        let mut goto_item: HashMap<Letter, GotoTableItem> = HashMap::new();
        // action table item
        let mut action_item: HashMap<Letter, ActionTableItem> = HashMap::new();


        // set goto table item
        for nt in non_terminals {
            if !item_set_family.go_function[state].get(nt).unwrap().is_err() {
                goto_item.insert(
                    nt.clone(),
                    GotoTableItem::Step(item_set_family.go_function[state].get(nt).unwrap().value())
                );
            } else {
                goto_item.insert(nt.clone(), GotoTableItem::Err);
            }
        }
        goto.push(goto_item);

        // set action table item
        for item in &item_set.items {
            // get item type
            let item_type = item.item_type();

            if item_type.is_step() {
                // println!("aaa {} {}", state, item.y[0]);
                action_item.insert(item.y[0].clone(), ActionTableItem::Step(
                    item_set_family.go_function[state].get(&item.y[0]).unwrap().value()
                ));
            } else if item_type.is_reduce() {
                for i in follow_set.get(&item.left).unwrap() {
                    if item.left == g.origin {
                        action_item.insert(i.clone(), ActionTableItem::Accept);
                    } else {
                        action_item.insert(i.clone(), ActionTableItem::Reduce(item.rule()));
                    }
                }
            }
        }

        // fill error
        for t in terminals.iter().chain(iter::once(&Letter::EndSign)) {
            if !action_item.contains_key(t) {
                action_item.insert(t.clone(), ActionTableItem::Err);
            }
        }
        action.push(action_item);
    }

    Ok(LRTable {
        action, goto
    })
}

pub fn lr_table_from_grammar(g: &Grammar) -> Result<LRTable, String> {
    lr_table_from_grammar_with_seed(g, 0)
}