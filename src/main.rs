mod ast;
mod bytecode;
mod error;
mod interpreter;
mod lexer;
mod parser;
mod vm;
use interpreter::Interpreter;
use lexer::Lexer;

use crate::{error::ErrorBag, lexer::TokenKind, parser::Parser};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let mut error_bag = ErrorBag { errors: vec![] };

    let lexer = Lexer::new(std::fs::read_to_string(&args[1]).unwrap(), &mut error_bag);

    let tokens: Vec<_> = lexer
        .into_iter()
        .filter(|token| match token.kind {
            TokenKind::Comment | TokenKind::Invalid => false,
            _ => true,
        })
        .collect();

    error_bag.drain();

    let mut parser = Parser::new(tokens);

    let ast = parser.parse();

    let mut interpreter = Interpreter::new();
    interpreter.execute(ast);
    // println!("{res:?}");
    // println!("{tokens:#?}");
    // println!("{ast:#?}");
    // println!("{parser:#?}");
}
