use super::scanner::{self, Token, TokenType};

pub enum Expr {
    Add(Box<Expr>, Box<Expr>),
    Subtract(Box<Expr>, Box<Expr>),
    Multiply(Box<Expr>, Box<Expr>),
    Divide(Box<Expr>, Box<Expr>),
    Power(Box<Expr>, Box<Expr>),
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
            self.current += 2;
            let right = self.factor();
            expr = Expr::Add(Box::new(expr), Box::new(right))
        } else if self.peek(TokenType::Subtract) == true {
            self.current += 2;
            let right = self.factor();
            expr = Expr::Subtract(Box::new(expr), Box::new(right))
        }
        expr
    }
    
    fn factor(&mut self) -> Expr {
        let mut expr = self.power();

        if self.peek(TokenType::Multiply) == true {
            self.current += 2;
            let right = self.power();
            expr = Expr::Multiply(Box::new(expr), Box::new(right))
        } else if self.peek(TokenType::Divide) == true {
            self.current += 1;
            self.current += 1;
            let right = self.power();
            expr = Expr::Divide(Box::new(expr), Box::new(right))
        }
        expr
    }

    fn power(&mut self) -> Expr {
        let mut expr = self.primary().unwrap();

        if self.peek(TokenType::Power) == true {
            self.current += 2;
            let right = self.factor();
            expr = Expr::Power(Box::new(expr), Box::new(right))
        }
        expr
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
