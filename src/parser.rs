use crate::{
    ast::{Binary, Expression, Literal, Unary},
    lexer::TokenKind,
};

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

    fn previous(&self) -> Option<&TokenKind> {
        self.tokens.get(self.cursor - 1)
    }

    fn check(&self, token: &TokenKind) -> bool {
        if self.cursor >= self.tokens.len() {
            false
        } else {
            match self.peek().unwrap() {
                token => true,
                _ => false,
            }
        }
    }

    fn matches(&mut self, tokens: Vec<TokenKind>) -> bool {
        for token in tokens {
            if self.check(&token) {
                self.next_token();
                return true;
            }
        }

        false
    }

    fn expression(&mut self) -> Box<dyn Expression> {
        self.equality()
    }

    fn equality(&mut self) -> Box<dyn Expression> {
        let mut expr = self.comparison();

        while self.matches(vec![TokenKind::NotEqual, TokenKind::Equal]) {
            let operator = self.previous().cloned().unwrap();
            let right = self.comparison();
            expr = Box::new(Binary::new(operator, expr, right));
        }

        expr
    }

    fn comparison(&mut self) -> Box<dyn Expression> {
        let mut expr = self.term();

        while self.matches(vec![
            TokenKind::GreaterThan,
            TokenKind::GreaterEqual,
            TokenKind::LessThan,
            TokenKind::LessEqual,
        ]) {
            let operator = self.previous().cloned().unwrap();
            let right = self.term();
            expr = Box::new(Binary::new(operator, expr, right));
        }

        expr
    }

    fn term(&mut self) -> Box<dyn Expression> {
        let mut expr = self.factor();

        while self.matches(vec![TokenKind::Minus, TokenKind::Plus]) {
            let operator = self.previous().cloned().unwrap();
            let right = self.factor();
            expr = Box::new(Binary::new(operator, expr, right));
        }

        expr
    }

    fn factor(&mut self) -> Box<dyn Expression> {
        let mut expr = self.unary();

        while self.matches(vec![TokenKind::ForwardSlash, TokenKind::Asterisk]) {
            let operator = self.previous().cloned().unwrap();
            let right = self.unary();
            expr = Box::new(Binary::new(operator, expr, right));
        }

        expr
    }

    fn unary(&mut self) -> Box<dyn Expression> {
        if self.matches(vec![TokenKind::Bang, TokenKind::Minus]) {
            let operator = self.previous().cloned().unwrap();
            let right = self.unary();
            return Box::new(Unary::new(operator, right));
        }

        self.primary()
    }

    fn primary(&mut self) -> Box<dyn Expression> {
        if self.matches(vec![TokenKind::False]) {
            return Box::new(Literal::new(false));
        }
        if self.matches(vec![TokenKind::True]) {
            return Box::new(Literal::new(true));
        }
        if self.matches(vec![TokenKind::None]) {
            return Box::new(Literal::new(None));
        }

        if self.matches(vec![
            TokenKind::Integer(usize),
            TokenKind::Decimal(f64),
            TokenKind::QuotedString(String),
        ]) {
            return Box::new(Literal::new(self.previous().cloned().unwrap()));
        }
    }
}
