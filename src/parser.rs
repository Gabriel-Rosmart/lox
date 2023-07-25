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

    fn expect_semicolon(&mut self, span: Option<(usize, usize)>) {
        match self.peek().map(|t| &t.kind) {
            Some(&TokenKind::Semicolon) => {
                self.to_next_token();
            }
            _ => {
                if span.is_none() {
                    crate::error::die(crate::error::LoxError::ParseError(
                        "Expected semicolon at end of statement at end of file".to_string(),
                    ));
                } else {
                    let span = span.unwrap();
                    let (line, column) = span;
                    crate::error::die(crate::error::LoxError::ParseError(format!(
                    "Expected semicolon at end of statement at line \x1b[32m{line}\x1b[0m column \x1b[32m{column}\x1b[0m"
                )));
                }
            }
        }
    }

    pub fn parse(&mut self) -> Vec<Box<Statement>> {
        self.statement()
    }

    pub fn statement(&mut self) -> Vec<Box<Statement>> {
        let mut stmts: Vec<Box<Statement>> = Vec::new();

        loop {
            match self.peek().map(|t| &t.kind) {
                Some(&TokenKind::Print) => self.print_statement(&mut stmts),
                Some(&TokenKind::Let) => self.variable_declaration(&mut stmts),
                Some(_) => stmts.push(Box::new(Statement::Expr(self.expression()))),
                None => break,
            }
        }

        stmts
    }

    fn print_statement(&mut self, stmts: &mut Vec<Box<Statement>>) {
        self.to_next_token();
        let value = self.expression();

        let span: Option<(usize, usize)> = self.peek().map(|t| (t.span.line, t.span.column));

        self.expect_semicolon(span);
        stmts.push(Box::new(Statement::Print(value)));
    }

    fn variable_declaration(&mut self, stmts: &mut Vec<Box<Statement>>) {
        self.to_next_token();

        let varname = match self.peek().map(|t| &t.kind) {
            Some(&TokenKind::Identifier(ref ident)) => ident.clone(),
            _ => {
                crate::error::die(crate::error::LoxError::ParseError(
                    "Expected identifier".to_string(),
                ));
                unreachable!()
            }
        };

        self.to_next_token();

        let initializer = match self.peek().map(|t| &t.kind) {
            Some(&TokenKind::Assign) => {
                self.to_next_token();
                self.expression()
            }
            Some(&TokenKind::Semicolon) => {
                stmts.push(Box::new(Statement::Let(
                    varname,
                    Box::new(Expression::Literal(LiteralKind::None)),
                )));
                self.to_next_token();
                return;
            }
            _ => {
                crate::error::die(crate::error::LoxError::ParseError(
                    "Expected assign operator".to_string(),
                ));
                unreachable!()
            }
        };

        let span = self.peek().map(|t| (t.span.line, t.span.column));
        self.expect_semicolon(span);
        stmts.push(Box::new(Statement::Let(varname, initializer)));
    }

    pub fn expression(&mut self) -> Box<Expression> {
        self.assignment()
    }

    pub fn assignment(&mut self) -> Box<Expression> {
        let mut expr = self.equality();

        loop {
            match self.peek().map(|t| &t.kind) {
                Some(&TokenKind::Assign) => {
                    let varname = match *expr {
                        Expression::Literal(val) => match val {
                            LiteralKind::Identifier(ref s) => s.clone(),
                            _ => {
                                crate::error::die(crate::error::LoxError::ParseError(
                                    "Expected identifier".to_string(),
                                ));
                                unreachable!()
                            }
                        },
                        _ => {
                            crate::error::die(crate::error::LoxError::ParseError(
                                "Expected literal on assignment".to_string(),
                            ));
                            unreachable!()
                        }
                    };

                    self.to_next_token();
                    let value = self.assignment();
                    expr = Box::new(Expression::Assign(varname, value));
                    let span = self.peek().map(|t| (t.span.line, t.span.column));
                    self.expect_semicolon(span);
                }
                _ => break,
            }
        }

        expr
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
            Some(&TokenKind::Identifier(ref s)) => {
                Box::new(Expression::Literal(LiteralKind::Identifier(s.clone())))
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
