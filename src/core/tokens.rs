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
    Eq,
    NotEq,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Lt,
    Gt,

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
    True,
    False,
    If,
    Else,
    Return,

    // No-ops
    NewLine,
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
            Self::Comma => s = ",",
            Self::Semicolon => s = ";",
            Self::LParen => s = "(",
            Self::RParen => s = ")",
            Self::LBrace => s = "{",
            Self::RBrace => s = "}",
            // Operators
            Self::Eq => s = "==",
            Self::NotEq => s = "!=",
            Self::Plus => s = "+",
            Self::Minus => s = "-",
            Self::Slash => s = "/",
            Self::Gt => s = ">",
            Self::Lt => s = "<",
            Self::Bang => s = "!",
            Self::Asterisk => s = "*",
            // Keywords
            Self::Function => s = "fn",
            Self::Let => s = "let",
            Self::True => s = "true",
            Self::False => s = "false",
            Self::If => s = "if",
            Self::Else => s = "else",
            Self::Return => s = "return",
            // No-op
            Self::NewLine => s = "\n",
        }
        write!(f, "{s}")
    }
}

#[derive(Debug, PartialEq, Clone)]
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
