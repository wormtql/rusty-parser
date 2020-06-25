use super::lr0_item::{LR0Item};
use crate::grammar::letter::Letter;
use crate::grammar::Grammar;
use std::fmt;
use std::collections::HashMap;
use std::collections::HashSet;

#[derive(Clone, Hash, Eq, PartialEq)]
pub struct LR0ItemSet {
    pub items: Vec<LR0Item>
}

impl fmt::Display for LR0ItemSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in &self.items {
            write!(f, "{}\n", i).unwrap();
        }
        Ok(())
    }
}

impl LR0ItemSet {
    pub fn new() -> LR0ItemSet {
        LR0ItemSet {
            items: Vec::new()
        }
    }

    fn organize(&mut self) {
        self.items.sort();
    }

    // pub fn type(&self) -> LR0ItemType {

    // }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn from_grammar(grammar: &Grammar) -> LR0ItemSet {
        let mut ans = LR0ItemSet::new();

        for rule in &grammar.rules {
            if rule.right.len() == 1 && rule.right[0].is_empty() {
                // xxx -> .
                ans.items.push(LR0Item::new_empty(rule.left.clone()));
            } else {
                for i in 0..=rule.right.len() {
                    ans.items.push(LR0Item::from_rule_and_position(rule, i));
                }
            }
        }

        ans.organize();

        ans
    }

    pub fn from_grammar_initial(grammar: &Grammar) -> LR0ItemSet {
        let mut ans = LR0ItemSet::new();

        for rule in &grammar.rules {
            if rule.left == grammar.origin {
                ans.items.push(LR0Item::from_rule_and_position(rule, 0));
            }
        }

        ans.organize();

        ans
    }

    pub fn add_item(&mut self, item: LR0Item) {
        self.items.push(item);
    }

    pub fn go(&self, whole: &LR0ItemSet, letter: &Letter) -> LR0ItemSet {
        if !letter.is_non_terminal() && !letter.is_terminal() {
            panic!("letter can only be non terminal or terminnal in go function");
        }

        let mut temp = LR0ItemSet::new();

        for item in &self.items {
            if item.y.len() >= 1 && item.y[0] == *letter {
                temp.add_item(item.shift_right());
            }
        }

        if !temp.is_empty() {
            temp.closure(whole)
        } else {
            temp
        }
    }

    pub fn closure(&self, whole: &LR0ItemSet) -> LR0ItemSet {
        let mut h: HashMap<Letter, Vec<&LR0Item>> = HashMap::new();

        for i in &whole.items {
            let v = h.entry(i.left.clone()).or_insert(Vec::new());
            v.push(i);
        }
        // println!("aaa: {:?}\n", h);

        // let mut ans = self.clone();
        let mut vis: HashSet<LR0Item> = HashSet::new();
        let mut ans: HashSet<LR0Item> = HashSet::new();
        let mut flag = true;

        for i in &self.items {
            vis.insert(i.clone());
            ans.insert(i.clone());
        }

        while flag {
            flag = false;
            for item in &ans {
                
                if item.y.len() >= 1 && item.y[0].is_non_terminal() {
                    // xxx -> xxx : Axxx
                    for i in h.get(&item.y[0]).unwrap() {
                        if i.x.len() == 0 {
                            // A -> : xxx
                            if !vis.contains(i) {
                                flag = true;
                                vis.insert((*i).clone());
                            }
                        }
                    }
                }
            }
            for item in &vis {
                if !ans.contains(item) {
                    ans.insert(item.clone());
                }
            }
        }
        
        let mut temp = LR0ItemSet {
            items: ans.iter().cloned().collect()
        };
        temp.organize();
        temp
    }
}