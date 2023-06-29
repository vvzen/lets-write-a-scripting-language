use crate::core::lexer::Lexer;
use crate::core::tokens::{Token, TokenType};

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

// Simple Parsing
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
]; "Test for operators and parenthesis")]
fn test_next_token(input: &str, expected_results: Vec<Token>) {
    let mut lexer = Lexer::new(input).unwrap();
    for (i, expected_result) in expected_results.iter().enumerate() {
        eprintln!("{i} - char: {}", lexer.char);
        assert_eq!(&lexer.next_token(), expected_result);
    }
}

// A sample script mimicking real life usage of this language.
#[test_case(
"
let five = 5;
let ten = 10;
let add = fn(x, y){
    x + y;
};
let result = add(five, ten);
",
vec![
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
]; "Sample real usage of vvlang")]
// Sample that contains also invalid code,
// to test edge cases of the lexer.
#[test_case(
"
let five = 5;
let ten = 10;
let add = fn(x, y){
    x + y;
};
let result = add(five, ten);
!-/*5
5 < 10 > 5;

if (5 < 10) {
    return true;
}
else {
    return false;
}

10 == 10;
10 != 9;
",
vec![
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
    Token::new(TokenType::Bang, "!"),
    Token::new(TokenType::Minus, "-"),
    Token::new(TokenType::Slash, "/"),
    Token::new(TokenType::Asterisk, "*"),
    Token::new(TokenType::Int, "5"),
    Token::new(TokenType::Int, "5"),
    Token::new(TokenType::Lt, "<"),
    Token::new(TokenType::Int, "10"),
    Token::new(TokenType::Gt, ">"),
    Token::new(TokenType::Int, "5"),
    Token::new(TokenType::Semicolon, ";"),
    Token::new(TokenType::If, "if"),
    Token::new(TokenType::LParen, "("),
    Token::new(TokenType::Int, "5"),
    Token::new(TokenType::Lt, "<"),
    Token::new(TokenType::Int, "10"),
    Token::new(TokenType::RParen, ")"),
    Token::new(TokenType::LBrace, "{"),
    Token::new(TokenType::Return, "return"),
    Token::new(TokenType::True, "true"),
    Token::new(TokenType::Semicolon, ";"),
    Token::new(TokenType::RBrace, "}"),
    Token::new(TokenType::Else, "else"),
    Token::new(TokenType::LBrace, "{"),
    Token::new(TokenType::Return, "return"),
    Token::new(TokenType::False, "false"),
    Token::new(TokenType::Semicolon, ";"),
    Token::new(TokenType::RBrace, "}"),
    Token::new(TokenType::Int, "10"),
    Token::new(TokenType::Eq, "=="),
    Token::new(TokenType::Int, "10"),
    Token::new(TokenType::Semicolon, ";"),
    Token::new(TokenType::Int, "10"),
    Token::new(TokenType::NotEq, "!="),
    Token::new(TokenType::Int, "9"),
    Token::new(TokenType::Semicolon, ";"),
    Token::new(TokenType::EOF, ""),
]; "Sample advanced vvlang usage")]
fn test_next_token_more_complex_string(input: &str, expected_results: Vec<Token>) {
    let mut lexer = Lexer::new(input).unwrap();
    for (i, expected_token) in expected_results.iter().enumerate() {
        let token = lexer.next_token();
        eprintln!("{i} - token: {token:?}");
        assert_eq!(&token, expected_token);
    }
}
