mod scanner;
mod parser;
mod interpreter;
use scanner::Scanner;
use parser::Parser;
use interpreter::Interpreter;
use std::io;

//TODO add error handling
pub fn run() {
    prompt()
}

fn prompt() {
    loop {
        let mut equation = String::new();
        io::stdin().read_line(&mut equation).expect("Failed to read line");
        let mut scanner = Scanner::new(equation.trim().chars());
        scanner.scan_tokens();
        if !scanner.error {
            let mut parser = Parser::new(scanner);
            let interpreter = Interpreter::new(parser.parse());
            println!("{}", Interpreter::evaluate(Box::new(interpreter.expr)))
        }
    }
}