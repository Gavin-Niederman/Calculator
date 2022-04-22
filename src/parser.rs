use super::scanner::{self, Token, TokenType};

enum Expr {
    Add(Box<Expr>, Token, Box<Expr>),
    Subtract(Box<Expr>, Token, Box<Expr>),
    Multiply(Box<Expr>, Token, Box<Expr>),
    Divide(Box<Expr>, Token, Box<Expr>),
    //Power(Box<Expr>, Token, Box<Expr>),
    Negate(Token, Box<Expr>),
    Number(Token),
    //Grouping(Box<Expr>),
}

pub struct Parser {
    pub tokens: Vec<Token>,
    pub current: usize,
}

impl Parser {
    pub fn new<S>(scanner: scanner::Scanner<S>) -> Parser {
        Parser {
            tokens: scanner.tokens,
            current: 0,
        }
    }

    fn is_at_end(&self) -> bool {
        self.peek(TokenType::Eos)
    }

    fn peek(&self, tokentype: scanner::TokenType) -> bool {
        if self.tokens[self.current].token_type() == tokentype {
            return true;
        }
        false
    }

    fn term(&mut self) -> Expr {
        let mut expr = self.factor();

        if self.peek(TokenType::Add) == true {
            let operator = self.tokens[self.current];
            self.current += 1;
            let right = self.factor();
            expr = Expr::Add(Box::new(expr), operator, Box::new(right))
        } else if self.peek(TokenType::Subtract) == true {
            let operator = self.tokens[self.current];
            self.current += 1;
            let right = self.factor();
            expr = Expr::Subtract(Box::new(expr), operator, Box::new(right))
        }
        expr
    }

    fn factor(&mut self) -> Expr {
        let mut expr = self.unary();

        if self.peek(TokenType::Multiply) == true {
            let operator = self.tokens[self.current];
            self.current += 1;
            let right = self.unary();
            expr = Expr::Multiply(Box::new(expr), operator, Box::new(right))
        } else if self.peek(TokenType::Divide) == true {
            let operator = self.tokens[self.current];
            self.current += 1;
            let right = self.unary();
            expr = Expr::Divide(Box::new(expr), operator, Box::new(right))
        }
        expr
    }

    //TODO add negatives and groupings
    fn unary(&mut self) -> Expr {
        if self.peek(TokenType::Negate) {
            let operator = self.tokens[self.current];
            self.current += 1;
            let right = self.unary();
            return Expr::Negate(operator, Box::new(right));
        }
        self.primary()
    }

    //TODO implement primary
    fn primary(&self) -> Expr {
    Expr::Number(self.tokens[self.current - 1])
    }
}
