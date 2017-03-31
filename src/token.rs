
use value::*;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    Toss,
    Catch,
    Joke,
    Curse,
    Equal,
    Greater,
    Lesser,
    And,
    Or,
    Not,
    If,
    While,
    Else,
    End,
    Append,
    Drop,
    Newline,
    Number(i64),
    Bool(bool),
    EndOfFile,
    None, // filter out later
}

#[derive(Debug)]
pub struct Token {
    pub which: TokenType,
    pub line: u64,
}

impl Token {
    pub fn new(which: TokenType, line: u64) -> Token {
        Token {
            which: which,
            line: line,
        }
    }

    pub fn is_value(&self) -> bool {
        match self.which {
            TokenType::Number(_) => true,
            TokenType::Bool(_) => true,
            _ => false
        }
    }

    pub fn to_value(&self) -> Value {
        match self.which {
            TokenType::Number(n) => Value::Number(n),
            TokenType::Bool(b) => Value::Bool(b),
            _ => panic!("Called to_value on non-value")
        }
    }
}

