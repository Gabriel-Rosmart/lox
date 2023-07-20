use crate::{
    ast::{BinaryExpr, Expression, LiteralKind, Statement, UnaryExpr},
    lexer::{Token, TokenKind},
};

#[derive(Debug)]
pub struct Parser {
    tokens: Vec<Token>,
    cursor: usize,
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, cursor: 0 }
    }

    pub fn to_next_token(&mut self) -> Option<&Token> {
        let token = self.tokens.get(self.cursor);

        if token.is_some() {
            self.cursor += 1;
        }

        token
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.cursor)
    }

    pub fn parse(&mut self) -> Vec<Box<Statement>> {
        self.statement()
    }

    pub fn statement(&mut self) -> Vec<Box<Statement>> {
        let mut stmts: Vec<Box<Statement>> = Vec::new();

        loop {
            match self.peek().map(|t| &t.kind) {
                Some(&TokenKind::Print) => {
                    self.to_next_token();
                    let value = self.expression();
                    match self.peek().map(|t| &t.kind) {
                        Some(&TokenKind::Semicolon) => {
                            stmts.push(Box::new(Statement::Print(value)));
                            self.to_next_token();
                        }
                        Some(_) => {
                            crate::error::die(crate::error::LoxError::RuntimeError(format!(
                                "Expected semicolon at end of statement at line {}",
                                self.peek().cloned().unwrap().span.line
                            )));
                            unreachable!()
                        }
                        None => {
                            crate::error::die(crate::error::LoxError::RuntimeError(
                                "Expected semicolon at enf of statement at end of file".to_string(),
                            ));
                            unreachable!()
                        }
                    };
                }
                Some(_) => stmts.push(Box::new(Statement::Expr(self.expression()))),
                None => break,
            }
        }

        stmts
    }

    pub fn expression(&mut self) -> Box<Expression> {
        self.equality()
    }

    pub fn equality(&mut self) -> Box<Expression> {
        let mut expr = self.comparison();

        loop {
            match self.peek().map(|t| &t.kind) {
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
            match self.peek().map(|t| &t.kind) {
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
            match self.peek().map(|t| &t.kind) {
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
            match self.peek().map(|t| &t.kind) {
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
        match self.peek().map(|t| &t.kind) {
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
        let token: Box<Expression> = match self.peek().map(|t| &t.kind) {
            Some(&TokenKind::True) => Box::new(Expression::Literal(LiteralKind::Boolean(true))),
            Some(&TokenKind::False) => Box::new(Expression::Literal(LiteralKind::Boolean(false))),
            Some(&TokenKind::None) => Box::new(Expression::Literal(LiteralKind::None)),
            Some(&TokenKind::Integer(i)) => Box::new(Expression::Literal(LiteralKind::Integer(i))),
            Some(&TokenKind::Decimal(d)) => Box::new(Expression::Literal(LiteralKind::Decimal(d))),
            Some(&TokenKind::QuotedString(ref s)) => {
                Box::new(Expression::Literal(LiteralKind::QuotedString(s.clone())))
            }
            Some(&TokenKind::OpenParen) => {
                self.to_next_token();
                let expr = self.expression();
                match self.peek().map(|t| &t.kind) {
                    Some(&TokenKind::CloseParen) => {}
                    Some(_) => panic!("Unclosed parentheses"),
                    None => {}
                };

                Box::new(Expression::Grouping(expr))
            }
            Some(other) => {
                let span = self.peek().cloned().unwrap().span;
                crate::error::die(crate::error::LoxError::ParseError(format!(
                    "Expected primary expression got \x1b[32m{:?}\x1b[0m at line {} column {}",
                    other, span.line, span.column,
                )));
                unreachable!()
            }
            None => panic!("Unexpected Eof"),
        };

        self.to_next_token();
        token
    }
}
