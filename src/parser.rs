use super::scanner::{self, Token};

enum Expr {
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
    pub tokens: Vec<Token>
}

impl Parser {
    pub fn new<S>(scanner: scanner::Scanner<S>) -> Parser {
        Parser { tokens: scanner.tokens }
    }
}

