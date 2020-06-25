use super::lr1_item::LR1Item;
use crate::grammar::letter::Letter;
use crate::grammar::rule::Rule;
use crate::grammar::Grammar;
use crate::first_follow_set::FirstFollowSet;
use std::fmt;
use std::collections::{HashMap, HashSet};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};


#[derive(Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct LR1ItemSet {
    pub items: Vec<LR1Item>
}

impl fmt::Display for LR1ItemSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in &self.items {
            write!(f, "{}\n", i).unwrap();
        }
        Ok(())
    }
}

impl fmt::Debug for LR1ItemSet {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl LR1ItemSet {
    pub fn new() -> LR1ItemSet {
        LR1ItemSet {
            items: Vec::new()
        }
    }

    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    pub fn from_grammar_initial(g: &Grammar) -> LR1ItemSet {
        let mut items: Vec<LR1Item> = Vec::new();

        for rule in g.rules.iter() {
            if rule.left == g.origin {
                if rule.is_empty() {
                    items.push(LR1Item::new_initial(rule.left.clone(), Vec::new()));
                } else {
                    items.push(LR1Item::new_initial(rule.left.clone(), rule.right.clone()));
                }
            }
        }

        items.sort();
        LR1ItemSet {
            items
        }
    }

    pub fn is_concentric(&self, other: &LR1ItemSet) -> bool {
        if self.items.len() != other.items.len() {
            return false;
        }

        for (x, y) in self.items.iter().zip(other.items.iter()) {
            if !x.is_lr0_equal(y) {
                return false;
            }
        }

        true
    }

    pub fn hash_lr0<H: Hasher>(&self, state: &mut H) {
        let mut temp: HashSet<u64> = HashSet::new();

        for item in self.items.iter() {
            let h = item.hash_lr0_value();
            temp.insert(h);
        }

        let mut temp: Vec<_> = temp.iter().cloned().collect();
        temp.sort();
        for i in temp {
            i.hash(state);
        }
    }

    pub fn hash_lr0_value(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash_lr0(&mut hasher);
        hasher.finish()
    }

    pub fn closure(&self, whole: &Vec<Rule>, ffset: &FirstFollowSet) -> LR1ItemSet {
        let mut h: HashMap<Letter, Vec<&Rule>> = HashMap::new();
        for i in whole.iter() {
            let v = h.entry(i.left.clone()).or_insert(Vec::new());
            v.push(i);
        }

        let mut vis: HashSet<LR1Item> = HashSet::new();
        // let mut ans: HashSet<LR1Item> = HashSet::new();
        let mut q: Vec<LR1Item> = Vec::new();
        // let mut flag = true;

        for i in &self.items {
            // vis.insert(i.clone());
            vis.insert(i.clone());
            q.push(i.clone());
        }

        while !q.is_empty() {
            let item = q.pop().unwrap();

            if item.y.len() >= 1 && item.y[0].is_non_terminal() {
                // sss -> xxx:Ayyy, a

                let mut sen = item.skip1_right();
                sen.push(item.expected.clone());

                let first_set: Vec<Letter> = ffset.first_set_of_sentence(&sen);

                for rule in h.get(&item.y[0]).unwrap() {
                    for expected in first_set.iter() {
                        if expected.is_empty() {
                            continue;
                        }
                        let new_item = LR1Item::from(
                            item.y[0].clone(),
                            Vec::new(),
                            if rule.is_empty() { Vec::new() } else { rule.right.clone() },
                            expected.clone()
                        );

                        if !vis.contains(&new_item) {
                            vis.insert(new_item.clone());
                            q.push(new_item);
                        }
                    }
                }
            }
        }

        let mut temp: Vec<_> = vis.iter().cloned().collect();
        temp.sort();
        LR1ItemSet {
            items: temp
        }
    }

    pub fn go(&self, whole: &Vec<Rule>, letter: &Letter, ffset: &FirstFollowSet) -> LR1ItemSet {
        if !letter.is_non_terminal() && !letter.is_terminal() {
            panic!("letter can only be non terminal or terminnal in go function");
        }

        let mut temp: Vec<LR1Item> = Vec::new();

        for item in &self.items {
            if item.y.len() >= 1 && item.y[0] == *letter {
                temp.push(item.shift_right());
            }
        }

        if !temp.is_empty() {
            LR1ItemSet {
                items: temp
            }.closure(whole, ffset)
        } else {
            LR1ItemSet::new()
        }
    }
}