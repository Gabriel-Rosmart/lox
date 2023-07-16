mod ast;
mod bytecode;
mod lexer;
mod parser;
mod vm;
use bytecode::Chunk;
use lexer::Lexer;
use parser::Parser;
use vm::VM;

use crate::lexer::TokenKind;

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

    // let mut parser = Parser::new(tokens);
    //
    // let ast = parser.expression();
    //
    // println!("{ast:#?}");
    // println!("{parser:#?}");
}
