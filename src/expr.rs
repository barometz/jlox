use crate::token::{Literal, Token};
pub enum Expr<'s> {
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
impl<'s> Expr<'s> {
    pub fn accept<R>(&self, visitor: &mut dyn ExprVisitor<'s, R>) -> R {
        match self {
            Expr::Binary { lhs, operator, rhs } => visitor.visit_binary(lhs, operator, rhs),
            Expr::Grouping { expression } => visitor.visit_grouping(expression),
            Expr::Literal { value } => visitor.visit_literal(value),
            Expr::Unary { operator, operand } => visitor.visit_unary(operator, operand),
        }
    }
}
pub trait ExprVisitor<'s, R> {
    fn visit_binary(&mut self, lhs: &Expr<'s>, operator: &Token<'s>, rhs: &Expr<'s>) -> R;
    fn visit_grouping(&mut self, expression: &Expr<'s>) -> R;
    fn visit_literal(&mut self, value: &Literal) -> R;
    fn visit_unary(&mut self, operator: &Token<'s>, operand: &Expr<'s>) -> R;
}
