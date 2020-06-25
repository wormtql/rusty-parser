use crate::grammar::Grammar;
use crate::grammar::letter::Letter;
use crate::grammar::rule::Rule;
use crate::ll_table::{LLTable, LLTableItem};
use crate::first_follow_set::FirstFollowSet;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter;

fn contains(set: &Vec<Letter>, letter: &Letter) -> bool {
    for i in set.iter() {
        if i == letter {
            return true;
        }
    }
    false
}

fn is_ll1_grammar_helper(ffset: &FirstFollowSet, rule_set: &HashMap<Letter, Vec<Rule>>) -> bool {
    // let non_terminals = &g.non_terminals;
    // let empties: HashSet<Letter> = g.calc_empties();
    
    for (letter, rules) in rule_set.iter() {
        let mut h: HashSet<Letter> = HashSet::new();
        let follow_set: &Vec<Letter> = ffset.follow_set.get(letter).unwrap();

        for rule in rules.iter() {
            
            let first_set: Vec<Letter> = ffset.first_set_of_sentence(&rule.right_sentence());
            for i in first_set.iter() {
                if i.is_empty() {
                    continue;
                }
                if h.contains(i) {
                    return false;
                } else {
                    h.insert(i.clone());
                }
            }
            if contains(&first_set, &Letter::Empty) {
                for i in follow_set.iter() {
                    if h.contains(i) {
                        return false;
                    } else {
                        h.insert(i.clone());
                    }
                }
            }
        }
    }

    true
}

pub fn is_ll1_grammar(g: &Grammar) -> bool {
    let ffset = FirstFollowSet::from_grammar(g);
    let mut rule_set: HashMap<Letter, Vec<Rule>> = HashMap::new();

    for rule in g.rules.iter() {
        let rules = rule_set.entry(rule.left.clone()).or_insert(Vec::new());
        rules.push((*rule).clone());
    }

    is_ll1_grammar_helper(&ffset, &rule_set)
}

pub fn ll_table_from_grammar(g: &Grammar) -> Result<LLTable, String> {
    let ffset = FirstFollowSet::from_grammar(g);
    let mut rule_set: HashMap<Letter, Vec<Rule>> = HashMap::new();
    let non_terminals = &g.non_terminals;
    let terminals = &g.terminals;

    for rule in g.rules.iter() {
        let rules = rule_set.entry(rule.left.clone()).or_insert(Vec::new());
        rules.push((*rule).clone());
    }

    if !is_ll1_grammar_helper(&ffset, &rule_set) {
        return Err(String::from("not an LL(1) grammar"));
    }

    let mut data: HashMap<Letter, HashMap<Letter, LLTableItem>> = HashMap::new();

    for nt in non_terminals.iter() {
        let follow_set: &Vec<Letter> = ffset.follow_set.get(nt).unwrap();
        let mut table_row: HashMap<Letter, LLTableItem> = HashMap::new();

        for rule in rule_set.get(nt).unwrap().iter() {
            let first_set: Vec<Letter> = ffset.first_set_of_sentence(&rule.right_sentence());

            for i in first_set.iter() {
                if i.is_empty() {
                    continue;
                }
                table_row.insert(i.clone(), LLTableItem::Action((*rule).clone()));
            }

            if contains(&first_set, &Letter::Empty) {
                for i in follow_set.iter() {
                    table_row.insert(i.clone(), LLTableItem::Action((*rule).clone()));
                }
            }
        }

        // fill error
        for t in terminals.iter().chain(iter::once(&Letter::EndSign)) {
            if !table_row.contains_key(t) {
                table_row.insert(t.clone(), LLTableItem::Err);
            }
        }

        data.insert(nt.clone(), table_row);
    }

    Ok(LLTable {
        data
    })
}