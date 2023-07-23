use crate::lexer::Token;

#[derive(Debug, Clone)]
pub enum LiteralKind {
    Integer(isize),
    Decimal(f64),
    QuotedString(String),
    Boolean(bool),
    Identifier(String),
    None,
}

#[derive(Debug)]
pub enum Statement {
    Print(Box<Expression>),
    Assign(String, Box<Expression>),
    Let(String, Box<Expression>),
    Expr(Box<Expression>),
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
    pub operator: Token,
    pub lhs: Box<Expression>,
    pub rhs: Box<Expression>,
}

#[derive(Debug)]
pub struct UnaryExpr {
    pub operator: Token,
    pub rhs: Box<Expression>,
}

impl std::fmt::Display for LiteralKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &Self::Integer(i) => write!(f, "{}", i),
            &Self::Decimal(d) => write!(f, "{}", d),
            &Self::Boolean(b) => write!(f, "{}", b),
            &Self::QuotedString(ref s) => write!(f, "{}", s),
            &Self::Identifier(ref s) => write!(f, "{}", s),
            &Self::None => write!(f, "None"),
        }
    }
}
