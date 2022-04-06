use std::{str::Chars, panic::Location};

enum TokenType {
    LeftParen,
    RightParen,
    Add,
    Subtract,
    Multiply,
    Divide,
    Number(i128)
} 

struct Token {
    tokentype: TokenType,
    location: i32,
}

struct Scanner {
    equation: String,
    tokens: Vec<Token>,
    start: i32,
    current: i32
}

fn main() {
    println!("Hello, world!");
}

impl Scanner {
    fn scan_tokens(&mut self) {
        let clone = self.equation.clone();
        let mut chars = clone.chars();

        while chars.clone().next() != None {
            self.current += 1;
            self.start = self.current;
            match chars.next().unwrap() {
                '(' => self.add_token(TokenType::LeftParen),
                ')' => self.add_token(TokenType::RightParen),
                '+' => self.add_token(TokenType::Add),
                '-' => self.add_token(TokenType::Subtract),
                '*' => self.add_token(TokenType::Multiply),
                '/' => self.add_token(TokenType::Divide),
                '0'..='9' => {}//whyyy
                other => if other != ' ' {
                    eprintln!("Unexpected character {} at charachter {}", other, self.current);
                }
            }
        }
    }

    fn add_token(&mut self, tokentype: TokenType) {
        self.tokens.push(Token{tokentype, location: self.current})
    }
}