use std::cell::RefCell;
use std::fmt::Display;

use color_eyre::eyre;

use crate::core::lexer::Lexer;
use crate::core::tokens::{Token, TokenType};

mod ast {

    use super::*;

    #[derive(Debug, PartialEq, Clone)]
    /// A 'let' assignment of the form:
    /// let <identifier> = <expression>;
    /// EG:
    ///   let x = 5;
    ///   let x = add(5 + 5);
    pub struct LetStatement {
        pub token: Token,
        pub identifier: Identifier,
        pub value: RefCell<Expression>,
    }

    /// A 'return' assignment of the form:
    /// return <expression>;
    /// EG:
    ///   return 5;
    ///   return add(5 + 5);
    #[derive(Debug, PartialEq, Clone)]
    pub struct ReturnStatement {
        pub token: Token,
        pub value: RefCell<Expression>,
    }

    /// Represents the binding of a variable.
    #[derive(Debug, PartialEq, Clone)]
    pub struct Identifier {
        /// The name of the variable.
        /// EG: let x = 10; -> 'x'
        pub name: String,
    }

    /// A statement consisting of a single expression.
    /// EG:
    ///   5;
    ///   x + 10;
    #[derive(Debug, PartialEq, Clone)]
    pub struct ExpressionStatement {
        pub token: Token,
        pub expression: Expression,
    }

    /// Anything that returns a value.
    /// EG:
    ///   5;
    ///   2+2;
    ///   add(1, 2);
    #[derive(Debug, PartialEq, Clone)]
    pub struct Expression {
        // pub token: Token,
        pub tokens: Vec<Token>,
    }

    impl Expression {
        /// TODO: Compute the value that the expression should return ?
        pub fn compute(&self) -> String {
            todo!();
        }

        pub fn literal(&self) -> String {
            let exp_literal = self
                .tokens
                .iter()
                .filter(|&t| t.r#type != TokenType::Semicolon)
                .map(|t| t.literal.clone())
                .collect::<Vec<String>>()
                .join(" ");

            exp_literal
        }
    }

    /// Using the jergon of the Book, a 'Statement' is basically a
    /// single node of the Abtract Syntax Tree.
    /// We support 3 main types of Statements:
    /// A 'let' assignment, a 'return' statement and a simple Expression.
    #[derive(Debug, Clone, PartialEq)]
    pub enum Statement {
        Assignment(LetStatement),
        Return(ReturnStatement),
        SingleExpression(ExpressionStatement),
    }

    impl Statement {
        fn token_literal(&self) -> String {
            match self {
                Statement::Assignment(let_statement) => let_statement.token.literal.to_owned(),
                Statement::Return(return_statement) => return_statement.token.literal.to_owned(),
                Statement::SingleExpression(expression) => expression.token.literal.to_owned(),
            }
        }
    }

    impl Display for Statement {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            let s = match self {
                Statement::Assignment(let_statement) => {
                    let exp = &let_statement.clone().value.into_inner();
                    format!("let {} = {};", self.token_literal(), &exp.literal())
                }
                Statement::Return(return_statement) => {
                    let exp = &return_statement.clone().value.into_inner();
                    format!("return {};", &exp.literal())
                }
                Statement::SingleExpression(_) => {
                    //
                    self.token_literal()
                }
            };

            write!(f, "{s}")
        }
    }

    pub struct Program {
        pub statements: Vec<Statement>,
    }

    impl Program {
        pub fn new() -> Program {
            Program {
                statements: Vec::new(),
            }
        }

        // FIXME: what needs this?
        fn token_literal(&self) -> String {
            match self.statements.get(0) {
                Some(statement) => statement.token_literal(),
                None => String::new(),
            }
        }
    }
}

pub struct ParserError {
    pub message: String,
    pub line_num: usize,
    pub char_offset: usize,
}

impl ParserError {
    fn new(message: &str, line_num: usize, char_offset: usize) -> ParserError {
        ParserError {
            message: message.to_owned(),
            line_num,
            char_offset,
        }
    }
}

pub struct Parser {
    lexer: Lexer,
    current_token: Token,
    peek_token: Token,
    /// Errors that we encountered while parsing the program.
    pub errors: Vec<ParserError>,
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
            errors: Vec::new(),
        })
    }

    pub fn report_errors(&self) {
        if !self.errors.is_empty() {
            let num_errors = self.errors.len();
            eprintln!(
                "\nFound {} error{} while parsing:",
                num_errors,
                if num_errors <= 1 { "" } else { "s" }
            );

            for error in self.errors.iter() {
                eprint!("line {}; ", error.line_num);
                eprintln!("{}", error.message);
            }
        }
    }

    /// Read the next token
    fn next_token(&mut self) {
        self.current_token = self.peek_token.clone();
        self.peek_token = self.lexer.next_token();
    }

    /// Parse the text given in input (consuming it) and return
    /// the whole program.
    pub fn parse_program(&mut self) -> ast::Program {
        let mut program = ast::Program::new();

        let mut line_num = 1;

        loop {
            // eprintln!("Current token: {:?}", self.current_token);
            // eprintln!("Peek token: {:?}", self.peek_token);

            // If there is nothing more to parse, exit
            if self.peek_token.r#type == TokenType::EOF {
                break;
            }

            let mut statement: Option<ast::Statement> = None;
            match self.current_token.r#type {
                // Newlines have no syntactical meaning, but are useful to keep
                // track of where we are in the source code so that we can emit
                // precise error messages.
                TokenType::NewLine => {
                    line_num += 1;
                }
                TokenType::Let => match self.parse_let_statement() {
                    Ok(s) => {
                        statement = Some(s);
                    }
                    Err(e) => {
                        let error_message = format!("{e}");
                        let error = ParserError::new(&error_message, line_num, 0);
                        self.errors.push(error);
                    }
                },
                TokenType::If => {
                    statement = Some(self.parse_if_statement());
                }
                TokenType::Return => match self.parse_return_statement() {
                    Ok(s) => statement = Some(s),
                    Err(e) => {
                        let error_message = format!("{e}");
                        let error = ParserError::new(&error_message, line_num, 0);
                        self.errors.push(error);
                    }
                },
                _ => {
                    // FIXME: Test this out
                    let error_message =
                        format!("Unsupported token: '{}'", self.current_token.literal);
                    let error = ParserError::new(&error_message, line_num, 0);
                    self.errors.push(error);
                }
            };

            match statement {
                Some(s) => {
                    let type_name = std::any::type_name_of_val(&s);
                    eprintln!("Current statement: '{s}', type: {type_name}");
                    program.statements.push(s);
                }
                None => {}
            }

            self.next_token();
        }

        program
    }

    fn parse_if_statement(&mut self) -> ast::Statement {
        todo!();
    }

    fn parse_let_statement(&mut self) -> eyre::Result<ast::Statement> {
        // The next token should be the identifier name
        // TODO: At some point I might need to implement a custom error type
        if !self.next_token_is_of_type(TokenType::Ident) {
            return Err(eyre::eyre!(
                "Expected identifier, found '{}'",
                self.peek_token.literal
            ));
        }

        // Advance, so we can parse the identifier
        self.next_token();
        let identifier = ast::Identifier {
            name: self.current_token.literal.to_owned(),
        };

        let let_statement_token = self.current_token.clone();

        // After the identifier there should be an '=' sign
        if !self.next_token_is_of_type(TokenType::Assign) {
            return Err(eyre::eyre!(
                "Expected '=' operator, found {}",
                self.peek_token.literal
            ));
        }
        self.next_token();

        // After the '=' there should be an expression
        // FIXME: this is just a placeholder
        let mut exp_literals: Vec<String> = vec![];

        // For now, we consume everything until we reach a semicolon
        // This means we're skipping expressions
        while !self.current_token_is_of_type(TokenType::Semicolon) {
            exp_literals.push(self.peek_token.literal.to_owned());
            self.next_token();

            if self.current_token_is_of_type(TokenType::EOF) {
                return Err(eyre::eyre!("Expected ';', found end of file (EOF)"));
            }
        }

        let exp_literal = exp_literals
            .iter()
            .filter(|&s| s != ";")
            .map(|s| s.clone())
            .collect::<Vec<String>>()
            .join(" ");

        let exp_token = Token {
            r#type: TokenType::Illegal,
            literal: exp_literal,
        };

        let expression = ast::Expression {
            tokens: vec![exp_token],
        };

        let statement = ast::LetStatement {
            token: let_statement_token,
            identifier,
            value: RefCell::new(expression),
        };

        Ok(ast::Statement::Assignment(statement))
    }

    fn parse_return_statement(&mut self) -> eyre::Result<ast::Statement> {
        // After the 'return' there should be an expression
        // FIXME: this is just a placeholder
        let mut exp_literals: Vec<String> = vec![];

        // For now, we consume everything until we reach a semicolon
        // This means we're skipping expressions
        while !self.current_token_is_of_type(TokenType::Semicolon) {
            exp_literals.push(self.peek_token.literal.to_owned());
            self.next_token();

            if self.current_token_is_of_type(TokenType::EOF) {
                return Err(eyre::eyre!("Expected ';', found end of file (EOF)"));
            }
        }

        let exp_literal = exp_literals
            .iter()
            .filter(|&s| s != ";")
            .map(|s| s.clone())
            .collect::<Vec<String>>()
            .join(" ");

        let exp_token = Token {
            r#type: TokenType::Illegal,
            literal: exp_literal,
        };

        let expression = ast::Expression {
            tokens: vec![exp_token],
        };
        let statement = ast::ReturnStatement {
            token: Token {
                r#type: TokenType::Return,
                literal: "return".to_owned(),
            },
            value: RefCell::new(expression),
        };

        Ok(ast::Statement::Return(statement))
    }

    fn current_token_is_of_type(&self, t: TokenType) -> bool {
        self.current_token.r#type == t
    }

    fn next_token_is_of_type(&self, t: TokenType) -> bool {
        self.peek_token.r#type == t
    }
}

#[cfg(test)]
#[path = "../tests/parser.rs"]
mod parser_tests;
