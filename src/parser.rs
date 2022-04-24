use super::scanner::{self, Token, TokenType};

#[derive(Debug)]
pub enum Expr {
    Add(Box<Expr>, Token, Box<Expr>),
    Subtract(Box<Expr>, Token, Box<Expr>),
    Multiply(Box<Expr>, Token, Box<Expr>),
    Divide(Box<Expr>, Token, Box<Expr>),
    Power(Box<Expr>, Token, Box<Expr>),
    Negate(Token, Box<Expr>),
    Number(Token),
    Grouping(Box<Expr>)
}

pub struct Parser {
    pub tokens: Vec<Token>,
    pub current: usize,
}

impl Parser {
    pub fn parse(&mut self) -> Expr {
        self.term()
    }
    
    pub fn new<S>(scanner: scanner::Scanner<S>) -> Parser {
        Parser {
            tokens: scanner.tokens,
            current: 0,
        }
    }

    fn is_at_end(&self) -> bool {
        self.peek(TokenType::Eos)
    }

    fn peek(&self, tokentype: TokenType) -> bool {
        if self.tokens[self.current + 1].tokentype == tokentype {
            return true;
        }
        false
    }

    fn consume(&mut self, tokentype: TokenType) -> Result<bool, &'static str> {
        if self.tokens[self.current + 1].tokentype == tokentype && !self.is_at_end() {
            self.current += 1;
            return Ok(self.tokens[self.current].tokentype == tokentype);
        }
        Err("Token not found")
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        if self.peek(TokenType::Add) == true {
            self.current += 1;
            let operator = self.tokens[self.current];
            self.current += 1;
            let right = self.factor();
            expr = Expr::Add(Box::new(expr), operator, Box::new(right))
        } else if self.peek(TokenType::Subtract) == true {
            self.current += 1;
            let operator = self.tokens[self.current];
            self.current += 1;
            let right = self.factor();
            expr = Expr::Subtract(Box::new(expr), operator, Box::new(right))
        }
        expr
    }
    
    fn factor(&mut self) -> Expr {
        let mut expr = self.power();

        if self.peek(TokenType::Multiply) == true {
            self.current += 1;
            let operator = self.tokens[self.current];
            self.current += 1;
            let right = self.unary();
            expr = Expr::Multiply(Box::new(expr), operator, Box::new(right))
        } else if self.peek(TokenType::Divide) == true {
            self.current += 1;
            let operator = self.tokens[self.current];
            self.current += 1;
            let right = self.unary();
            expr = Expr::Divide(Box::new(expr), operator, Box::new(right))
        }
        expr
    }

    fn power(&mut self) -> Expr {
        let mut expr = self.unary();

        if self.peek(TokenType::Power) == true {
            self.current += 1;
            let operator = self.tokens[self.current];
            self.current += 1;
            let right = self.factor();
            expr = Expr::Power(Box::new(expr), operator, Box::new(right))
        }
        expr
    }
    
    fn unary(&mut self) -> Expr {
        if self.peek(TokenType::Negate) {
            self.current += 1;
            let operator = self.tokens[self.current];
            self.current += 1;
            let right = self.unary();
            return Expr::Negate(operator, Box::new(right));
        }
        self.primary().unwrap()
    }

    fn primary(&mut self) -> Result<Expr, &'static str> {
        if let TokenType::Number(_) = self.tokens[self.current].tokentype {
            return Ok(Expr::Number(self.tokens[self.current]));
        }

        if self.tokens[self.current].tokentype == TokenType::LeftParen {
            self.current += 1;
            let expr = self.term();
            if self.consume(TokenType::RightParen).unwrap() {
                return Ok(Expr::Grouping(Box::new(expr)));
            }
        }
        Err("Mangled token")
    }
}
