use super::letter::Letter;
use super::sentence::Sentence;
// use letter::Letter;
use std::fmt;

use serde::{Serialize, Deserialize};

#[derive(PartialEq, Eq, Clone, Hash, Serialize, Deserialize)]
pub struct Rule {
    pub left: Letter,
    pub right: Vec<Letter>
}

impl fmt::Display for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} -> ", match &self.left {
            Letter::NonTerminal(ref x) => x,
            Letter::Terminal(ref x) => x,
            Letter::Empty => ".",
            Letter::EndSign => "#"
        }).unwrap();
        for i in &self.right {
            write!(f, "{}", i).unwrap();
        }
        Ok(())
    }
}

impl fmt::Debug for Rule {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Rule {
    pub fn new(left: Letter, right: Vec<Letter>) -> Rule {
        Rule {
            left, right
        }
    }

    pub fn from_single(left: &str, right: &str) -> Rule {
        let left_letter = Letter::NonTerminal(String::from(left));
        let mut rights: Vec<Letter> = Vec::new();

        let mut flag = false;
        let mut temp_str = String::new();
        for c in right.chars() {
            if flag {
                if c == '}' {
                    flag = false;
                    rights.push(Letter::NonTerminal(temp_str.clone()));
                    temp_str.clear();
                } else {
                    temp_str.push(c);
                }
            } else {
                if c == '{' {
                    flag = true;
                } else if c == '.' {
                    rights.push(Letter::Empty);
                    // panic!("empty sign in rule");
                } else {
                    rights.push(Letter::Terminal(c.to_string()));
                }
            }
        }

        Rule {
            left: left_letter,
            right: rights
        }
    }

    pub fn rules_from_str(s: &str) -> Result<Vec<Rule>, String> {
        let temp: Vec<&str> = s.split(' ').collect();
        if temp.len() != 2 {
            return Err(String::from("wrong format"));
        }

        // let left_letter = Letter::NonTerminal(String::from(temp[0]));

        let mut ans: Vec<Rule> = Vec::new();

        let rights: Vec<&str> = temp[1].split('|').collect();
        for right in rights {
            let rule = Rule::from_single(temp[0], right);
            ans.push(rule);
        }

        Ok(ans)
    }

    pub fn right_sentence(&self) -> Sentence {
        Sentence::from_slice(&self.right)
    }

    pub fn is_empty(&self) -> bool {
        self.right.len() == 1 && self.right[0].is_empty()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_rule_1() {
        let r = Rule::from_single("S", "A{B}");
        assert_eq!(r, Rule {
            left: Letter::NonTerminal(String::from("S")),
            right: vec![
                Letter::Terminal(String::from("A")),
                Letter::NonTerminal(String::from("B"))
            ]
        });
    }

    #[test]
    fn test_rule_2() {
        let r = Rule::from_single("S", "A{Second}.");
        assert_eq!(r, Rule {
            left: Letter::NonTerminal(String::from("S")),
            right: vec![
                Letter::Terminal(String::from("A")),
                Letter::NonTerminal(String::from("Second")),
                Letter::Empty
            ]
        });
    }

    #[test]
    fn test_rule_3() {
        let r = Rule::from_single("S", "{First}{Second}3");
        assert_eq!(r, Rule {
            left: Letter::NonTerminal(String::from("S")),
            right: vec![
                Letter::NonTerminal(String::from("First")),
                Letter::NonTerminal(String::from("Second")),
                Letter::Terminal(String::from("3"))
            ]
        });
    }

    #[test]
    fn test_rule_4() {
        let r1 = Rule::from_single("S", "A{Second}.");
        let r2 = Rule::from_single("S", "{First}{Second}3");
        let rules = Rule::rules_from_str("S A{Second}.|{First}{Second}3");
        assert_eq!(rules, Ok(vec![r1, r2]));
    }
}