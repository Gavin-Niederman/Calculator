mod scanner;
mod parser;
use scanner::Scanner;
use parser::Parser;

//TODO add error handling
pub fn run(equation: &str) {
    let mut scanner = Scanner::new(equation.chars());
    scanner.scan_tokens();
    let mut parser = Parser::new(scanner);
    let expr = parser.parse();
    println!("{:?}", expr);
}
