#[derive(Debug)]
pub enum Token {
    Function,
    Var,
    
    While,
    If, Else,
    
    End,

    Return,
    
    Id (String),
    Int (i32),

    LParen, RParen,
    
    Assign,
    Eq,
    Gt, Ge, Lt, Le,
    Plus, Minus, Mul, Div,
    And, Or, Not,

    EOF,
}
