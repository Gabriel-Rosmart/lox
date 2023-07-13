mod lexer;
use lexer::{Lexer, TokenType};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let mut lexer = Lexer::new(std::fs::read_to_string(&args[1]).unwrap());

    while let Ok(token) = lexer.next_token() {
        match token {
            TokenType::Eof => break,
            _ => println!("{token:?}"),
        }
    }
}
