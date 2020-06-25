// use std::rc::Rc;
use crate::grammar::letter::Letter;
use std::fmt;
// use std::iter;
use crate::lr_table::{LRTable, ActionTableItem};
use crate::token::{TokenStream, Token};

use serde::ser::{Serialize, Serializer, SerializeStruct};

// #[derive(Serialize, Deserialize)]
pub struct NonLeafNode {
    pub name: String,
    pub children: Vec<Box<ParseTree>>
}

impl NonLeafNode {
    pub fn new(name: &str) -> NonLeafNode {
        NonLeafNode {
            name: String::from(name),
            children: Vec::new()
        }
    }

    pub fn push_child_front(&mut self, node: ParseTree) {
        self.children.insert(0, Box::new(node));
    }
}

// #(derive(Serialize, Deserialize))
pub enum ParseTree {
    Leaf(Token),
    NonLeaf(NonLeafNode)
}

impl fmt::Display for ParseTree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut stack: Vec<(&ParseTree, usize)> = Vec::new();

        stack.push((self, 0));

        while stack.len() != 0 {
            let (p, depth) = stack.pop().unwrap();

            match p {
                ParseTree::Leaf(token) => {
                    write!(f, "{blank:<width$}{name}\n", blank="", width=depth * 2, name=token.ttype).unwrap();
                },
                ParseTree::NonLeaf(node) => {
                    write!(f, "{blank:<width$}{name}\n", blank="", width=depth * 2, name=node.name).unwrap();
                    
                    for i in node.children.iter() {
                        stack.push((i, depth + 1));
                    }
                }
            }
        }

        Ok(())
    }
}

impl Serialize for ParseTree {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        match self {
            ParseTree::Leaf(t) => {
                let temp = format!("{} {} {}:{}", t.ttype, t.text, t.row, t.col);
                serializer.serialize_str(&temp)
            },
            ParseTree::NonLeaf(n) => {
                let mut state = serializer.serialize_struct("NonLeafNode", 2)?;
                state.serialize_field("name", &n.name).unwrap();
                state.serialize_field("children", &n.children);
                state.end()
            }
        }
    }
}

impl ParseTree {

    pub fn non_leaf_with_name(name: &str) -> ParseTree {
        let node = NonLeafNode {
            name: String::from(name),
            children: Vec::new()
        };
        ParseTree::NonLeaf(node)
    }
    // pub fn lr_analysis(table: &LRTable, tokens: &TokenStream) -> ParseTree {
    //     let mut state: Vec<usize> = Vec::new();
    //     let mut signs: Vec<Letter> = Vec::new();
    //     let mut tree: Vec<ParseTree> = Vec::new();

    //     state.push(0);
    //     signs.push(Letter::EndSign);

    //     let mut i = 0;
    //     while i < tokens.stream.len() {
    //         let token = &tokens.stream[i];
    //         let current_state = match state.last() {
    //             Some(&x) => x,
    //             None => panic!("this cannot happen, in lr_analysis()"),
    //         };

    //         if current_state >= table.action.len() {
    //             panic!("this cannot happen, in lr_analysis()");
    //         }

    //         let input_letter;
    //         if token.ttype != "EOF" {
    //             input_letter = Letter::Terminal(token.ttype.clone());
    //         } else {
    //             input_letter = Letter::EndSign;
    //         }


    //         // println!("{}:\n    state: {}\n    input: {}", i, current_state, token.ttype);

    //         // if current_state == 28 {
    //         //     println!("{:?}", table.action[28]);
    //         // }
    //         match table.action[current_state].get(&input_letter).unwrap() {
    //             ActionTableItem::Err => panic!("error parsing"),
    //             ActionTableItem::Accept => break,
    //             ActionTableItem::Step(next_state) => {
    //                 signs.push(input_letter.clone());
    //                 state.push(*next_state);
    //                 tree.push(ParseTree::Leaf(token.clone()));
    //                 i += 1;
    //                 // println!("    step");
    //             }
    //             ActionTableItem::Reduce(rule) => {
    //                 // println!("    reduce using rule: {}", rule);
    //                 let mut node = NonLeafNode::new(rule.left.get_str());

    //                 let mut it1 = rule.right.len() - 1;
    //                 let mut it2 = signs.len() - 1;
    //                 while rule.right[it1] == signs[it2] {
    //                     node.push_child_front(tree.pop().unwrap());

    //                     state.pop();
    //                     signs.pop();

    //                     if it1 == 0 {
    //                         break;
    //                     }

    //                     it1 -= 1;
    //                     it2 -= 1;
    //                 }

    //                 let reduced = rule.left.clone();
    //                 let temp_state = *state.last().unwrap();
    //                 state.push(table.goto[temp_state].get(&reduced).unwrap().get_state());
    //                 signs.push(reduced);
    //                 tree.push(ParseTree::NonLeaf(node));
    //             }
    //         }
    //     }

    //     if tree.len() != 1 {
    //         panic!("this cannot happen, in lr_analysis()");
    //     }

    //     tree.pop().unwrap()
    // }
}