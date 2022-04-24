#[derive(PartialEq, Clone, Copy, Debug)]
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
    Eos
}

#[derive(Clone, Copy, Debug)]
pub struct Token {
    pub tokentype: TokenType,
    pub location: usize,
}

pub struct Scanner<S> {
    pub source: S,
    pub tokens: Vec<Token>,
    pub start: usize,
    pub current: usize,
    pub error: bool,
}


impl<S> Scanner<S> 
where 
    S: Iterator<Item = char> + Clone
{
    //TODO add error handling and negatives
    //Iterates over source and converts it into tokens
    pub fn scan_tokens(&mut self) {
        let mut last = TokenType::Eos;
        while self.source.clone().next() != None {
            match self.source.next().unwrap() {
                '(' => self.add_token(TokenType::LeftParen),
                ')' => self.add_token(TokenType::RightParen),
                '+' => self.add_token(TokenType::Add),
                '-' =>  { 
                    if let TokenType::Number(_) = last { 
                        self.add_token(TokenType::Subtract); 
                    } else {                        
                        let current = self.source.next().unwrap();
                        self.number_negative(current);
                    }
                },
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
                            "Unexpected character {} at character {}",
                            other, self.current
                        );
                        self.error = true;
                    } else {
                        self.current -= 1;
                    }
                }
            }
            last = self.tokens[self.current].tokentype;
            //Store location data for forming tokens
            self.current += 1;
            self.start = self.current;
        }
        self.add_token(TokenType::Eos);
    }

    //Adds a token to the vector of tokens in scanner
    fn add_token(&mut self, tokentype: TokenType) {
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
    fn number(&mut self, current: char) {
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

    fn number_negative(&mut self, current: char) {
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
            -number
                .parse::<i128>()
                .expect("Internal error! Failed to parse number"),
        ))
    }
}