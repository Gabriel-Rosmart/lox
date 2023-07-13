mod lexer;
use lexer::Lexer;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    let lexer = Lexer::new(std::fs::read_to_string(&args[1]).unwrap());

    for token in lexer {
        println!("{token:?}");
    }
}
