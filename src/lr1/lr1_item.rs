use crate::grammar::letter::Letter;
use crate::grammar::sentence::Sentence;
use crate::grammar::rule::Rule;
use std::fmt;
use std::hash::{Hasher, Hash};
use std::collections::hash_map::DefaultHasher;


#[derive(Clone, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct LR1Item {
    pub left: Letter,
    pub x: Vec<Letter>,
    pub y: Vec<Letter>,
    pub expected: Letter,
}

impl fmt::Display for LR1Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> ", self.left.get_str()).unwrap();
        for i in &self.x {
            write!(f, "{}", i).unwrap();
        }
        write!(f, ":").unwrap();
        for i in &self.y {
            write!(f, "{}", i).unwrap();
        }
        write!(f, ", {}", self.expected).unwrap();
        Ok(())
    }
}

impl fmt::Debug for LR1Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl LR1Item {
    pub fn from(left: Letter, x: Vec<Letter>, y: Vec<Letter>, expected: Letter) -> LR1Item {
        LR1Item {
            left, x, y, expected
        }
    }

    pub fn new_initial(left: Letter, y: Vec<Letter>) -> LR1Item {
        LR1Item {
            left,
            x: Vec::new(),
            y,
            expected: Letter::EndSign
        }
    }

    pub fn hash_lr0<H: Hasher>(&self, state: &mut H) {
        self.left.hash(state);
        state.write_i32(666);
        for i in self.x.iter() {
            i.hash(state);
        }
        state.write_i32(666);
        for i in self.y.iter() {
            i.hash(state);
        }
    }

    pub fn hash_lr0_value(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash_lr0(&mut hasher);
        hasher.finish()
    }

    pub fn is_lr0_equal(&self, other: &Self) -> bool {
        if self.left != other.left {
            return false;
        }
        for (u, v) in self.x.iter().zip(other.x.iter()) {
            if u != v {
                return false;
            }
        }
        for (u, v) in self.y.iter().zip(other.y.iter()) {
            if u != v {
                return false;
            }
        }
        true
    }

    pub fn rule(&self) -> Rule {
        let mut temp: Vec<Letter> = Vec::new();
        temp.extend_from_slice(&self.x);
        temp.extend_from_slice(&self.y);

        if temp.len() == 0 {
            temp.push(Letter::Empty);
        }

        Rule {
            left: self.left.clone(),
            right: temp
        }
    }

    pub fn skip1_right(&self) -> Sentence {
        Sentence::from_slice(&self.y[1..])
    }

    pub fn shift_right(&self) -> LR1Item {
        let mut ans = LR1Item {
            left: self.left.clone(),
            x: self.x.clone(),
            y: Vec::new(),
            expected: self.expected.clone()
        };

        ans.x.push(self.y[0].clone());

        for i in 1..self.y.len() {
            ans.y.push(self.y[i].clone());
        }

        ans
    }
}