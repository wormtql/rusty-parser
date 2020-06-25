use crate::grammar::letter::Letter;
use crate::grammar::rule::Rule;
use std::fmt;


pub enum LR0ItemType {
    Reduce,
    // Accept,
    Step,
    Pend
}

impl LR0ItemType {
    pub fn is_reduce(&self) -> bool {
        if let LR0ItemType::Reduce = self {
            true
        } else {
            false
        }
    }

    pub fn is_step(&self) -> bool {
        if let LR0ItemType::Step = self {
            true
        } else {
            false
        }
    }

    pub fn is_pend(&self) -> bool {
        if let LR0ItemType::Pend = self {
            true
        } else {
            false
        }
    }
}

// left -> x : y
#[derive(Clone, Hash, Eq, PartialEq, Ord, PartialOrd)]
pub struct LR0Item {
    pub left: Letter,
    pub x: Vec<Letter>,
    pub y: Vec<Letter>
}

impl fmt::Display for LR0Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> ", self.left.get_str()).unwrap();
        for i in &self.x {
            write!(f, "{}", i).unwrap();
        }
        write!(f, ":").unwrap();
        for i in &self.y {
            write!(f, "{}", i).unwrap();
        }
        // write!(f, "\n")
        Ok(())
    }
}

impl fmt::Debug for LR0Item {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl LR0Item {
    pub fn from(left: Letter, x: Vec<Letter>, y: Vec<Letter>) -> LR0Item {
        LR0Item {
            left, x, y
        }
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

    pub fn item_type(&self) -> LR0ItemType {
        if self.y.len() == 0 {
            LR0ItemType::Reduce
        } else {
            if self.y[0].is_non_terminal() {
                LR0ItemType::Pend
            } else if self.y[0].is_terminal() {
                LR0ItemType::Step
            } else {
                panic!("error")
            }
        }
    }

    pub fn new_empty(left: Letter) -> LR0Item {
        LR0Item {
            left,
            x: Vec::new(),
            y: Vec::new()
        }
    }

    pub fn from_rule_and_position(rule: &Rule, position: usize) -> LR0Item {
        let mut x: Vec<Letter> = Vec::new();
        let mut y: Vec<Letter> = Vec::new();

        for i in 0..position {
            x.push(rule.right[i].clone());
        }
        for i in position..rule.right.len() {
            y.push(rule.right[i].clone());
        }

        LR0Item {
            left: rule.left.clone(),
            x,
            y,
        }
    }

    pub fn shift_right(&self) -> LR0Item {
        let mut ans = LR0Item {
            left: self.left.clone(),
            x: self.x.clone(),
            y: Vec::new(),
        };

        ans.x.push(self.y[0].clone());

        for i in 1..self.y.len() {
            ans.y.push(self.y[i].clone());
        }

        ans
    }
}