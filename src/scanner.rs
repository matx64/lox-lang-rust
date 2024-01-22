use crate::token::{Token, TokenKind};

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: i64,
}

impl Scanner {
    pub fn new(source: Vec<char>) -> Self {
        Scanner {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()
        }
        self.tokens
            .push(Token::new(TokenKind::EOF, None, self.line))
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenKind::LEFT_PAREN),
            ')' => self.add_token(TokenKind::RIGHT_PAREN),
            '{' => self.add_token(TokenKind::LEFT_BRACE),
            '}' => self.add_token(TokenKind::RIGHT_BRACE),
            ',' => self.add_token(TokenKind::COMMA),
            '.' => self.add_token(TokenKind::DOT),
            '-' => self.add_token(TokenKind::MINUS),
            '+' => self.add_token(TokenKind::PLUS),
            ';' => self.add_token(TokenKind::SEMICOLON),
            '*' => self.add_token(TokenKind::STAR),
            '!' => {
                if self.match_next('=') {
                    self.add_token(TokenKind::BANG_EQUAL)
                } else {
                    self.add_token(TokenKind::BANG)
                }
            }
            '=' => {
                if self.match_next('=') {
                    self.add_token(TokenKind::EQUAL_EQUAL)
                } else {
                    self.add_token(TokenKind::EQUAL)
                }
            }
            '<' => {
                if self.match_next('=') {
                    self.add_token(TokenKind::LESS_EQUAL)
                } else {
                    self.add_token(TokenKind::LESS)
                }
            }
            '>' => {
                if self.match_next('=') {
                    self.add_token(TokenKind::GREATER_EQUAL)
                } else {
                    self.add_token(TokenKind::GREATER)
                }
            }
            _ => {}
        }
    }

    fn advance(&mut self) -> char {
        let c = self.source[self.current];
        self.current += 1;
        c
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() || self.source[self.current] != expected {
            return false;
        }

        self.current += 1;
        true
    }

    fn add_token(&mut self, kind: TokenKind) {
        let lex: String = String::from_iter(&self.source[self.start..self.current]);

        self.tokens.push(Token::new(kind, Some(lex), self.line));
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
