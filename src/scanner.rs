use crate::token::Token;

pub struct Scanner {
    source: &'static str,
    tokens: Vec<Token>,
}

impl Scanner {
    pub fn new(source: &'static str) -> Self {
        Scanner {
            source,
            tokens: vec![],
        }
    }
}
