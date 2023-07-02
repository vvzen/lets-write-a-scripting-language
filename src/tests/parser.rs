use crate::core::lexer::Lexer;
use crate::core::parser::ast::Statement;
use crate::core::parser::Parser;
use crate::core::tokens::{Token, TokenType};

use test_case::test_case;

#[test_case("let x = 5;", vec!["x"], 1; "Simple parser test with a single let assignment")]
#[test_case("let x = 5; let y = 10;", vec!["x", "y"], 2; "Simple parser test with two let assignments on a single line")]
#[test_case("
let x = 5;
let y = 10;
let foobar = 838383;
", vec!["x", "y", "foobar"], 3; "Simple parser test with a let assignment per line"
)]
fn test_let_statements(
    input: &str,
    expected_identifiers_names: Vec<&str>,
    expected_num_statements: usize,
) {
    let mut parser = Parser::new(input).unwrap();
    let program = parser.parse_program();

    assert_eq!(
        program.statements.len(),
        expected_num_statements,
        "Program should contain {expected_num_statements} statements"
    );

    for (statement, expected_identifier_name) in
        std::iter::zip(program.statements.iter(), expected_identifiers_names)
    {
        eprintln!(
            "Current statement: '{statement}' - expected identifier name: {}",
            expected_identifier_name
        );

        // Every statement should be a let assignment
        assert!(std::matches!(statement, Statement::Assignment(_)));

        match statement {
            Statement::Assignment(let_statement) => {
                assert_eq!(let_statement.identifier.name, expected_identifier_name);
            }
            _ => {}
        }
    }
}
