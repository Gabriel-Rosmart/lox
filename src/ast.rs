use crate::lexer::TokenKind;
use std::any::{Any, TypeId};

pub trait Expression: std::fmt::Debug + Any {}

#[allow(unused)]
#[derive(Debug)]
pub struct Binary {
    pub operator: TokenKind,
    pub left: Box<dyn Expression>,
    pub right: Box<dyn Expression>,
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
#[derive(Debug)]
pub struct Grouping {
    pub expression: Box<dyn Expression>,
}

#[allow(unused)]
impl Grouping {
    pub fn new(expression: Box<dyn Expression>) -> Self {
        Self { expression }
    }
}

#[allow(unused)]
#[derive(Debug)]
pub struct Literal<T> {
    pub value: T,
}

#[allow(unused)]
impl<T> Literal<T> {
    pub fn new(value: T) -> Self {
        Self { value }
    }
}

#[allow(unused)]
#[derive(Debug)]
pub struct Unary {
    pub operator: TokenKind,
    pub right: Box<dyn Expression>,
}

#[allow(unused)]
impl Unary {
    pub fn new(operator: TokenKind, right: Box<dyn Expression>) -> Self {
        Self { operator, right }
    }
}

impl Expression for Binary {}
impl Expression for Grouping {}
impl<T: std::fmt::Debug + Any> Expression for Literal<T> {}
impl Expression for Unary {}

trait InstanceOf
where
    Self: Any,
{
    fn instance_of<U: ?Sized + Any>(&self) -> bool {
        TypeId::of::<Self>() == TypeId::of::<U>()
    }
}

// implement this trait for every type that implements `Any` (which is most types)
impl<T: ?Sized + Any> InstanceOf for T {}
