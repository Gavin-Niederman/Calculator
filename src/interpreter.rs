use crate::parser::Expr;
use crate::scanner::TokenType;

pub struct Interpreter {
    pub expr: Expr,
}

impl Interpreter {
    pub fn new(expr: Expr) -> Interpreter {
        Interpreter { expr }
    }

    pub fn evaluate(expr: Box<Expr>) -> i128 {
        match *expr {
            Expr::Add(left, right) => {
                Interpreter::evaluate(left) + Interpreter::evaluate(right)
            }
            Expr::Subtract(left, right) => {
                Interpreter::evaluate(left) - Interpreter::evaluate(right)
            }
            Expr::Multiply(left, right) => {
                Interpreter::evaluate(left) * Interpreter::evaluate(right)
            }
            Expr::Divide(left, right) => {
                Interpreter::evaluate(left) / Interpreter::evaluate(right)
            }
            Expr::Power(left, right) => {
                let num = Interpreter::evaluate(left);
                num.pow(Interpreter::evaluate(right).try_into().unwrap())
            },
            Expr::Number(token) => {
                if let TokenType::Number(val) = token.tokentype {
                    val
                } else {
                    panic!("Internal error, failed to parse equation")
                }
            }
            Expr::Grouping(group) => Interpreter::evaluate(group),
        }
    }
}
