use color_eyre::eyre;

mod core;

/// Start a REPL that prints back the result
/// of tokenizing what the user has typed.
fn repl() -> eyre::Result<()> {
    eprintln!("Welcome to vvlang!");

    loop {
        eprint!(">>> ");

        let mut user_input = String::new();
        std::io::stdin().read_line(&mut user_input)?;

        if &user_input == "exit()\n" {
            eprintln!("Exiting..");
            break;
        }
        let mut lexer = core::lexer::Lexer::new(&user_input)?;

        loop {
            let token = lexer.next_token();
            println!("{token:?}");

            if token.r#type == core::tokens::TokenType::EOF {
                break;
            }
        }
    }

    Ok(())
}

fn main() -> eyre::Result<()> {
    // repl()?;
    use crate::core::parser::Parser;
    let text = "
    let something = 5;
    return 10;
    5;";

    let mut parser = Parser::new(text)?;
    parser.parse_program();
    parser.report_errors();

    eprintln!("");

    Ok(())
}
