use super::scanner::{self, Token};

enum Expr {
    Add(Box<Expr>, Token, Box<Expr>),
    Subtract(Box<Expr>, Token, Box<Expr>),
    Multiply(Box<Expr>, Token, Box<Expr>),
    Divide(Box<Expr>, Token, Box<Expr>),
    Power(Box<Expr>, Token, Box<Expr>),
    Negate(Token, Box<Expr>),
    Number(Token)
}