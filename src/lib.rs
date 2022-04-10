mod scanner;
use scanner::Scanner;

pub fn run(equation: String) {
    let mut scanner = Scanner::new(equation);
    scanner.scan_tokens();
}