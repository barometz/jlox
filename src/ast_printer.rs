use crate::{
    expr::{Expr, ExprVisitor},
    token::{Literal, Token},
};

// TODO: add multiline pretty-printing
struct AstPrinter {}

impl AstPrinter {
    pub fn print(&mut self, expression: &Expr) -> String {
        expression.accept(self)
    }

    fn parenthesize(&mut self, name: &str, exprs: &[&Expr]) -> String {
        let mut result = String::new();

        result += &format!("({} ", name);
        result += &exprs
            .iter()
            .map(|e| e.accept(self))
            .collect::<Vec<String>>()
            .join(" ");
        result += ")";

        result
    }
}

impl ExprVisitor<String> for AstPrinter {
    fn visit_binary(&mut self, lhs: &Expr, operator: &Token, rhs: &Expr) -> String {
        self.parenthesize(&operator.lexeme, &[lhs, rhs])
    }

    fn visit_grouping(&mut self, expression: &Expr) -> String {
        self.parenthesize("group", &[expression])
    }

    fn visit_literal(&mut self, value: &Literal) -> String {
        // TODO: figure out what to do with nil, for which the book uses Java's null
        match value {
            Literal::String(s) => s.clone(),
            Literal::Number(n) => n.to_string(),
        }
    }

    fn visit_unary(&mut self, operator: &Token, operand: &Expr) -> String {
        self.parenthesize(&operator.lexeme, &[operand])
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::token::TokenType;

    #[test]
    fn print_an_expression() {
        // Boxing all the elements of the AST may have been a mistake; revisit that
        let expr = Expr::Binary {
            lhs: Box::new(Expr::Unary {
                operator: Box::new(Token::new(TokenType::Plus, "-", 0)),
                operand: Box::new(Expr::Literal {
                    value: Box::new(Literal::Number(123.0)),
                }),
            }),
            operator: Box::new(Token::new(TokenType::Plus, "*", 0)),
            rhs: Box::new(Expr::Grouping {
                expression: Box::new(Expr::Literal {
                    value: Box::new(Literal::Number(45.67)),
                }),
            }),
        };

        assert_eq!(AstPrinter {}.print(&expr), "(* (- 123) (group 45.67))");
    }
}
