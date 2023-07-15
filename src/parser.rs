use crate::{
    ast::{Binary, Expression, Grouping, Literal, Unary},
    lexer::TokenKind,
};

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<TokenKind>,
    cursor: usize,
}

impl Parser {
    pub fn new(tokens: Vec<TokenKind>) -> Self {
        Self { tokens, cursor: 0 }
    }

    pub fn to_next_token(&mut self) -> Option<&TokenKind> {
        let token = self.tokens.get(self.cursor);

        if token.is_some() {
            self.cursor += 1;
        }

        token
    }

    fn peek(&self) -> Option<&TokenKind> {
        self.tokens.get(self.cursor)
    }

    /// Priority 7
    pub fn expression(&mut self) -> Box<dyn Expression> {
        self.equality()
    }

    /// Priority 6
    fn equality(&mut self) -> Box<dyn Expression> {
        let mut expr = self.comparison();

        loop {
            match self.peek() {
                Some(&TokenKind::NotEqual) | Some(&TokenKind::Equal) => {
                    let operator = self.to_next_token().cloned().unwrap();
                    let right = self.comparison();
                    expr = Box::new(Binary::new(operator, expr, right));
                }
                _ => break,
            };
        }

        expr
    }

    /// Priority 5
    fn comparison(&mut self) -> Box<dyn Expression> {
        let mut expr = self.term();

        loop {
            match self.peek() {
                Some(&TokenKind::GreaterThan)
                | Some(&TokenKind::GreaterEqual)
                | Some(&TokenKind::LessThan)
                | Some(&TokenKind::LessEqual) => {
                    let operator = self.to_next_token().cloned().unwrap();
                    let right = self.term();
                    expr = Box::new(Binary::new(operator, expr, right));
                }
                _ => break,
            };
        }

        expr
    }

    /// Priority 4
    fn term(&mut self) -> Box<dyn Expression> {
        let mut expr = self.factor();

        loop {
            match self.peek() {
                Some(&TokenKind::Minus) | Some(&TokenKind::Plus) => {
                    let operator = self.to_next_token().cloned().unwrap();
                    let right = self.factor();
                    expr = Box::new(Binary::new(operator, expr, right));
                }
                _ => break,
            };
        }

        expr
    }

    /// Priority 3
    fn factor(&mut self) -> Box<dyn Expression> {
        let mut expr = self.unary();

        loop {
            match self.peek() {
                Some(&TokenKind::ForwardSlash) | Some(&TokenKind::Asterisk) => {
                    let operator = self.to_next_token().cloned().unwrap();
                    let right = self.unary();
                    expr = Box::new(Binary::new(operator, expr, right));
                }
                _ => break,
            };
        }

        expr
    }

    /// Priority 2
    fn unary(&mut self) -> Box<dyn Expression> {
        match self.peek() {
            Some(&TokenKind::Bang) | Some(TokenKind::Minus) => {
                let operator = self.to_next_token().cloned().unwrap();
                let right = self.unary();
                return Box::new(Unary::new(operator, right));
            }
            Some(_) => self.primary(),
            None => panic!("Fix this"),
        }
    }

    /// Priority 1
    fn primary(&mut self) -> Box<dyn Expression> {
        let token: Box<dyn Expression> = match self.peek() {
            Some(&TokenKind::True) => Box::new(Literal::new(true)),
            Some(&TokenKind::False) => Box::new(Literal::new(false)),
            Some(&TokenKind::None) => Box::new(Literal::new(Option::None::<usize>)),
            Some(&TokenKind::Integer(i)) => Box::new(Literal::new(i)),
            Some(&TokenKind::Decimal(d)) => Box::new(Literal::new(d)),
            Some(&TokenKind::QuotedString(ref s)) => Box::new(Literal::new(s.clone())),
            Some(&TokenKind::OpenParen) => {
                self.to_next_token();
                let expr = self.expression();
                match self.peek() {
                    Some(&TokenKind::CloseParen) => {}
                    Some(_) => panic!("Unclosed parentheses"),
                    None => {}
                };

                Box::new(Grouping::new(expr))
            }
            Some(_) => panic!("Expected primary expression"),
            None => panic!("Unexpected Eof"),
        };

        self.to_next_token();
        token
    }
}
