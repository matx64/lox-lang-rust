mod scanner;
mod token;

use scanner::Scanner;
use std::fs::read_to_string;

fn main() {
    let source: Vec<char> = read_to_string("input.lox").unwrap().chars().collect();

    let mut scanner = Scanner::new(source);

    println!("{:?}", scanner.scan_tokens())
}
