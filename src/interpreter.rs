#[allow(unused)]
use crate::ast::{Expression, LiteralKind};
use crate::{
    ast::{BinaryExpr, UnaryExpr},
    lexer::TokenKind,
};

#[allow(unused)]
pub fn interpret(expression: Box<Expression>) -> LiteralKind {
    expression.eval()
}

macro_rules! numeric_binary_op (
    ($op:tt, $lhs:ident, $rhs:ident) => (
        match ($lhs, $rhs) {
            (LiteralKind::Integer(ilhs), LiteralKind::Integer(irhs)) => {
                LiteralKind::Integer(ilhs $op irhs)
            },
            (LiteralKind::Integer(ilhs), LiteralKind::Decimal(drhs)) => {
                LiteralKind::Decimal(ilhs as f64 $op drhs)
            },
            (LiteralKind::Decimal(dlhs), LiteralKind::Integer(irhs)) => {
                LiteralKind::Decimal(dlhs $op irhs as f64)
            },
            (LiteralKind::Decimal(dlhs), LiteralKind::Decimal(drhs)) => {
                LiteralKind::Decimal(dlhs $op drhs)
            }
            _ => panic!("Binary expression not allowed between those two types"),
        }
    );
);

pub trait Eval {
    fn eval(&self) -> LiteralKind;
}

impl Eval for BinaryExpr {
    fn eval(&self) -> LiteralKind {
        let lhs = self.lhs.eval();
        let rhs = self.rhs.eval();

        match self.operator {
            TokenKind::Plus => numeric_binary_op!(+, lhs, rhs),
            TokenKind::Minus => numeric_binary_op!(-, lhs, rhs),
            TokenKind::Asterisk => numeric_binary_op!(*, lhs, rhs),
            TokenKind::ForwardSlash => numeric_binary_op!(/, lhs, rhs),
            _ => unreachable!("Binary expression should not contain operator"),
        }
    }
}

impl Eval for UnaryExpr {
    fn eval(&self) -> LiteralKind {
        let rhs = self.rhs.eval();

        match self.operator {
            TokenKind::Minus => match rhs {
                LiteralKind::Integer(i) => LiteralKind::Integer(-i),
                LiteralKind::Decimal(d) => LiteralKind::Decimal(-d),
                _ => panic!("Unary expression not allowed here"),
            },
            TokenKind::Bang => match rhs {
                LiteralKind::True => LiteralKind::False,
                LiteralKind::False => LiteralKind::True,
                _ => panic!("Unary expression [Bang] not allowed to this operand"),
            },
            _ => unreachable!("Unary should not contain operator"),
        }
    }
}

impl Eval for Expression {
    fn eval(&self) -> LiteralKind {
        match self {
            Self::Binary(expr) => expr.eval(),
            Self::Unary(expr) => expr.eval(),
            Self::Grouping(expr) => expr.eval(),
            Self::Literal(expr) => expr.clone(),
        }
    }
}
