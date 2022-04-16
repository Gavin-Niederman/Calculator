pub enum TokenType {
    LeftParen,
    RightParen,
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    Negate,
    Number(i128),
}

pub struct Token {
    pub tokentype: TokenType,
    pub location: i32,
}

pub struct Scanner<S> {
    pub source: S,
    pub tokens: Vec<Token>,
    pub start: i32,
    pub current: i32,
    pub error: bool,
}


impl<S> Scanner<S> 
where 
    S: Iterator<Item = char> + Clone
{
    //TODO add error handling
    //Iterates over source and converts it into tokens
    pub fn scan_tokens(&mut self) {
        while self.source.clone().next() != None {
            //Store location data for forming tokens
            self.current += 1;
            self.start = self.current;
            match self.source.next().unwrap() {
                '(' => self.add_token(TokenType::LeftParen),
                ')' => self.add_token(TokenType::RightParen),
                '+' => self.add_token(TokenType::Add),
                '-' => self.add_token(TokenType::Subtract),
                '*' => self.add_token(TokenType::Multiply),
                '/' => self.add_token(TokenType::Divide),
                '!' => self.add_token(TokenType::Negate),
                '^' => self.add_token(TokenType::Power),
                current @ '0'..='9' => {
                    self.number(current)
                }
                //Ignores whitespace
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

    //Adds a token to the vector of tokens in scanner
    pub fn add_token(&mut self, tokentype: TokenType) {
        self.tokens.push(Token {
            tokentype,
            location: self.current,
        })
    }

    //Returns a scanner
    pub fn new(source: S) -> Scanner<S> {
        Scanner { source, tokens: Vec::new(), start: 0, current: 0, error: false }
    }

    //Iterates over source until the next character is not a number
    //Adds numbers to a string which is parsed when a character that is not part of a number is found
    pub fn number(&mut self, current: char) {
        let mut number = String::from(current.to_string());
        //Check next character and deal with it accordingly
        while self.source.clone().next() != None {
            if let '0'..='9' = self.source.clone().next().unwrap() {
                self.current += 1;
                number += &self.source.next().unwrap().to_string();  
            } else {
                break;
            }
        }
        //Add finished number token
        self.add_token(TokenType::Number(
            number
                .parse()
                .expect("Internal error! Failed to parse number"),
        ))
    }
}
