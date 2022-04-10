use std::str::Chars;

pub enum TokenType {
    LeftParen,
    RightParen,
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    Number(i128),
}

pub struct Token {
    pub tokentype: TokenType,
    pub location: i32,
}

pub struct Scanner {
    pub equation: String,
    pub tokens: Vec<Token>,
    pub start: i32,
    pub current: i32,
    pub error: bool,
}


impl Scanner {
    pub fn scan_tokens(&mut self) {
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

    pub fn add_token(&mut self, tokentype: TokenType) {
        self.tokens.push(Token {
            tokentype,
            location: self.current,
        })
    }

    pub fn new(equation: String) -> Scanner {
        Scanner { equation, tokens: Vec::new(), start: 0, current: 0, error: false }
    }

    pub fn number(&mut self, current: char, chars: &mut Chars) {
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
