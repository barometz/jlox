use thiserror::Error;

use crate::token::{Literal, Token, TokenType};

#[derive(Clone, Error, Debug)]
#[error("{line}: {message}")]
pub struct ScannerError {
    line: usize,
    message: String,
}

enum ScanResult<'source> {
    Skip,
    Error(ScannerError),
    Token(Token<'source>),
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
    pub fn new(source: &'source str) -> Self {
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
            match self.scan_token() {
                ScanResult::Skip => continue,
                ScanResult::Error(error) => errors.push(error),
                ScanResult::Token(token) => self.tokens.push(token),
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

    /// View the next character
    fn peek(&self) -> Option<char> {
        self.peek_n(0)
    }

    fn peek_n(&self, n: usize) -> Option<char> {
        self.source.chars().nth(self.current + n)
    }

    /// Consume the next character iff it matches expected
    fn match_next(&mut self, expected: char) -> bool {
        if Some(expected) == self.source.chars().nth(self.current) {
            self.current += 1;
            return true;
        }
        false
    }

    fn string(&mut self) -> Result<Token<'source>, ScannerError> {
        let mut line = self.line;

        // TODO: this can probably be ... more concise
        while let Some(c) = self.advance() {
            match c {
                '"' => {
                    let result = Ok(self.new_literal_token(
                        TokenType::String,
                        self.source[self.start + 1..self.current - 1].into(),
                    ));
                    self.line = line;
                    return result;
                }
                '\n' => {
                    line += 1;
                }
                _ => continue,
            }
        }

        let result = ScannerError {
            line: self.line,
            message: "Unterminated string".into(),
        };
        self.line = line;
        Err(result)
    }

    fn number(&mut self) -> Token<'source> {
        while let Some(c) = self.peek() {
            if c.is_ascii_digit() {
                self.advance();
            } else {
                break;
            }
        }

        if self.peek() == Some('.') {
            if let Some(c) = self.peek_n(1) {
                if c.is_ascii_digit() {
                    self.advance();
                }

                while let Some(c) = self.peek() {
                    if c.is_ascii_digit() {
                        self.advance();
                    } else {
                        break;
                    }
                }
            }
        }

        self.new_literal_token(
            TokenType::Number,
            Literal::Number(self.source[self.start..self.current].parse().unwrap()),
        )
    }

    fn scan_token(&mut self) -> ScanResult<'source> {
        use ScanResult::{Error, Skip, Token};

        match self.advance() {
            None => Error(ScannerError {
                line: self.line,
                message: "Expected token".into(),
            }),
            Some('(') => Token(self.new_token(TokenType::LeftParen)),
            Some(')') => Token(self.new_token(TokenType::RightParen)),
            Some('{') => Token(self.new_token(TokenType::LeftBrace)),
            Some('}') => Token(self.new_token(TokenType::RightBrace)),
            Some(',') => Token(self.new_token(TokenType::Comma)),
            Some('.') => Token(self.new_token(TokenType::Dot)),
            Some('-') => Token(self.new_token(TokenType::Minus)),
            Some('+') => Token(self.new_token(TokenType::Plus)),
            Some(';') => Token(self.new_token(TokenType::Semicolon)),
            Some('*') => Token(self.new_token(TokenType::Star)),
            Some('!') if self.match_next('=') => Token(self.new_token(TokenType::BangEqual)),
            Some('=') if self.match_next('=') => Token(self.new_token(TokenType::EqualEqual)),
            Some('<') if self.match_next('=') => Token(self.new_token(TokenType::LessEqual)),
            Some('>') if self.match_next('=') => Token(self.new_token(TokenType::GreaterEqual)),
            Some('!') => Token(self.new_token(TokenType::Bang)),
            Some('=') => Token(self.new_token(TokenType::Equal)),
            Some('<') => Token(self.new_token(TokenType::Less)),
            Some('>') => Token(self.new_token(TokenType::Greater)),
            Some('/') => {
                if self.match_next('/') {
                    while self.peek() != Some('\n') && !self.is_at_end() {
                        self.advance();
                    }
                    Skip
                } else {
                    Token(self.new_token(TokenType::Slash))
                }
            }
            Some(' ') => Skip,
            Some('\t') => Skip,
            Some('\r') => Skip,
            Some('\n') => {
                self.line += 1;
                Skip
            }
            Some('"') => match self.string() {
                Ok(token) => Token(token),
                Err(error) => Error(error),
            },
            Some(c) if c.is_ascii_digit() => Token(self.number()),
            Some(c) => Error(ScannerError {
                line: self.line,
                message: format!("Unexpected token {}", c),
            }),
        }
    }

    fn new_token(&self, token_type: TokenType) -> Token<'source> {
        Token::new(
            token_type,
            &self.source[self.start..self.current],
            self.line,
        )
    }

    fn new_literal_token(&self, token_type: TokenType, literal: Literal) -> Token<'source> {
        Token::new_literal(
            token_type,
            &self.source[self.start..self.current],
            literal,
            self.line,
        )
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
        assert!(tokens.contains(&Token::new(TokenType::LeftParen, "(", 1)));
        assert!(tokens.contains(&Token::new(TokenType::RightBrace, "}", 1)));
        assert!(tokens.contains(&Token::new(TokenType::Minus, "-", 1)));
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
        assert!(tokens.contains(&Token::new(TokenType::Bang, "!", 1)));
        assert!(tokens.contains(&Token::new(TokenType::BangEqual, "!=", 1)));
        assert!(tokens.contains(&Token::new(TokenType::Plus, "+", 1)));
    }

    #[test]
    fn tokenize_comment_whitespace() {
        let mut under_test = Scanner::new("+// testing\n=");
        let tokens = under_test.scan_tokens();
        assert!(tokens.is_ok());
        let tokens = tokens.unwrap();
        assert!(tokens.contains(&Token::new(TokenType::Plus, "+", 1)));
        assert!(tokens.contains(&Token::new(TokenType::Equal, "=", 2)));
    }

    #[test]
    fn tokenize_multiline_string() {
        let mut under_test = Scanner::new(
            r#""multiline
+ tokens"+"#,
        );
        let tokens = under_test.scan_tokens();
        assert!(tokens.is_ok());
        let tokens = tokens.unwrap();
        assert!(tokens.contains(&Token::new_literal(
            TokenType::String,
            "\"multiline\n+ tokens\"",
            "multiline\n+ tokens".into(),
            1
        )));
        assert!(tokens.contains(&Token::new(TokenType::Plus, "+", 2)));
    }

    #[test]
    fn tokenize_numbers() {
        // A more generic "this source will result in this sequence of tokens"
        // test would be nice, ideally without obscuring the token being tested
        let test = |input, literal| {
            let mut under_test = Scanner::new(input);
            let token = under_test.scan_token();
            assert!(matches!(token, ScanResult::Token(_)));
            if let ScanResult::Token(token) = token {
                assert_eq!(token.token_type, TokenType::Number);
                assert!(matches!(token.literal, Some(Literal::Number(l)) if l == literal));
            }
        };

        test("5", 5.0);
        test("142.006 ", 142.006);
        test("2.0", 2.0);
        test("0000", 0.0);
        test("0.6+", 0.6);
    }
}
