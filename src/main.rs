mod ast;
mod lexer;
mod parser;
use lexer::Lexer;
use parser::Parser;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let lexer = Lexer::new(std::fs::read_to_string(&args[1]).unwrap());

    let tokens: Vec<_> = lexer.into_iter().collect();

    let mut parser = Parser::new(tokens);

    let ast = parser.expression();

    println!("{ast:#?}");
}
