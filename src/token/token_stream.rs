// use crate::grammar::letter::Letter;
use std::fs;
use super::Token;
use std::fmt;

pub struct TokenStream {
    pub stream: Vec<Token>
}

impl TokenStream {
    pub fn from_file(file: &str) -> TokenStream {
        let contents = fs::read_to_string(file).unwrap();
        let mut tokens: Vec<Token> = Vec::new();

        for line in contents.lines() {
            let temp: Vec<&str> = line.split(" ").collect();
            let row = temp[3].parse::<usize>().unwrap();
            let col = temp[4].parse::<usize>().unwrap();
            let text = String::from(temp[1]);
            let ttype = String::from(temp[2]);

            tokens.push(Token::new(ttype, text, row, col));
        }

        tokens.push(Token::new(String::from("EOF"), String::new(), 0, 0));

        TokenStream {
            stream: tokens
        }
    }
}

impl fmt::Display for TokenStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for token in self.stream.iter() {
            write!(f, "type: {} text: {}\n", token.ttype, token.text).unwrap();
        }

        Ok(())
    }
}