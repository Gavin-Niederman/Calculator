mod scanner;
use scanner::Scanner;

fn main() {
    let equation = String::from("1 + 1");
    run(equation);
}

// TODO: add eror handling
pub fn run(equation: String) {
    let mut scanner = Scanner::new(equation);
    scanner.scan_tokens();
}