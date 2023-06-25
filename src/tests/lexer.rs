use crate::core::lexer::Lexer;
use crate::core::tokens::{Token, TokenType};

use indoc::indoc;
use test_case::test_case;

// Initialization
#[test]
fn test_new() {
    let input = "let a = 5;";
    assert!(Lexer::new(input).is_ok());
}

#[test]
fn test_new_error() {
    let input = "";
    assert!(Lexer::new(input).is_err());
}

// Parsing
#[test_case("+", vec![Token::new(TokenType::Plus, "+")])]
#[test_case("=+(){},;", vec![
        Token::new(TokenType::Assign, "="),
        Token::new(TokenType::Plus, "+"),
        Token::new(TokenType::LParen, "("),
        Token::new(TokenType::RParen, ")"),
        Token::new(TokenType::LBrace, "{"),
        Token::new(TokenType::RBrace, "}"),
        Token::new(TokenType::Comma, ","),
        Token::new(TokenType::Semicolon, ";"),
        Token::new(TokenType::EOF, ""),
])]
fn test_next_token(input: &str, expected_results: Vec<Token>) {
    let mut lexer = Lexer::new(input).unwrap();
    for (i, expected_result) in expected_results.iter().enumerate() {
        eprintln!("{i} - char: {}", lexer.char);
        assert_eq!(&lexer.next_token(), expected_result);
    }
}

#[test]
fn test_next_token_more_complex_string() {
    let text = indoc! {"
        let five = 5;
        let ten = 10;
        let add = fn(x, y){
            x + y;
        };
        let result = add(five, ten);
    "};
    let expected_results = vec![
        Token::new(TokenType::Let, "let"),
        Token::new(TokenType::Ident, "five"),
        Token::new(TokenType::Assign, "="),
        Token::new(TokenType::Int, "5"),
        Token::new(TokenType::Semicolon, ";"),
        Token::new(TokenType::Let, "let"),
        Token::new(TokenType::Ident, "ten"),
        Token::new(TokenType::Assign, "="),
        Token::new(TokenType::Int, "10"),
        Token::new(TokenType::Semicolon, ";"),
        Token::new(TokenType::Let, "let"),
        Token::new(TokenType::Ident, "add"),
        Token::new(TokenType::Assign, "="),
        Token::new(TokenType::Function, "fn"),
        Token::new(TokenType::LParen, "("),
        Token::new(TokenType::Ident, "x"),
        Token::new(TokenType::Comma, ","),
        Token::new(TokenType::Ident, "y"),
        Token::new(TokenType::RParen, ")"),
        Token::new(TokenType::LBrace, "{"),
        Token::new(TokenType::Ident, "x"),
        Token::new(TokenType::Plus, "+"),
        Token::new(TokenType::Ident, "y"),
        Token::new(TokenType::Semicolon, ";"),
        Token::new(TokenType::RBrace, "}"),
        Token::new(TokenType::Semicolon, ";"),
        Token::new(TokenType::Let, "let"),
        Token::new(TokenType::Ident, "result"),
        Token::new(TokenType::Assign, "="),
        Token::new(TokenType::Ident, "add"),
        Token::new(TokenType::LParen, "("),
        Token::new(TokenType::Ident, "five"),
        Token::new(TokenType::Comma, ","),
        Token::new(TokenType::Ident, "ten"),
        Token::new(TokenType::RParen, ")"),
        Token::new(TokenType::Semicolon, ";"),
        Token::new(TokenType::EOF, ""),
    ];
    let mut lexer = Lexer::new(text).unwrap();
    for (i, expected_token) in expected_results.iter().enumerate() {
        let token = lexer.next_token();
        eprintln!("{i} - token: {token:?}");
        assert_eq!(&token, expected_token);
    }
}
