use std::fmt::Display;

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Illegal, // Unknown token
    EOF,     // End of File

    // Identifiers + literals
    Ident, // foo, bar, x, y..
    Int,   // 123456

    // Operators
    Assign,
    Plus,

    // Delimiters
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,

    // Keywords
    Function,
    Let,
}

impl Display for TokenType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s;
        match self {
            Self::Illegal => s = "Illegal",
            Self::EOF => s = "EOF",
            Self::Ident => s = "IDENT",
            Self::Int => s = "int",
            Self::Assign => s = "=",
            Self::Plus => s = "+",
            Self::Comma => s = ",",
            Self::Semicolon => s = ";",
            Self::LParen => s = "(",
            Self::RParen => s = ")",
            Self::LBrace => s = "{",
            Self::RBrace => s = "}",
            Self::Function => s = "function",
            Self::Let => s = "let",
        }
        write!(f, "{s}")
    }
}

#[derive(Debug, PartialEq)]
pub struct Token {
    pub r#type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(r#type: TokenType, literal: &str) -> Token {
        Token {
            r#type,
            literal: literal.to_owned(),
        }
    }
}
