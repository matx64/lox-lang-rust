mod expr;
mod parser;
mod scanner;
mod token;

use scanner::Scanner;
use std::fs::read_to_string;
use crate::parser::Parser;

fn main() {
    let source: Vec<char> = read_to_string("input.lox").unwrap().chars().collect();

    let mut scanner = Scanner::new(source);

    let tokens = scanner.scan_tokens();

    let parser = Parser::new(tokens.clone());

    println!("{:?}", tokens)
}
