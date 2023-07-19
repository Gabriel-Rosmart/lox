pub enum LoxError {
    LexerError(String),
    ParseError(String),
    RuntimeError(String),
}

pub struct ErrorBag {
    pub errors: Vec<LoxError>,
}

impl ErrorBag {
    pub fn drain(&mut self) {
        if self.errors.is_empty() {
            return;
        }

        while let Some(e) = self.errors.pop() {
            match e {
                LoxError::LexerError(lxerr) => {
                    eprintln!("{lxerr}");
                }
                _ => {}
            }
        }

        std::process::exit(1);
    }
}

pub fn die(error: LoxError) {
    match error {
        LoxError::ParseError(message) => {
            eprint!("Parse Error: ");
            eprintln!("{message}");
        }
        LoxError::RuntimeError(message) => {
            eprint!("Runtime Error: ");
            eprintln!("{message}");
        }
        _ => {}
    }
    std::process::exit(1);
}
