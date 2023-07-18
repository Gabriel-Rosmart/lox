use crate::{
    ast::{BinaryExpr, Expression, LiteralKind, UnaryExpr},
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

    pub fn expression(&mut self) -> Box<Expression> {
        self.equality()
    }

    pub fn equality(&mut self) -> Box<Expression> {
        let mut expr = self.comparison();

        loop {
            match self.peek() {
                Some(&TokenKind::NotEqual) | Some(&TokenKind::Equal) => {
                    let operator = self.to_next_token().cloned().unwrap();
                    let right = self.comparison();
                    expr = Box::new(Expression::Binary(BinaryExpr {
                        operator,
                        lhs: expr,
                        rhs: right,
                    }));
                }
                _ => break,
            };
        }

        expr
    }

    pub fn comparison(&mut self) -> Box<Expression> {
        let mut expr = self.term();

        loop {
            match self.peek() {
                Some(&TokenKind::GreaterThan)
                | Some(&TokenKind::GreaterEqual)
                | Some(&TokenKind::LessThan)
                | Some(&TokenKind::LessEqual) => {
                    let operator = self.to_next_token().cloned().unwrap();
                    let right = self.term();
                    expr = Box::new(Expression::Binary(BinaryExpr {
                        operator,
                        lhs: expr,
                        rhs: right,
                    }));
                }
                _ => break,
            };
        }

        expr
    }

    pub fn term(&mut self) -> Box<Expression> {
        let mut expr = self.factor();

        loop {
            match self.peek() {
                Some(&TokenKind::Minus) | Some(&TokenKind::Plus) => {
                    let operator = self.to_next_token().cloned().unwrap();
                    let right = self.factor();
                    expr = Box::new(Expression::Binary(BinaryExpr {
                        operator,
                        lhs: expr,
                        rhs: right,
                    }));
                }
                _ => break,
            };
        }

        expr
    }

    pub fn factor(&mut self) -> Box<Expression> {
        let mut expr = self.unary();

        loop {
            match self.peek() {
                Some(&TokenKind::ForwardSlash)
                | Some(&TokenKind::Asterisk)
                | Some(&TokenKind::Percentage) => {
                    let operator = self.to_next_token().cloned().unwrap();
                    let right = self.unary();
                    expr = Box::new(Expression::Binary(BinaryExpr {
                        operator,
                        lhs: expr,
                        rhs: right,
                    }));
                }
                _ => break,
            };
        }

        expr
    }

    pub fn unary(&mut self) -> Box<Expression> {
        match self.peek() {
            Some(&TokenKind::Bang) | Some(TokenKind::Minus) => {
                let operator = self.to_next_token().cloned().unwrap();
                let right = self.unary();
                return Box::new(Expression::Unary(UnaryExpr {
                    operator,
                    rhs: right,
                }));
            }
            Some(_) => self.primary(),
            None => panic!("Fix this"),
        }
    }

    pub fn primary(&mut self) -> Box<Expression> {
        let token: Box<Expression> = match self.peek() {
            Some(&TokenKind::True) => Box::new(Expression::Literal(LiteralKind::True)),
            Some(&TokenKind::False) => Box::new(Expression::Literal(LiteralKind::False)),
            Some(&TokenKind::None) => Box::new(Expression::Literal(LiteralKind::NULL)),
            Some(&TokenKind::Integer(i)) => Box::new(Expression::Literal(LiteralKind::Integer(i))),
            Some(&TokenKind::Decimal(d)) => Box::new(Expression::Literal(LiteralKind::Decimal(d))),
            Some(&TokenKind::QuotedString(ref s)) => {
                Box::new(Expression::Literal(LiteralKind::QuotedString(s.clone())))
            }
            Some(&TokenKind::OpenParen) => {
                self.to_next_token();
                let expr = self.expression();
                match self.peek() {
                    Some(&TokenKind::CloseParen) => {}
                    Some(_) => panic!("Unclosed parentheses"),
                    None => {}
                };

                Box::new(Expression::Grouping(expr))
            }
            Some(_) => panic!("Expected primary expression"),
            None => panic!("Unexpected Eof"),
        };

        self.to_next_token();
        token
    }
}
