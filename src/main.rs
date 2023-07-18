mod ast;
mod bytecode;
mod interpreter;
mod lexer;
mod parser;
mod vm;
// use interpreter::interpret;
use lexer::Lexer;

use crate::{lexer::TokenKind, parser::Parser};

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let lexer = Lexer::new(std::fs::read_to_string(&args[1]).unwrap());

    let tokens: Vec<_> = lexer
        .into_iter()
        .filter(|token| match token {
            TokenKind::Comment => false,
            _ => true,
        })
        .collect();

    let mut parser = Parser::new(tokens);

    let ast = parser.expression();

    // interpret(ast);
    println!("{ast:#?}");
    // println!("{parser:#?}");
}
