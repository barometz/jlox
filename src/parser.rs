use crate::{
    expr::Expr,
    token::{Token, TokenType},
};

use std::result::Result;

/// A recursive descent parser that walks through the available tokens one at a
/// time, eventually producing an Expr or ParserError.
pub struct Parser<'tokens> {
    pub tokens: &'tokens [Token],
}

#[derive(thiserror::Error, Debug)]
#[error("{}: {:?}: {message}", token.line, token.token_type)]
pub struct ParserError {
    token: Token,
    message: String,
}

impl<'tokens> Parser<'tokens> {
    pub fn parse(&mut self) -> Result<Expr, ParserError> {
        self.expression()
    }

    /// Return the next token, if any
    fn advance(&mut self) -> Option<Token> {
        let result = self.tokens.first();
        self.tokens = &self.tokens[1..];
        result.cloned()
    }

    fn peek(&self) -> Option<Token> {
        self.tokens.first().cloned()
    }

    /// Return the next token iff it matches one of the provided token types.
    fn match_one_of(&mut self, token_types: &[TokenType]) -> Option<Token> {
        for token_type in token_types {
            if let Some(token) = self.tokens.first() {
                if token.token_type == *token_type {
                    return self.advance();
                }
            }
        }
        None
    }

    /// Return a token of the specified type or an error with the specified
    /// message.
    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token, ParserError> {
        match self.peek() {
            Some(token) if token.token_type == token_type => Ok(self.advance().unwrap()),
            Some(token) => match token.token_type {
                TokenType::Eof => Err(ParserError {
                    token,
                    message: format!("Unexpected end of file. {}", message),
                }),
                _ => {
                    let lexeme: String = token.lexeme.clone();
                    Err(ParserError {
                        token,
                        message: format!("Unexpected token '{}'. {}", lexeme, message),
                    })
                }
            },
            None => panic!("Unexpected end of token stream"),
        }
    }

    fn expression(&mut self) -> Result<Expr, ParserError> {
        // expression -> equality
        self.equality()
    }

    /// Reusable parsing step for rules shaped like
    /// head -> operand ( ( operator1 | operator2 ) operand )*
    fn binary(
        &mut self,
        operand: &dyn Fn(&mut Self) -> Result<Expr, ParserError>,
        operators: &[TokenType],
    ) -> Result<Expr, ParserError> {
        let mut expr = operand(self)?;
        while let Some(operator) = self.match_one_of(operators) {
            expr = Expr::new_binary(expr, operator, operand(self)?);
        }
        Ok(expr)
    }

    fn equality(&mut self) -> Result<Expr, ParserError> {
        // equality -> comparison ( ( "!=" | "==" ) comparison )*
        self.binary(
            &Self::comparison,
            &[TokenType::BangEqual, TokenType::EqualEqual],
        )
    }

    fn comparison(&mut self) -> Result<Expr, ParserError> {
        // term ( ( ">" | ">=" | "<" | "<=" ) term )*
        self.binary(
            &Self::term,
            &[
                TokenType::Greater,
                TokenType::GreaterEqual,
                TokenType::Less,
                TokenType::LessEqual,
            ],
        )
    }

    fn term(&mut self) -> Result<Expr, ParserError> {
        // factor ( ( "-" | "+" ) factor )*
        self.binary(&Self::factor, &[TokenType::Plus, TokenType::Minus])
    }

    fn factor(&mut self) -> Result<Expr, ParserError> {
        // unary ( ( "/" | "*" ) factor )*
        self.binary(&Self::unary, &[TokenType::Slash, TokenType::Star])
    }

    fn unary(&mut self) -> Result<Expr, ParserError> {
        // ( ( "!" | "-" ) unary ) | primary
        if let Some(operator) = self.match_one_of(&[TokenType::Bang, TokenType::Minus]) {
            Ok(Expr::new_unary(operator, self.unary()?))
        } else {
            self.primary()
        }
    }

    fn primary(&mut self) -> Result<Expr, ParserError> {
        // NUMBER | STRING | TRUE | FALSE | NIL | "(" expression ")"

        if let Some(primary) = self.match_one_of(&[
            TokenType::Number,
            TokenType::String,
            TokenType::True,
            TokenType::False,
            TokenType::Nil,
        ]) {
            Ok(Expr::new_literal(primary.literal.unwrap()))
        } else {
            self.consume(
                TokenType::LeftParen,
                "Expected one of Number, String, True, False, Nil, or (Expr)",
            )?;
            let expr = self.expression()?;
            self.consume(TokenType::RightParen, "Unterminated (Expr)")?;
            Ok(Expr::new_grouping(expr))
        }
    }
}

#[cfg(test)]
mod test {
    use crate::token::Literal;

    use super::*;
    #[test]
    fn parse_plus() {
        let tokens = [
            Token::new_literal(TokenType::True, "true", Literal::Bool(true), 0),
            Token::new(TokenType::Plus, "+", 1),
            Token::new_literal(TokenType::Number, "6.2", Literal::Number(6.2), 2),
            Token::new(TokenType::Eof, "", 3),
        ];
        let mut under_test = Parser { tokens: &tokens };

        assert_eq!(
            under_test.parse().unwrap(),
            Expr::new_binary(
                Expr::new_literal(Literal::Bool(true)),
                Token {
                    token_type: TokenType::Plus,
                    lexeme: "+".into(),
                    line: 1,
                    literal: None
                },
                Expr::new_literal(Literal::Number(6.2))
            )
        );
    }
}
