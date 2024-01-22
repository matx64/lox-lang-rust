mod scanner;
mod token;

use scanner::Scanner;
use std::fs::read_to_string;

fn main() {
    let source: Vec<char> = read_to_string("input.lox").unwrap().chars().collect();

    let scanner = Scanner::new(source);
}
