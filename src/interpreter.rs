#[allow(unused)]
use crate::ast::{Expression, LiteralKind};
use crate::{
    ast::{BinaryExpr, Statement, UnaryExpr},
    error::LoxError,
    lexer::TokenKind,
};

#[allow(unused)]
pub fn interpret(statements: Vec<Box<Statement>>) {
    for statement in statements {
        match *statement {
            Statement::Expr(expr) => {
                let _ = expr.eval();
            }
            Statement::Print(expr) => {
                let value = expr.eval();
                println!("{value}");
            }
        };
    }
}

macro_rules! numeric_binary_op (
    ($op:tt, $lhs:ident, $rhs:ident) => (
        match (&$lhs, &$rhs) {
            (LiteralKind::Integer(ilhs), LiteralKind::Integer(irhs)) => {
                LiteralKind::Integer(ilhs $op irhs)
            },
            (LiteralKind::Integer(ilhs), LiteralKind::Decimal(drhs)) => {
                LiteralKind::Decimal(*ilhs as f64 $op drhs)
            },
            (LiteralKind::Decimal(dlhs), LiteralKind::Integer(irhs)) => {
                LiteralKind::Decimal(dlhs $op *irhs as f64)
            },
            (LiteralKind::Decimal(dlhs), LiteralKind::Decimal(drhs)) => {
                LiteralKind::Decimal(dlhs $op drhs)
            }
            _ => {
                crate::error::die(LoxError::RuntimeError(
                    format!("Binary expression not allowed between those two types \x1b[34m{:?}\x1b[0m and \x1b[34m{:?}\x1b[0m", $lhs, $rhs))
                );
                unreachable!("")
            },
        }
    );
);

macro_rules! comparison_op (
    ($op:tt, $lhs:ident, $rhs:ident) => (
        match (&$lhs, &$rhs) {
            (LiteralKind::Integer(ilhs), LiteralKind::Integer(irhs)) => {
                LiteralKind::Boolean(ilhs $op irhs)
            },
            (LiteralKind::Integer(ilhs), LiteralKind::Decimal(drhs)) => {
                LiteralKind::Boolean((*ilhs as f64) $op *drhs)
            },
            (LiteralKind::Decimal(dlhs), LiteralKind::Integer(irhs)) => {
                LiteralKind::Boolean(dlhs $op &(*irhs as f64))
            },
            (LiteralKind::Decimal(dlhs), LiteralKind::Decimal(drhs)) => {
                LiteralKind::Boolean(dlhs $op drhs)
            },
            (LiteralKind::Boolean(blhs), LiteralKind::Boolean(brhs)) => {
                LiteralKind::Boolean(blhs $op brhs)
            }
            _ => {
                crate::error::die(LoxError::RuntimeError(
                    format!("Comparison expression not allowed between those two types \x1b[34m{:?}\x1b[0m and \x1b[34m{:?}\x1b[0m", $lhs, $rhs))
                );
                unreachable!()
            },
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

        match self.operator.kind {
            TokenKind::Plus => numeric_binary_op!(+, lhs, rhs),
            TokenKind::Minus => numeric_binary_op!(-, lhs, rhs),
            TokenKind::Asterisk => numeric_binary_op!(*, lhs, rhs),
            TokenKind::ForwardSlash => numeric_binary_op!(/, lhs, rhs),
            TokenKind::Percentage => numeric_binary_op!(%, lhs, rhs),
            TokenKind::GreaterThan => comparison_op!(>, lhs, rhs),
            TokenKind::GreaterEqual => comparison_op!(>=, lhs, rhs),
            TokenKind::LessThan => comparison_op!(<, lhs, rhs),
            TokenKind::LessEqual => comparison_op!(<=, lhs, rhs),
            TokenKind::Equal => match (&lhs, &rhs) {
                (LiteralKind::None, LiteralKind::None) => LiteralKind::Boolean(true),
                (LiteralKind::None, _) => LiteralKind::Boolean(false),
                (_, LiteralKind::None) => LiteralKind::Boolean(false),
                _ => comparison_op!(==, lhs, rhs),
            },
            TokenKind::NotEqual => match (&lhs, &rhs) {
                (LiteralKind::None, LiteralKind::None) => LiteralKind::Boolean(false),
                (LiteralKind::None, _) => LiteralKind::Boolean(true),
                (_, LiteralKind::None) => LiteralKind::Boolean(true),
                _ => comparison_op!(!=, lhs, rhs),
            },
            _ => {
                crate::error::die(LoxError::RuntimeError(format!(
                    "Binary expression should not contain operator {}",
                    self.operator.clone().kind
                )));
                unreachable!()
            }
        }
    }
}

impl Eval for UnaryExpr {
    fn eval(&self) -> LiteralKind {
        let rhs = self.rhs.eval();

        match self.operator.kind {
            TokenKind::Minus => match rhs {
                LiteralKind::Integer(i) => LiteralKind::Integer(-i),
                LiteralKind::Decimal(d) => LiteralKind::Decimal(-d),
                _ => {
                    crate::error::die(LoxError::RuntimeError(format!(
                        "Unary expression {} not allowed with operand \x1b[34m{:?}\x1b[0m at line {}",
                        self.operator.clone().kind,
                        rhs,
                        self.operator.span.line
                    )));
                    unreachable!()
                }
            },
            TokenKind::Bang => match rhs {
                LiteralKind::Boolean(b) => LiteralKind::Boolean(!b),
                LiteralKind::None => LiteralKind::Boolean(true),
                _ => {
                    crate::error::die(LoxError::RuntimeError(format!(
                        "Unary expression {} not allowed to this operand \x1b[34m{:?}\x1b[0m at line {}",
                        self.operator.clone().kind,
                        rhs,
                        self.operator.span.line
                    )));
                    unreachable!()
                }
            },
            _ => {
                crate::error::die(LoxError::RuntimeError(format!(
                    "Unary expression should not contain operator {}",
                    self.operator.clone().kind
                )));
                unreachable!()
            }
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
