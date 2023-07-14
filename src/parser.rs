use crate::lexer::TokenKind;

pub struct Parser {
    tokens: Vec<TokenKind>,
    cursor: usize,
}

impl Parser {
    pub fn new(tokens: Vec<TokenKind>) -> Self {
        Self { tokens, cursor: 0 }
    }

    pub fn next_token(&mut self) -> Option<&TokenKind> {
        let token = self.tokens.get(self.cursor);

        if token.is_some() {
            self.cursor += 1;
        }

        token
    }

    fn peek(&self) -> Option<&TokenKind> {
        self.tokens.get(self.cursor)
    }
}
