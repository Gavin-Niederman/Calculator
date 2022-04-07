use std::str::Chars;

#[derive(Debug)]
enum TokenType {
    LeftParen,
    RightParen,
    Add,
    Subtract,
    Multiply,
    Divide,
    Number(i128),
}

#[derive(Debug)]
struct Token {
    tokentype: TokenType,
    location: i32,
}

struct Scanner {
    equation: String,
    tokens: Vec<Token>,
    start: i32,
    current: i32,
}

fn main() {
    let mut scanner = Scanner::new("1 + 1".to_string());
    scanner.scan_tokens();
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
                current @ '0'..='9' => {
                    self.form_numbers(current, &mut chars)
                }
                other => {
                    if other != ' ' {
                        eprintln!(
                            "Unexpected character {} at charachter {}",
                            other, self.current
                        );
                    }
                }
            }
        }
        println!("{:?}", self.tokens);
    }

    fn add_token(&mut self, tokentype: TokenType) {
        self.tokens.push(Token {
            tokentype,
            location: self.current,
        })
    }

    fn new(equation: String) -> Scanner {
        Scanner { equation, tokens: Vec::new(), start: 0, current: 0 }
    }

    fn form_numbers(&mut self, current: char, chars: &mut Chars) {
        let mut number = String::from(current.to_string());
        while chars.clone().next() != None {
            if let '0'..='9' = chars.clone().next().unwrap() {
                self.current += 1;
                number += &chars.next().unwrap().to_string();  
            } else {
                break;
            }
        }
        self.add_token(TokenType::Number(
            number
                .parse()
                .expect("Internal error! Failed to parse number"),
        ))
    }
}
