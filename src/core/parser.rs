use std::any::Any;

use super::lexer::Lexer;
use crate::core::{
    parser::ast::{Identifier, LetStatement},
    tokens::{Token, TokenType},
};

mod ast {

    use super::*;

    pub enum NodeType {
        Statement,
        Expression,
    }

    /// A node composing our Abstract Syntax Tree.
    pub trait Node {
        fn token_literal(&self) -> String;
        fn ast_node_type(&self) -> NodeType;
    }

    /// A let statement of the form:
    /// let <identifier> = <expression>;
    /// EG:
    ///   let x = 5;
    ///   let x = add(5 + 5);
    pub struct LetStatement {
        token: Token,
        name: Identifier,
        expression: String,
    }

    /// Represents the binding of a variable
    pub struct Identifier {
        token: Token,
        value: String,
    }

    impl Node for LetStatement {
        fn token_literal(&self) -> String {
            self.token.literal.to_owned()
        }

        fn ast_node_type(&self) -> NodeType {
            return NodeType::Statement;
        }
    }

    pub struct Program {
        pub nodes: Vec<Box<dyn Node>>,
    }

    impl Program {
        pub fn new() -> Program {
            Program { nodes: Vec::new() }
        }

        fn token_literal(&self) -> String {
            match self.nodes.get(0) {
                Some(node) => {
                    return node.token_literal();
                }
                None => {
                    return String::new();
                }
            }
        }
    }
}

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
}

fn parse_let_statement() -> Box<dyn ast::Node> {
    todo!();
}

fn parse_return_statement() -> Box<dyn ast::Node> {
    todo!();
}

fn parse_if_statement() -> Box<dyn ast::Node> {
    todo!();
}

impl Parser {
    /// Create a new parser from the given text.
    pub fn new(text: &str) -> eyre::Result<Parser> {
        let mut lexer = Lexer::new(text)?;
        let first_token = lexer.next_token();
        let second_token = lexer.next_token();
        Ok(Parser {
            lexer,
            current_token: first_token,
            peek_token: second_token,
        })
    }

    /// Read the next token
    fn next_token(&mut self) {
        self.current_token = self.lexer.next_token();
        self.peek_token = self.lexer.next_token();
    }

    pub fn parse_program(&mut self) -> ast::Program {
        let mut program = ast::Program::new();

        loop {
            eprintln!("Current token: {:?}", self.current_token);
            eprintln!("Peek token: {:?}", self.peek_token);

            // If there is nothing more to parse, exit
            if self.peek_token.r#type == TokenType::EOF {
                break;
            }

            let statement = match self.current_token.r#type {
                TokenType::Let => parse_let_statement(),
                TokenType::If => parse_if_statement(),
                _ => {
                    //
                    parse_let_statement()
                }
            };

            program.nodes.push(statement);
            self.next_token();
        }

        program
    }
}

#[cfg(test)]
#[path = "../tests/parser.rs"]
mod parser_tests;
