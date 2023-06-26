use color_eyre::eyre;
use lazy_static::lazy_static;
use phf::phf_map;

use crate::core::tokens::{Token, TokenType};

lazy_static! {

    /// Characters considered valid to be used in identifiers
    pub static ref LETTERS: Vec<char> = {
        let letters =
            // Extra supported chars
            std::iter::once(b'_')
            // Any lower/upper case alphabetic char
            .chain(b'a'..b'z')
            .chain(b'A'..b'Z')
            .map(|c| c as char)
            .collect();

        letters
    };
}

pub const WHITESPACE_CHARS: [char; 4] = [' ', '\t', '\r', '\n'];

/// Language reserved keywords
pub static KEYWORDS: phf::Map<&'static str, TokenType> = phf_map! {
    "fn" => TokenType::Function,
    "let" => TokenType::Let
};

pub struct Lexer<'a> {
    /// Text to lex
    input: &'a str,
    /// Current position in ``input``, points to the current char
    position: usize,
    /// Current reading position in ``input``, after the current char
    read_position: usize,
    /// Current char under examination
    pub r#char: char,
}

impl Lexer<'_> {
    pub fn new(text: &str) -> eyre::Result<Lexer> {
        let first_char = match text.chars().nth(0) {
            Some(c) => c,
            None => {
                eyre::bail!("No character found in position '0' in given text: '{text}'");
            }
        };

        Ok(Lexer {
            input: text,
            position: 0,
            read_position: 1,
            r#char: first_char,
        })
    }

    fn skip_whitspace(&mut self) {
        while WHITESPACE_CHARS.contains(&self.char) {
            self.read_char();
        }
    }

    pub fn next_token(&mut self) -> Token {
        self.skip_whitspace();

        // Special cases first
        // A potential keyword or variable name
        if is_letter(self.char) {
            return self.read_identifier();
        }

        // Numbers
        if self.char.is_numeric() {
            return self.read_number();
        }

        // Any other token we support
        let token = match self.char {
            '=' => Token::new(TokenType::Assign, &self.char.to_string()),
            ',' => Token::new(TokenType::Comma, &self.char.to_string()),
            '+' => Token::new(TokenType::Plus, &self.char.to_string()),
            ';' => Token::new(TokenType::Semicolon, &self.char.to_string()),
            '(' => Token::new(TokenType::LParen, &self.char.to_string()),
            ')' => Token::new(TokenType::RParen, &self.char.to_string()),
            '{' => Token::new(TokenType::LBrace, &self.char.to_string()),
            '}' => Token::new(TokenType::RBrace, &self.char.to_string()),
            '\0' => Token::new(TokenType::EOF, ""),
            _ => Token::new(TokenType::Illegal, &self.char.to_string()),
        };

        self.read_char();
        token
    }

    fn read_identifier(&mut self) -> Token {
        // Read all chars until we find a non letter
        let mut letters: Vec<char> = Vec::new();
        while is_letter(self.char) {
            letters.push(self.char);
            self.read_char();
        }

        let s: String = letters.iter().collect();

        // Check whether this is a reserved keyword or not
        // If not, we consider it to be a valid identifier name
        let token_type = match KEYWORDS.get(s.as_str()) {
            Some(keyword_type) => keyword_type.clone(),
            None => TokenType::Ident,
        };

        let token = Token::new(token_type, &s);
        token
    }

    pub fn read_number(&mut self) -> Token {
        // Read all chars until we find a non number
        let mut digits: Vec<char> = Vec::new();
        while self.char.is_numeric() {
            digits.push(self.char);
            self.read_char();
        }

        let s: String = digits.iter().collect();
        let token = Token::new(TokenType::Int, &s);
        token
    }

    pub fn read_char(&mut self) {
        self.char = match self.input.chars().nth(self.read_position) {
            Some(c) => c,
            None => '\0', // ASCII NUL character
        };

        self.position = self.read_position;
        self.read_position += 1;
    }
}

fn is_letter(c: char) -> bool {
    LETTERS.contains(&c)
}

#[cfg(test)]
#[path = "../tests/lexer.rs"]
mod lexer_tests;
