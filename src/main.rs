mod scanner;
use scanner::Scanner;

fn main() {
    let equation = String::from("1 + 1");
    let mut scanner = Scanner::new(equation);
    scanner.scan_tokens();
}
