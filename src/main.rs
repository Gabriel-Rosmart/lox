mod lexer;
use lexer::Lexer;

mod lx;
use lx::Lexer as NLex;

fn main() {
    let args = std::env::args().collect::<Vec<String>>();

    // let lexer = Lexer::new(std::fs::read_to_string(&args[1]).unwrap());
    //
    // for token in lexer {
    //     println!("{token:?}");
    // }

    let lexer = NLex::new(std::fs::read_to_string(&args[1]).unwrap());

    for token in lexer {
        println!("{token:?}");
    }
}
