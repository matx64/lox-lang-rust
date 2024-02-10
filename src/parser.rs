use crate::{
    expr::Expr,
    token::{Token, TokenKind},
};

pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, current: 0 }
    }

    fn expression(&mut self) -> Expr {
        self.equality()
    }

    fn equality(&mut self) -> Expr {
        let mut expr = self.comparison();

        while self.match_next(TokenKind::BangEqual) || self.match_next(TokenKind::EqualEqual) {
            let operator = self.previous();
            let right = self.comparison();
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        expr
    }

    fn comparison(&mut self) -> Expr {
        todo!()
    }

    fn match_next(&mut self, kind: TokenKind) -> bool {
        if self.check(kind) {
            self.advance();
            return true;
        }
        false
    }

    fn check(&self, kind: TokenKind) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().kind == kind
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    fn peek(&self) -> Token {
        self.tokens[self.current]
    }

    fn previous(&self) -> Token {
        self.tokens[self.current - 1]
    }

    fn is_at_end(&self) -> bool {
        self.peek().kind == TokenKind::Eof
    }
}
