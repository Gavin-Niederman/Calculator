enum TokenType {
    LeftParen,
    RightParen,
    Add,
    Subtract,
    Multiply,
    Divide,
    Number(i128)
} 

struct Token {
    tokentype: TokenType,
    location: i32,
}

struct Scanner {
    equation: String,
    tokens: Vec<Token>,
    start: i32,
    current: i32
}

fn main() {
    println!("Hello, world!");
}

impl Scanner {
    fn scan_tokens(&mut self) {
        
    }
}