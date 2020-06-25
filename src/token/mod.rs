mod token_stream;

pub use token_stream::TokenStream;

#[derive(Clone)]
pub struct Token {
    pub ttype: String,
    pub text: String,
    pub row: usize,
    pub col: usize
}

impl Token {
    pub fn new(ttype: String, text: String, row: usize, col: usize) -> Token {
        Token {
            ttype, text, row, col
        }
    }
}