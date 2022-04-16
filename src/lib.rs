mod scanner;
mod parser;
use scanner::Scanner;

pub fn run(equation: &str) {
    let mut scanner = Scanner::new(equation.chars());
    scanner.scan_tokens();
}
