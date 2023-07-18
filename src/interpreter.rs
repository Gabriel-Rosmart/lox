use crate::ast::{Expression, Literal};
use std::any::Any;

pub fn interpret(expression: Box<dyn Expression>) {
    // if expression.instance_of::<Literal<isize>>() {
    //     println!("{}", literal::<isize>(expression));
    // }
}

pub fn literal<T>(node: Box<Literal<T>>) -> T {
    node.value
}
