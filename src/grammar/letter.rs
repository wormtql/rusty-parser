use std::fmt;
// use std::hash::{Hash, Hasher};
// use serde::{Serialize, Deserialize};
use serde::ser::{Serialize, Serializer};
use serde::de::{self, Visitor, Deserializer, Deserialize};

struct MyVisitor;
impl <'de> Visitor<'de> for MyVisitor {
    type Value = Letter;

    fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "worm string")
    }

    fn visit_str<E>(self, value: &str) -> Result<Self::Value, E>
        where E: de::Error
    {
        let temp: Vec<&str> = value.split("wormtql").collect();
        if temp[0] == "NonTerminal" {
            return Ok(Letter::NonTerminal(String::from(temp[1])));
        } else if temp[0] == "Terminal" {
            return Ok(Letter::Terminal(String::from(temp[1])));
        } else if temp[0] == "Empty" {
            return Ok(Letter::Empty);
        } else if temp[0] == "EndSign" {
            return Ok(Letter::EndSign);
        }

        Err(E::custom("err"))
    }
}


#[derive(Eq, Hash, PartialEq, Clone, Ord, PartialOrd)]
pub enum Letter {
    NonTerminal(String),
    Terminal(String),
    Empty,
    EndSign
}

impl Serialize for Letter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer
    {
        let mut temp = match self {
            Letter::NonTerminal(x) => String::from("NonTerminalwormtql") + x,
            Letter::Terminal(x) => String::from("Terminalwormtql") + x,
            Letter::Empty => String::from("Emptywormtql"),
            Letter::EndSign => String::from("EndSignwormtql")
        };

        serializer.serialize_str(&temp)
    }
}

impl<'de> Deserialize<'de> for Letter {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        // deserializer.deserialize_enum("Letter", &["NonTerminal", "Terminal", "Empty", "EndSign"], MyVisitor)
        deserializer.deserialize_str(MyVisitor)
    }
}


impl fmt::Display for Letter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Letter::NonTerminal(x) => write!(f, "{{{}}}", x),
            Letter::Terminal(x) => write!(f, "{}", x),
            Letter::Empty => write!(f, "."),
            Letter::EndSign => write!(f, "#"),
        }
    }
}

impl fmt::Debug for Letter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl Letter {
    pub fn get_str(&self) -> &str {
        match self {
            Letter::NonTerminal(ref x) => x,
            Letter::Terminal(ref x) => x,
            Letter::Empty => ".",
            Letter::EndSign => "#"
        }
    }

    pub fn is_terminal(&self) -> bool {
        if let Letter::Terminal(_) = self {
            return true;
        }
        false
    }

    pub fn is_empty(&self) -> bool {
        if let Letter::Empty = self {
            return true;
        }
        false
    }

    pub fn is_non_terminal(&self) -> bool {
        if let Letter::NonTerminal(_) = self {
            return true;
        }
        false
    }

    pub fn is_end_sign(&self) -> bool {
        if let Letter::EndSign = self {
            return true;
        }
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_letter_1() {
        let x = Letter::NonTerminal(String::from("asd"));
        assert_eq!(x.to_string(), String::from("{asd}"));
    }

    #[test]
    fn test_letter_2() {
        let x = Letter::Terminal(String::from("a123"));
        assert_eq!(x.to_string(), String::from("a123"));
    }

    #[test]
    fn test_letter_3() {
        let x = Letter::Empty;
        assert_eq!(x.to_string(), String::from("."));
    }
}