use crate::lr1::lr1_item_set_family::LR1ItemSetFamily;
use crate::lr1::lr1_item::LR1Item;
use std::collections::{HashMap, HashSet};
// use std::hash::Hash;
use std::iter;
use crate::lr_table::LALRTable;
use crate::grammar::letter::Letter;
use crate::grammar::Grammar;
use crate::lr_table::{GotoTableItem, ActionTableItem};


fn is_lalr1_grammar_helper(data: &LR1ItemSetFamily, hash_lr0: &HashMap<u64, Vec<usize>>) -> bool
{
    // println!("{:?}", hash_lr0);
    for sets in hash_lr0.values() {
        // println!("bbb");
        let mut h: HashSet<Letter> = HashSet::new();
        let mut step_set: HashSet<Letter> = HashSet::new();
        let mut merge: HashSet<LR1Item> = HashSet::new();

        for set in sets.iter() {
            let item_set = &data.item_sets[*set];

            for item in item_set.items.iter() {
                if merge.contains(item) {
                    continue;
                } else {
                    merge.insert(item.clone());
                }
                if item.y.len() == 0 {
                    if h.contains(&item.expected) {
                        // println!("aaa: {:?}", h);
                        return false;
                    } else {
                        h.insert(item.expected.clone());
                    }
                } else if item.y.len() >= 1 && item.y[0].is_terminal() {
                    step_set.insert(item.y[0].clone());
                }
            }
        }

        for v in step_set.iter() {
            if h.contains(v) {
                panic!("step reduce conflict is not possible");
                // return false;
            }
        }
    }

    true
}


pub fn is_lalr1_grammar(g: &Grammar) -> bool {
    let mut hash_lr0: HashMap<u64, Vec<usize>> = HashMap::new();

    let data = LR1ItemSetFamily::from_grammar(g);

    for (i, item_set) in data.item_sets.iter().enumerate() {
        let hash = item_set.hash_lr0_value();
        let entry = hash_lr0.entry(hash).or_insert(Vec::new());
        entry.push(i);
    }


    is_lalr1_grammar_helper(&data, &hash_lr0)
}

pub fn lalr_table_from_grammar_with_seed(g: &Grammar, seed: u64) -> Result<LALRTable, String> {
    let mut hash_lr0: HashMap<u64, Vec<usize>> = HashMap::new();
    let mut state2hash: HashMap<usize, u64> = HashMap::new();

    let data = LR1ItemSetFamily::from_grammar_with_seed(g, seed);

    for (i, item_set) in data.item_sets.iter().enumerate() {
        let hash = item_set.hash_lr0_value();
        let entry = hash_lr0.entry(hash).or_insert(Vec::new());
        entry.push(i);
        state2hash.insert(i, hash);
    }


    if !is_lalr1_grammar_helper(&data, &hash_lr0) {
        return Err(String::from("not an LALR(1) grammar"));
    }


    let mut goto: Vec<HashMap<Letter, GotoTableItem>> = Vec::new();
    let mut action: Vec<HashMap<Letter, ActionTableItem>> = Vec::new();

    let non_terminals = &g.non_terminals;
    let terminals = &g.terminals;

    let mut state_vis: HashSet<u64> = HashSet::new();
    let mut hash_to_new_state: HashMap<u64, usize> = HashMap::new();
    let mut new_state_count = 0;
    let mut state_map: Vec<Vec<usize>> = Vec::new();

    // determine new state number
    for state in 0..data.item_sets.len() {
        let state_hash = *state2hash.get(&state).unwrap();
        if state_vis.contains(&state_hash) {
            state_map[*hash_to_new_state.get(&state_hash).unwrap()].push(state);
            continue;
        }
        state_vis.insert(state_hash);
        state_map.push(vec![state]);
        hash_to_new_state.insert(state_hash, new_state_count);
        new_state_count += 1;
    }

    // println!("h2ns: {:?}", hash_to_new_state);
    // println!("sm: {:?}", state_map);

    state_vis.clear();
    // iterate over all states
    for state in 0..data.item_sets.len() {
        let state_hash = state2hash.get(&state).unwrap();
        if state_vis.contains(state_hash) {
            continue;
        } else {
            state_vis.insert(*state_hash);
        }


        let mut goto_item: HashMap<Letter, GotoTableItem> = HashMap::new();
        let mut action_item: HashMap<Letter, ActionTableItem> = HashMap::new();

        let mut merge: HashSet<LR1Item> = HashSet::new();

        for set in hash_lr0.get(&state_hash).unwrap().iter() {
            // println!("aaa");
            let item_set = &data.item_sets[*set];
            // println!("aaa");
    
            for item in item_set.items.iter() {
                if merge.contains(item) {
                    continue;
                } else {
                    merge.insert(item.clone());
                }
                if item.y.len() == 0 {
                    if item.left == g.origin && item.expected.is_end_sign() {
                        action_item.insert(Letter::EndSign, ActionTableItem::Accept);
                    } else {
                        action_item.insert(item.expected.clone(), ActionTableItem::Reduce(item.rule()));
                    }
                } else if item.y.len() >= 1 && item.y[0].is_terminal() {
                    let step_state = data.go_function[state].get(&item.y[0]).unwrap().value();
                    let step_hash = *state2hash.get(&step_state).unwrap();
                    action_item.insert(item.y[0].clone(), ActionTableItem::Step(
                        *hash_to_new_state.get(&step_hash).unwrap()
                    ));
                } else if item.y.len() >= 1 && item.y[0].is_non_terminal() {
                    let step_state = data.go_function[state].get(&item.y[0]).unwrap().value();
                    let step_hash = *state2hash.get(&step_state).unwrap();
                    goto_item.insert(
                        item.y[0].clone(),
                        GotoTableItem::Step(*hash_to_new_state.get(&step_hash).unwrap())
                    );
                }
            }
        }

        

        for i in terminals.iter().chain(iter::once(&Letter::EndSign)) {
            if !action_item.contains_key(i) {
                action_item.insert(i.clone(), ActionTableItem::Err);
            }
        }

        for i in non_terminals.iter() {
            if !goto_item.contains_key(i) {
                goto_item.insert(i.clone(), GotoTableItem::Err);
            }
        }

        goto.push(goto_item);
        action.push(action_item);
    }

    
    Ok(LALRTable {
        goto, action, state_map
    })
}

pub fn lalr_table_from_grammar(g: &Grammar) -> Result<LALRTable, String> {
    lalr_table_from_grammar_with_seed(g, 0)
}