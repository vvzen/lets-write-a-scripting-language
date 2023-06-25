use color_eyre::eyre;

mod core;

fn main() -> eyre::Result<()> {
    let text = "let a = 5;";
    let mut lexer = core::lexer::Lexer::new(text)?;

    eprintln!("First char read: {}", lexer.char);

    loop {
        let token = lexer.next_token();
        eprintln!("Token: {token:?}");

        if token.r#type == core::tokens::TokenType::EOF {
            break;
        }
    }

    Ok(())
}
