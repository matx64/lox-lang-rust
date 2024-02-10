use std::collections::HashMap;

use crate::token::{Token, TokenKind};

pub struct Scanner {
    source: Vec<char>,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: i64,
    keywords: HashMap<&'static str, TokenKind>,
}

impl Scanner {
    pub fn new(source: Vec<char>) -> Self {
        Scanner {
            source,
            tokens: vec![],
            start: 0,
            current: 0,
            line: 1,
            keywords: Scanner::load_keywords(),
        }
    }

    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token()
        }

        self.tokens
            .push(Token::new(TokenKind::Eof, "".to_string(), self.line));

        &self.tokens
    }

    fn scan_token(&mut self) {
        let c = self.advance();
        match c {
            ' ' | '\r' | '\t' => {} // Ignore whitespace
            '\n' => self.line += 1,
            '(' => self.add_token(TokenKind::LeftParen),
            ')' => self.add_token(TokenKind::RightParen),
            '{' => self.add_token(TokenKind::LeftBrace),
            '}' => self.add_token(TokenKind::RightBrace),
            ',' => self.add_token(TokenKind::Comma),
            '.' => self.add_token(TokenKind::Dot),
            '-' => self.add_token(TokenKind::Minus),
            '+' => self.add_token(TokenKind::Plus),
            ';' => self.add_token(TokenKind::Semicolon),
            '*' => self.add_token(TokenKind::Star),
            '!' => {
                if self.match_next('=') {
                    self.add_token(TokenKind::BangEqual)
                } else {
                    self.add_token(TokenKind::Bang)
                }
            }
            '=' => {
                if self.match_next('=') {
                    self.add_token(TokenKind::EqualEqual)
                } else {
                    self.add_token(TokenKind::Equal)
                }
            }
            '<' => {
                if self.match_next('=') {
                    self.add_token(TokenKind::LessEqual)
                } else {
                    self.add_token(TokenKind::Less)
                }
            }
            '>' => {
                if self.match_next('=') {
                    self.add_token(TokenKind::GreaterEqual)
                } else {
                    self.add_token(TokenKind::Greater)
                }
            }
            '/' => {
                if self.match_next('/') {
                    // A comment goes until the end of the line
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenKind::Slash);
                }
            }
            '"' => self.string(),
            _ => {
                if self.is_digit(c) {
                    self.number();
                } else if self.is_alpha(c) {
                    self.identifier();
                } else {
                    self.error("Unexpected character")
                }
            }
        }
    }

    fn identifier(&mut self) {
        while self.is_alpha_numeric(self.peek()) {
            self.advance();
        }

        let lexeme = String::from_iter(&self.source[self.start..self.current]);

        match self.keywords.get(&lexeme.as_str()) {
            Some(&kind) => self.add_token_with_lex(kind, lexeme),
            None => self.add_token_with_lex(TokenKind::Identifier, lexeme),
        }
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.error("Unterminated string.");
        }

        // The closing "
        self.advance();

        self.add_token(TokenKind::Str);
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        // Look for a fractional part
        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            // Consume the "."
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        self.add_token(TokenKind::Number);
    }

    fn add_token(&mut self, kind: TokenKind) {
        let lexeme: String = String::from_iter(&self.source[self.start..self.current]);

        self.tokens.push(Token::new(kind, lexeme, self.line));
    }

    fn add_token_with_lex(&mut self, kind: TokenKind, lexeme: String) {
        self.tokens.push(Token::new(kind, lexeme, self.line));
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

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source[self.current]
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }
        self.source[self.current + 1]
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn is_alpha(&self, c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }

    fn is_digit(&self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
        self.is_alpha(c) || self.is_digit(c)
    }

    pub fn load_keywords() -> HashMap<&'static str, TokenKind> {
        let mut keywords = HashMap::new();

        keywords.insert("and", TokenKind::And);
        keywords.insert("class", TokenKind::Class);
        keywords.insert("else", TokenKind::Else);
        keywords.insert("false", TokenKind::False);
        keywords.insert("for", TokenKind::For);
        keywords.insert("fun", TokenKind::Fun);
        keywords.insert("if", TokenKind::If);
        keywords.insert("nil", TokenKind::Nil);
        keywords.insert("or", TokenKind::Or);
        keywords.insert("print", TokenKind::Print);
        keywords.insert("return", TokenKind::Return);
        keywords.insert("super", TokenKind::Super);
        keywords.insert("this", TokenKind::This);
        keywords.insert("true", TokenKind::True);
        keywords.insert("var", TokenKind::Var);
        keywords.insert("while", TokenKind::While);

        keywords
    }

    fn error(&self, msg: &'static str) {
        panic!("{}", format!("Error in Line {}: {}", self.line, msg));
    }
}
