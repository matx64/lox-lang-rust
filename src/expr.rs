use crate::token::{Token};

pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Grouping,
    Literal,
    Unary,
}
