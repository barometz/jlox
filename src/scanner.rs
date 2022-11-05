use thiserror::Error;

use crate::token::{Token, TokenType};

#[derive(Clone, Error, Debug)]
#[error("{line}: {message}")]
pub struct ScannerError {
    line: usize,
    message: String,
}

pub struct Scanner<'source> {
    source: &'source str,
    tokens: Vec<Token<'source>>,

    /// First character in the lexeme being scanned
    start: usize,
    /// Current character in the lexeme being scanned
    current: usize,
    /// Line number of the current lexeme
    line: usize,
}

impl<'source> Scanner<'source> {
    pub fn new(source: &'source str) -> Scanner {
        Scanner {
            source,
            tokens: Vec::<Token<'source>>::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token<'source>>, Vec<ScannerError>> {
        let mut errors = Vec::<ScannerError>::new();

        while !self.is_at_end() {
            self.start = self.current;
            // bail immediately, or store error to produce multiple errors?
            match self.scan_token() {
                Ok(token) => self.tokens.push(token),
                Err(error) => errors.push(error),
            }
        }

        if errors.is_empty() {
            Ok(self.tokens.clone())
        } else {
            Err(errors)
        }
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&mut self) -> Option<char> {
        match self.source.chars().nth(self.current) {
            Some(c) => {
                self.current += 1;
                Some(c)
            }
            None => None,
        }
    }

    /// Consume the next character iff it matches expected
    fn match_next(&mut self, expected: char) -> bool {
        if let Some(c) = self.source.chars().nth(self.current) {
            if c == expected {
                self.current += 1;
                return true;
            }
        }
        false
    }

    fn scan_token(&mut self) -> Result<Token<'source>, ScannerError> {
        match self.advance() {
            None => Err(ScannerError {
                line: self.line,
                message: "Expected token".into(),
            }),
            // this feels like a lot of duplication. There's probably a better
            // way to structure this, maybe with a function that returns
            // Option<TokenType>?
            Some('(') => Ok(self.new_token(TokenType::LeftParen)),
            Some(')') => Ok(self.new_token(TokenType::RightParen)),
            Some('{') => Ok(self.new_token(TokenType::LeftBrace)),
            Some('}') => Ok(self.new_token(TokenType::RightBrace)),
            Some(',') => Ok(self.new_token(TokenType::Comma)),
            Some('.') => Ok(self.new_token(TokenType::Dot)),
            Some('-') => Ok(self.new_token(TokenType::Minus)),
            Some('+') => Ok(self.new_token(TokenType::Plus)),
            Some(';') => Ok(self.new_token(TokenType::Semicolon)),
            Some('*') => Ok(self.new_token(TokenType::Star)),
            Some('!') if self.match_next('=') => Ok(self.new_token(TokenType::BangEqual)),
            Some('=') if self.match_next('=') => Ok(self.new_token(TokenType::EqualEqual)),
            Some('<') if self.match_next('=') => Ok(self.new_token(TokenType::LessEqual)),
            Some('>') if self.match_next('=') => Ok(self.new_token(TokenType::GreaterEqual)),
            Some('!') => Ok(self.new_token(TokenType::Bang)),
            Some('=') => Ok(self.new_token(TokenType::Equal)),
            Some('<') => Ok(self.new_token(TokenType::Less)),
            Some('>') => Ok(self.new_token(TokenType::Greater)),
            Some(character) => Err(ScannerError {
                line: self.line,
                message: format!("Unexpected token {}", character),
            }),
        }
    }

    fn new_token(&self, token_type: TokenType) -> Token<'source> {
        Token {
            token_type,
            lexeme: &self.source[self.start..self.current],
            literal: None,
            line: self.line,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tokenize_singles() {
        let mut under_test = Scanner::new("(}-");
        let tokens = under_test.scan_tokens();
        assert!(tokens.is_ok());
        let tokens = tokens.unwrap();
        assert!(tokens.contains(&Token {
            token_type: TokenType::LeftParen,
            lexeme: "(",
            literal: None,
            line: 1
        }));
        assert!(tokens.contains(&Token {
            token_type: TokenType::RightBrace,
            lexeme: "}",
            literal: None,
            line: 1
        }));
        assert!(tokens.contains(&Token {
            token_type: TokenType::Minus,
            lexeme: "-",
            literal: None,
            line: 1
        }));
    }

    #[test]
    fn tokenize_unknown_char() {
        let mut under_test = Scanner::new("(}-%+_+");
        let tokens = under_test.scan_tokens();
        assert!(tokens.is_err());
        let errors = tokens.unwrap_err();
        assert_eq!(errors[0].message, "Unexpected token %");
        assert_eq!(errors[1].message, "Unexpected token _");
    }

    #[test]
    fn tokenize_two_char_ops() {
        let mut under_test = Scanner::new("!!=+");
        let tokens = under_test.scan_tokens();
        assert!(tokens.is_ok());
        let tokens = tokens.unwrap();
        assert!(tokens.contains(&Token {
            token_type: TokenType::Bang,
            lexeme: "!",
            literal: None,
            line: 1
        }));
        assert!(tokens.contains(&Token {
            token_type: TokenType::BangEqual,
            lexeme: "!=",
            literal: None,
            line: 1
        }));
        assert!(tokens.contains(&Token {
            token_type: TokenType::Plus,
            lexeme: "+",
            literal: None,
            line: 1
        }));
    }
}
