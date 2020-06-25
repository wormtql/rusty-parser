mod graph;

use std::fmt;
use std::collections::HashSet;
use std::hash::Hash;

pub use graph::{Graph, Edge};

pub fn vec_to_set_string<T: fmt::Display>(v: &Vec<T>) -> String {
    if v.len() == 0 {
        return String::from("{}");
    }

    let mut ans = String::new();
    ans.push('{');
    ans.push_str(v[0].to_string().as_str());
    for i in v.iter().skip(1) {
        ans.push_str(", ");
        ans.push_str(i.to_string().as_str());
    }
    ans.push('}');

    ans
}

pub fn vec_to_string<T: fmt::Display>(v: &[T], split: &str) -> String {
    if v.len() == 0 {
        return String::new();
    }

    let mut ans = String::new();
    ans.push_str(v[0].to_string().as_str());
    for i in v.iter().skip(1) {
        ans.push_str(split);
        ans.push_str(i.to_string().as_str());
    }
    
    ans
}

pub fn hash_set_to_sorted_str<T: fmt::Display + Ord + Hash>(v: &HashSet<T>, split: &str) -> String {
    if v.len() == 0 {
        return String::from("{}");
    }
    let mut temp: Vec<&T> = v.iter().collect();
    temp.sort();

    let mut ans = String::from("{");
    ans.push_str(temp[0].to_string().as_str());
    for i in temp.iter().skip(1) {
        ans.push_str(split);
        ans.push_str(i.to_string().as_str());
    }
    ans.push_str("}");

    ans
}