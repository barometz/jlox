use crate::token::{Literal, Token};
enum Expr<'s> {
    Binary {
        lhs: Box<Expr<'s>>,
        operator: Box<Token<'s>>,
        rhs: Box<Expr<'s>>,
    },
    Grouping {
        expression: Box<Expr<'s>>,
    },
    Literal {
        value: Box<Literal>,
    },
    Unary {
        operator: Box<Token<'s>>,
        operand: Box<Expr<'s>>,
    },
}
