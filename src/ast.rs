use crate::lexer::TokenKind;

#[derive(Debug, Clone)]
pub enum LiteralKind {
    Integer(isize),
    Decimal(f64),
    QuotedString(String),
    Boolean(bool),
    None,
}

#[derive(Debug)]
pub enum Expression {
    Binary(BinaryExpr),
    Grouping(Box<Expression>),
    Unary(UnaryExpr),
    Literal(LiteralKind),
}

#[derive(Debug)]
pub struct BinaryExpr {
    pub operator: TokenKind,
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

#[derive(Debug)]
pub struct UnaryExpr {
    pub operator: TokenKind,
    pub rhs: Box<Expression>,
}
