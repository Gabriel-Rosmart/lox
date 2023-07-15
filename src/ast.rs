use crate::lexer::TokenKind;

pub trait Expression {}

#[allow(unused)]
pub struct Binary {
    operator: TokenKind,
    left: Box<dyn Expression>,
    right: Box<dyn Expression>,
}

#[allow(unused)]
impl Binary {
    pub fn new(operator: TokenKind, left: Box<dyn Expression>, right: Box<dyn Expression>) -> Self {
        Self {
            operator,
            left,
            right,
        }
    }
}

#[allow(unused)]
pub struct Grouping {
    expression: Box<dyn Expression>,
}

#[allow(unused)]
impl Grouping {
    pub fn new(expression: Box<dyn Expression>) -> Self {
        Self { expression }
    }
}

#[allow(unused)]
pub struct Literal<T> {
    value: T,
}

#[allow(unused)]
impl<T> Literal<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }
}

#[allow(unused)]
pub struct Unary {
    operator: TokenKind,
    right: Box<dyn Expression>,
}

#[allow(unused)]
impl Unary {
    pub fn new(operator: TokenKind, right: Box<dyn Expression>) -> Self {
        Self { operator, right }
    }
}

impl Expression for Binary {}
impl Expression for Grouping {}
impl<T> Expression for Literal<T> {}
impl Expression for Unary {}
