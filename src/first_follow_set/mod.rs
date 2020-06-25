// use crate::grammar::rule::Rule;
use crate::grammar::letter::Letter;
use crate::grammar::sentence::Sentence;
use crate::grammar::Grammar;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::fmt;
use prettytable::{Table, Cell, Row};

mod graph;

pub struct FirstFollowSet {
    pub first_set: HashMap<Letter, Vec<Letter>>,
    pub follow_set: HashMap<Letter, Vec<Letter>>
}

impl fmt::Display for FirstFollowSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut table = Table::new();

        let mut letters: Vec<_> = self.first_set.keys().cloned().collect();
        letters.sort();

        let mut header: Vec<Cell> = Vec::new();
        header.push(Cell::new(""));
        header.push(Cell::new("FIRST SET"));
        header.push(Cell::new("FOLLOW SET"));
        table.add_row(Row::new(header));

        for letter in letters.iter() {
            let mut row: Vec<Cell> = Vec::new();
            row.push(Cell::new(letter.get_str()));

            let first = self.first_set.get(letter).unwrap();
            let follow = self.follow_set.get(letter).unwrap();

            let mut temp = String::new();
            if first.len() > 0 {
                temp.push_str(first[0].get_str());
            }
            for i in first.iter().skip(1) {
                temp.push_str("\n");
                temp.push_str(i.get_str());
            }
            row.push(Cell::new(&temp));

            let mut temp = String::new();
            if follow.len() > 0 {
                temp.push_str(follow[0].get_str());
            }
            for i in follow.iter().skip(1) {
                temp.push_str("\n");
                temp.push_str(i.get_str());
            }
            row.push(Cell::new(&temp));

            table.add_row(Row::new(row));
        }
        
        write!(f, "{}", table).unwrap();

        Ok(())
    }
}

impl FirstFollowSet {
    pub fn from_grammar(g: &Grammar) -> FirstFollowSet {
        // terminal count
        let terminal_count = g.terminals.len() as i32;

        // non terminal count
        let non_terminal_count = g.non_terminals.len() as i32;

        // signs that can infer empty
        let empties = g.calc_empties();

        /*
            node 0: #
                 1 ~ terminal_count: terminals
                 terminal_count + 1 ~ terminal_count + non_terminal_count: FIRST(NT)
                 terminal_count + non_terminal_count + 1 ~ terminal_count + 2 * non_terminal_count: FOLLOW(NT) 
        */
        let mut graph = graph::Graph::new(2 * non_terminal_count + terminal_count + 1);


        // terminal sign to node id
        let mut tn: HashMap<Letter, i32> = HashMap::new();
        for i in 0..g.terminals.len() {
            tn.insert(g.terminals[i].clone(), i as i32 + 1);
        }
        tn.insert(Letter::EndSign, 0);


        // non terminal sign to node id
        let mut firstn: HashMap<Letter, i32> = HashMap::new();
        let mut follown: HashMap<Letter, i32> = HashMap::new();
        for i in 0..g.non_terminals.len() {
            firstn.insert(g.non_terminals[i].clone(), terminal_count + 1 + i as i32);
            follown.insert(g.non_terminals[i].clone(), terminal_count + non_terminal_count + 1 + i as i32);
        }


        // add edges:
        for rule in &g.rules {
            if rule.right.len() == 1 && rule.right[0].is_empty() {
                continue;
            }


            // FIRST -> FIRST
            let mut i = 0;
            while i < rule.right.len() && rule.right[i].is_non_terminal() {
                if rule.right[i] != rule.left {
                    graph.add_edge(*firstn.get(&rule.left).unwrap(), *firstn.get(&rule.right[i]).unwrap());
                }
                if empties.contains(&rule.right[i]) {
                    i += 1;
                } else {
                    break;
                }
            }
            if i < rule.right.len() && rule.right[i].is_terminal() {
                graph.add_edge(*firstn.get(&rule.left).unwrap(), *tn.get(&rule.right[i]).unwrap());
            }


            // FOLLOW -> FOLLOW
            i = rule.right.len() - 1;
            while rule.right[i].is_non_terminal() {
                if rule.left != rule.right[i] {
                    graph.add_edge(*follown.get(&rule.right[i]).unwrap(), *follown.get(&rule.left).unwrap());
                }
                if empties.contains(&rule.right[i]) {
                    if i == 0 {
                        break
                    } else {
                        i -= 1;
                    }
                } else {
                    break;
                }
            }


            // FOLLOW -> FIRST
            for i in 0..rule.right.len() {
                if rule.right[i].is_non_terminal() {
                    for j in i + 1..rule.right.len() {
                        if rule.right[j].is_terminal() {
                            graph.add_edge(*follown.get(&rule.right[i]).unwrap(), *tn.get(&rule.right[j]).unwrap());
                            break;
                        }
                        if rule.right[j].is_non_terminal() {
                            graph.add_edge(*follown.get(&rule.right[i]).unwrap(), *firstn.get(&rule.right[j]).unwrap());
                            if !empties.contains(&rule.right[j]) {
                                break;
                            }
                        }
                    }
                }
            }
        }
        // add FOLLOW(Origin) -> #
        graph.add_edge(*follown.get(&g.origin).unwrap(), 0);

        // println!("{}", graph);


        // first set
        let mut first_set: HashMap<Letter, Vec<Letter>> = HashMap::new();
        // follow set
        let mut follow_set: HashMap<Letter, Vec<Letter>> = HashMap::new();


        // calc first and follow set
        // calc first set
        for nt in &g.non_terminals {
            let mut vis: HashSet<i32> = HashSet::new();
            let mut q: VecDeque<i32> = VecDeque::new();
            let mut temp: Vec<Letter> = Vec::new();
            
            vis.insert(*firstn.get(nt).unwrap());
            q.push_back(*firstn.get(nt).unwrap());

            while !q.is_empty() {
                let p = q.pop_front().unwrap();

                let mut e = graph.head[p as usize];
                while e != -1 {
                    let to = graph.edges[e as usize].to;
                    if !vis.contains(&to) {
                        vis.insert(to);
                        q.push_back(to);
                        if to >= 0 && to <= terminal_count {
                            // is terminal
                            temp.push(if to == 0 { Letter::EndSign } else { g.terminals[to as usize - 1].clone() });
                        }
                    }
                    
                    e = graph.edges[e as usize].next;
                }
            }

            if empties.contains(nt) {
                temp.push(Letter::Empty);
            }

            first_set.insert(nt.clone(), temp);
        }


        // calc follow set
        for nt in &g.non_terminals {
            let mut vis: HashSet<i32> = HashSet::new();
            let mut q: VecDeque<i32> = VecDeque::new();
            let mut temp: Vec<Letter> = Vec::new();
            
            vis.insert(*follown.get(nt).unwrap());
            q.push_back(*follown.get(nt).unwrap());

            while !q.is_empty() {
                let p = q.pop_front().unwrap();

                let mut e = graph.head[p as usize];
                while e != -1 {
                    let to = graph.edges[e as usize].to;
                    if !vis.contains(&to) {
                        vis.insert(to);
                        q.push_back(to);
                        if to >= 0 && to <= terminal_count {
                            // is terminal
                            temp.push(if to == 0 { Letter::EndSign } else { g.terminals[to as usize - 1].clone() });
                        }
                    }
                    
                    e = graph.edges[e as usize].next;
                }
            }
            follow_set.insert(nt.clone(), temp);
        }


        FirstFollowSet {
            first_set,
            follow_set
        }
    }

    pub fn first_set_of_sentence(&self, sentence: &Sentence) -> Vec<Letter> {
        let sentence = &sentence.sentence;

        if sentence.len() == 1 && sentence[0].is_empty() {
            return vec![Letter::Empty];
        }

        let mut i = 0;
        let mut ans: HashSet<Letter> = HashSet::new();
        while i < sentence.len() {
            if sentence[i].is_non_terminal() {
                for v in self.first_set.get(&sentence[i]).unwrap().iter().filter(|x| !x.is_empty()) {
                    ans.insert(v.clone());
                }
                if !contains(&self.first_set.get(&sentence[i]).unwrap(), &Letter::Empty) {
                    break;
                }
            } else if sentence[i].is_terminal() || sentence[i].is_end_sign() {
                ans.insert(sentence[i].clone());
                break;
            } else {
                panic!("empty sign should not appear");
            }
            i += 1;
        }
        if i == sentence.len() {
            ans.insert(Letter::Empty);
        }

        ans.iter().cloned().collect()
    }
}

fn contains(set: &Vec<Letter>, letter: &Letter) -> bool {
    for i in set.iter() {
        if i == letter {
            return true;
        }
    }
    false
}