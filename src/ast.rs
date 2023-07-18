use crate::lexer::TokenKind;

#[derive(Debug)]
pub enum LiteralKind {
    Integer(isize),
    Decimal(f64),
    QuotedString(String),
    True,
    False,
    NULL,
}

#[derive(Debug)]
pub enum Expression {
    Binary {
        operator: TokenKind,
        lhs: Box<Expression>,
        rhs: Box<Expression>,
    },
    Grouping {
        expression: Box<Expression>,
    },
    Unary {
        operator: TokenKind,
        rhs: Box<Expression>,
    },
    Literal(LiteralKind),
}
