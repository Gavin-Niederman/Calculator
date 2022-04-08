use std::str::Chars;

enum TokenType {
    LeftParen,
    RightParen,
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    Number(i128),
}

struct Token {
    tokentype: TokenType,
    location: i32,
}

struct Scanner {
    equation: String,
    tokens: Vec<Token>,
    start: i32,
    current: i32,
    error: bool,
}

fn main() {
    let equation = String::from("1 + 1");
    let mut scanner = Scanner::new(equation);
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
                '^' => self.add_token(TokenType::Power),
                current @ '0'..='9' => {
                    self.number(current, &mut chars)
                }
                other => {
                    if other != ' ' {
                        eprintln!(
                            "Unexpected character {} at charachter {}",
                            other, self.current
                        );
                        self.error = true;
                    }
                }
            }
        }
    }

    fn add_token(&mut self, tokentype: TokenType) {
        self.tokens.push(Token {
            tokentype,
            location: self.current,
        })
    }

    fn new(equation: String) -> Scanner {
        Scanner { equation, tokens: Vec::new(), start: 0, current: 0, error: false }
    }

    fn number(&mut self, current: char, chars: &mut Chars) {
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
