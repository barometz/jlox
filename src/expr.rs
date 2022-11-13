// generated by: cargo run --bin generate_ast src

use crate::token::{Literal, Token};

pub enum Expr {
    Binary {
        lhs: Box<Expr>,
        operator: Box<Token>,
        rhs: Box<Expr>,
    },
    Grouping {
        expression: Box<Expr>,
    },
    Literal {
        value: Box<Literal>,
    },
    Unary {
        operator: Box<Token>,
        operand: Box<Expr>,
    },
}
impl Expr {
    pub fn accept<R>(&self, visitor: &mut dyn ExprVisitor<R>) -> R {
        match self {
            Expr::Binary { lhs, operator, rhs } => visitor.visit_binary(lhs, operator, rhs),
            Expr::Grouping { expression } => visitor.visit_grouping(expression),
            Expr::Literal { value } => visitor.visit_literal(value),
            Expr::Unary { operator, operand } => visitor.visit_unary(operator, operand),
        }
    }
    pub fn new_binary(lhs: Expr, operator: Token, rhs: Expr) -> Expr {
        Expr::Binary { lhs: Box::new(lhs), operator: Box::new(operator), rhs: Box::new(rhs) }
    }
    pub fn new_grouping(expression: Expr) -> Expr {
        Expr::Grouping { expression: Box::new(expression) }
    }
    pub fn new_literal(value: Literal) -> Expr {
        Expr::Literal { value: Box::new(value) }
    }
    pub fn new_unary(operator: Token, operand: Expr) -> Expr {
        Expr::Unary { operator: Box::new(operator), operand: Box::new(operand) }
    }
}
pub trait ExprVisitor<R> {
    fn visit_binary(&mut self, lhs: &Expr, operator: &Token, rhs: &Expr) -> R;
    fn visit_grouping(&mut self, expression: &Expr) -> R;
    fn visit_literal(&mut self, value: &Literal) -> R;
    fn visit_unary(&mut self, operator: &Token, operand: &Expr) -> R;
}
