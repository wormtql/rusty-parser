
// terminals and non terminals;
pub mod letter;
// grammar rule
pub mod rule;
pub mod sentence;

use std::collections::HashSet;
use letter::Letter;
use rule::Rule;
use std::fmt;
use std::fs;
use std::iter;


#[derive(PartialEq, Eq, Clone)]
pub struct Grammar {
    // rules
    pub rules: Vec<Rule>,

    // origin
    pub origin: Letter,

    // terminals
    pub terminals: Vec<Letter>,

    // non terminals
    pub non_terminals: Vec<Letter>,

    // A =*> .
    // pub empties: Vec<Letter>
}

impl fmt::Display for Grammar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "rules:\n").unwrap();
        for rule in &self.rules {
            write!(f, "    {}\n", rule).unwrap();
        }

        write!(f, "origin:\n    {}\n", self.origin.get_str()).unwrap();

        write!(f, "terminals:\n    ").unwrap();
        for i in &self.terminals {
            write!(f, "{}, ", i).unwrap();
        }

        write!(f, "\nnon terminals:\n    ").unwrap();
        for i in &self.non_terminals {
            write!(f, "{}, ", i.get_str()).unwrap();
        }

        Ok(())
    }
}

impl fmt::Debug for Grammar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Grammar {
    pub fn new() -> Grammar {
        Grammar {
            rules: Vec::new(),
            origin: letter::Letter::Empty,
            terminals: Vec::new(),
            non_terminals: Vec::new(),
            // empties: Vec::new()
        }
    }

    pub fn calc_empties(&self) -> HashSet<Letter> {
        let mut h: HashSet<Letter> = HashSet::new();

        for rule in &self.rules {
            if let Letter::Empty = rule.right[0] {
                h.insert(rule.left.clone());
            }
        }

        let mut new_flag = true;
        while new_flag {
            new_flag = false;
            for rule in &self.rules {
                if h.contains(&rule.left) {
                    continue;
                }

                let mut flag = false;
                for i in &rule.right {
                    if let Letter::NonTerminal(_) = i {
                        if !h.contains(i) {
                            flag = true;
                            break;
                        }
                    } else {
                        flag = true;
                        break;
                    }
                }
                if !flag {
                    h.insert(rule.left.clone());
                    new_flag = true;
                }
            }
        }

        h
    }

    pub fn from_rules(rules: Vec<Rule>, origin: Letter) -> Result<Grammar, String> {
        match origin {
            Letter::NonTerminal(_) => (),
            _ => {
                return Err(String::from("only non terminal can be origin"));
            }
        };

        let mut ans = Grammar {
            rules: Vec::new(),
            origin: origin,
            terminals: Vec::new(),
            non_terminals: Vec::new(),
            // empties: Vec::new()
        };

        let mut h1: HashSet<Letter> = HashSet::new();
        let mut h2: HashSet<Letter> = HashSet::new();
        for rule in rules {
            // ans.non_terminals.push(rule.left.clone());
            h1.insert(rule.left.clone());
            for i in &rule.right {
                if let &Letter::Terminal(_) = i {
                    // ans.terminals.push(i.clone());
                    h2.insert(i.clone());
                }
            }
            ans.rules.push(rule);
        }
        for i in h1.iter() {
            ans.non_terminals.push(i.clone());
        }
        for i in h2.iter() {
            
            ans.terminals.push(i.clone());
        }

        // ans.non_terminals.sort();
        // ans.terminals.sort();
        // ans.calc_empty();
        Ok(ans)
    }

    pub fn from_file(file: &str) -> Result<Grammar, String> {
        let contents = fs::read_to_string(file).unwrap();
        let mut rules: Vec<Rule> = Vec::new();
        let mut origin = Letter::Empty;

        let mut first = true;
        for line in contents.lines() {
            if line.len() == 0 {
                continue;
            }
            if first {
                first = false;
                origin = Letter::NonTerminal(String::from(line.split(' ').next().unwrap()));
            }
            let t = Rule::rules_from_str(line).unwrap();
            for r in t {
                rules.push(r);
            }
        }

        Grammar::from_rules(rules, origin)
    }

    // pub fn add_rule(&mut self, r: rule::Rule) {
    //     self.rules.push(r);
    // }

    pub fn set_origin(&mut self, origin: Letter) {
        self.origin = origin;
    }

    pub fn expand(&mut self) {
        let new_origin = format!("{}_EX", self.origin.get_str());
        let letter = Letter::NonTerminal(new_origin);

        self.non_terminals.push(letter.clone());
        self.rules.push(Rule::from_single(letter.get_str(), format!("{{{}}}", self.origin.get_str()).as_str()));
        self.origin = letter;
    }

    pub fn from_advanced_file(file: &str) -> Result<Grammar, String> {
        let contents = fs::read_to_string(file).unwrap();

        let mut item: Vec<&str> = Vec::new();
        let mut rules: Vec<Rule> = Vec::new();
        let mut origin: Letter = Letter::Empty;
        for line in contents.lines().chain(iter::once("####")) {
            if line.len() == 0 {
                continue;
            }
            if line.trim().starts_with("//") {
                continue;
            }

            if !line.starts_with(" ") {
                // an item is formed

                if item.len() <= 1 {
                    item.push(line);
                    continue;
                }

                // a rule set
                // println!("{:?}", item);
                if item[0].starts_with("ORIGIN") {
                    origin = Letter::NonTerminal(String::from(item[0].trim_start_matches("ORIGIN").trim()));
                }
                let left = Letter::NonTerminal(String::from(item[0].trim_start_matches("ORIGIN").trim()));
                for &i in item.iter().skip(1) {
                    let letters: Vec<&str> = i.trim().split(" ").collect();
                    let mut right: Vec<Letter> = Vec::new();
                    for &letter in letters.iter() {
                        if letter.len() == 0 {
                            continue;
                        }
                        if letter.starts_with("@") {
                            // a token
                            let token_type = String::from(letter.trim_start_matches("@"));
                            right.push(Letter::Terminal(token_type));
                        } else if letter == "EMPTY" {
                            right.push(Letter::Empty);
                        } else {
                            right.push(Letter::NonTerminal(String::from(letter)));
                        }
                    }

                    rules.push(Rule::new(left.clone(), right));
                }

                item.clear();
                item.push(line);
            } else {
                item.push(line);
            }
        }

        Grammar::from_rules(rules, origin)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_grammar_1() {
        let rules = vec![
            Rule::from_single("S", "Aa|a"),
            Rule::from_single("A", "ab|S"),
        ];
        let grammar = Grammar::from_rules(rules, Letter::NonTerminal(String::from("S"))).unwrap();
        assert_eq!(grammar, Grammar::new());
    }
}