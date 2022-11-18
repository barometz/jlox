use std::collections::HashMap;

use lazy_static::lazy_static;
use thiserror::Error;

use crate::token::{Literal, Token, TokenType};

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenType> = HashMap::from([
        ("and", TokenType::And),
        ("class", TokenType::Class),
        ("else", TokenType::Else),
        ("false", TokenType::False),
        ("for", TokenType::For),
        ("fun", TokenType::Fun),
        ("if", TokenType::If),
        ("nil", TokenType::Nil),
        ("or", TokenType::Or),
        ("print", TokenType::Print),
        ("return", TokenType::Return),
        ("super", TokenType::Super),
        ("and", TokenType::And),
        ("this", TokenType::This),
        ("true", TokenType::True),
        ("var", TokenType::Var),
        ("while", TokenType::While),
    ]);
}

#[derive(Clone, Error, Debug)]
#[error("{line}: {message}")]
pub struct ScannerError {
    line: usize,
    message: String,
}

enum ScanResult {
    Skip,
    Error(ScannerError),
    Token(Token),
}

pub struct Scanner<'source> {
    /// View of the source that remains to be scanned
    source: &'source str,
    tokens: Vec<Token>,

    /// Current character in the lexeme being scanned
    current: usize,
    /// Line number of the current lexeme
    line: usize,
}

impl<'source> Scanner<'source> {
    pub fn new(source: &'source str) -> Self {
        Scanner {
            source,
            tokens: Vec::<Token>::new(),
            current: 0,
            line: 1,
        }
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<Token>, Vec<ScannerError>> {
        let mut errors = Vec::<ScannerError>::new();

        while !self.is_at_end() {
            self.source = &self.source[self.current..];
            self.current = 0;
            match self.scan_token() {
                ScanResult::Skip => continue,
                ScanResult::Error(error) => errors.push(error),
                ScanResult::Token(token) => self.tokens.push(token),
            }
        }

        self.tokens.push(self.new_token(TokenType::Eof));

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

    fn block_comment(&mut self) -> Result<(), ScannerError> {
        let mut line = self.line;

        while let Some(c) = self.advance() {
            match c {
                '*' if self.match_next('/') => {
                    self.line = line;
                    return Ok(());
                }
                '\n' => line += 1,
                _ => continue,
            }
        }

        let result = ScannerError {
            line: self.line,
            message: "Unterminated block comment".to_owned(),
        };
        self.line = line;
        Err(result)
    }

    fn string(&mut self) -> Result<Token, ScannerError> {
        let mut line = self.line;

        // TODO: this can probably be ... more concise
        while let Some(c) = self.advance() {
            match c {
                '"' => {
                    let lexeme = self.lexeme();
                    let result = Ok(self
                        .new_literal_token(TokenType::String, lexeme[1..lexeme.len() - 1].into()));
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
            message: "Unterminated string".to_owned(),
        };
        self.line = line;
        Err(result)
    }

    fn number(&mut self) -> Token {
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
            Literal::Number(self.lexeme().parse().unwrap()),
        )
    }

    fn identifier(&mut self) -> Token {
        while let Some(c) = self.peek() {
            if c.is_alphanumeric() || c == '_' {
                self.advance();
            } else {
                break;
            }
        }

        match KEYWORDS.get(self.lexeme()) {
            Some(token_type) if token_type == &TokenType::True => {
                self.new_literal_token(*token_type, Literal::Bool(true))
            }
            Some(token_type) if token_type == &TokenType::False => {
                self.new_literal_token(*token_type, Literal::Bool(false))
            }
            Some(token_type) if token_type == &TokenType::Nil => {
                self.new_literal_token(*token_type, Literal::Nil())
            }
            Some(token_type) => self.new_token(*token_type),
            None => self.new_token(TokenType::Identifier),
        }
    }

    fn scan_token(&mut self) -> ScanResult {
        use ScanResult::{Error, Skip, Token};

        match self.advance() {
            None => Error(ScannerError {
                line: self.line,
                message: "Expected token".to_owned(),
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
            Some('?') => Token(self.new_token(TokenType::Interro)),
            Some(':') => Token(self.new_token(TokenType::Colon)),
            Some('!') if self.match_next('=') => Token(self.new_token(TokenType::BangEqual)),
            Some('=') if self.match_next('=') => Token(self.new_token(TokenType::EqualEqual)),
            Some('<') if self.match_next('=') => Token(self.new_token(TokenType::LessEqual)),
            Some('>') if self.match_next('=') => Token(self.new_token(TokenType::GreaterEqual)),
            Some('!') => Token(self.new_token(TokenType::Bang)),
            Some('=') => Token(self.new_token(TokenType::Equal)),
            Some('<') => Token(self.new_token(TokenType::Less)),
            Some('>') => Token(self.new_token(TokenType::Greater)),
            Some('/') if self.match_next('/') => {
                while self.peek() != Some('\n') && !self.is_at_end() {
                    self.advance();
                }
                Skip
            }
            Some('/') if self.match_next('*') => match self.block_comment() {
                Ok(_) => Skip,
                Err(error) => Error(error),
            },
            Some('/') => Token(self.new_token(TokenType::Slash)),
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
            Some(c) if c.is_alphabetic() || c == '_' => Token(self.identifier()),
            Some(c) => Error(ScannerError {
                line: self.line,
                message: format!("Unexpected character {}", c),
            }),
        }
    }

    fn lexeme(&self) -> &'source str {
        &self.source[..self.current]
    }

    fn new_token(&self, token_type: TokenType) -> Token {
        Token::new(token_type, self.lexeme(), self.line)
    }

    fn new_literal_token(&self, token_type: TokenType, literal: Literal) -> Token {
        Token::new_literal(token_type, self.lexeme(), literal, self.line)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn tokenize_singles() -> Result<(), Vec<ScannerError>> {
        let mut under_test = Scanner::new("(}-");
        let tokens = under_test.scan_tokens()?;
        assert!(tokens.contains(&Token::new(TokenType::LeftParen, "(", 1)));
        assert!(tokens.contains(&Token::new(TokenType::RightBrace, "}", 1)));
        assert!(tokens.contains(&Token::new(TokenType::Minus, "-", 1)));
        Ok(())
    }

    #[test]
    fn tokenize_unknown_char() {
        let mut under_test = Scanner::new("%(}-+&+");
        let tokens = under_test.scan_tokens();
        assert!(tokens.is_err());
        let errors = tokens.unwrap_err();
        assert_eq!(errors[0].message, "Unexpected character %");
        assert_eq!(errors[1].message, "Unexpected character &");
    }

    #[test]
    fn tokenize_two_char_ops() -> Result<(), Vec<ScannerError>> {
        let mut under_test = Scanner::new("!!=+");
        let tokens = under_test.scan_tokens()?;
        assert!(tokens.contains(&Token::new(TokenType::Bang, "!", 1)));
        assert!(tokens.contains(&Token::new(TokenType::BangEqual, "!=", 1)));
        assert!(tokens.contains(&Token::new(TokenType::Plus, "+", 1)));
        Ok(())
    }

    #[test]
    fn tokenize_comment_whitespace() -> Result<(), Vec<ScannerError>> {
        let mut under_test = Scanner::new("+// testing\n=");
        let tokens = under_test.scan_tokens()?;
        assert!(tokens.contains(&Token::new(TokenType::Plus, "+", 1)));
        assert!(tokens.contains(&Token::new(TokenType::Equal, "=", 2)));
        Ok(())
    }

    #[test]
    fn tokenize_block_comment() -> Result<(), Vec<ScannerError>> {
        let mut under_test = Scanner::new(
            r#"+ /* comment
            more /*comment* */
            -"#,
        );
        let tokens = under_test.scan_tokens()?;
        assert!(tokens.contains(&Token::new(TokenType::Plus, "+", 1)));
        assert!(tokens.contains(&Token::new(TokenType::Minus, "-", 3)));
        Ok(())
    }

    #[test]
    fn tokenize_multiline_string() -> Result<(), Vec<ScannerError>> {
        let mut under_test = Scanner::new(
            r#""multiline
+ tokens"+"#,
        );
        let tokens = under_test.scan_tokens()?;
        assert!(tokens.contains(&Token::new_literal(
            TokenType::String,
            "\"multiline\n+ tokens\"",
            "multiline\n+ tokens".into(),
            1
        )));
        assert!(tokens.contains(&Token::new(TokenType::Plus, "+", 2)));
        Ok(())
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

    #[test]
    fn tokenize_identifiers() -> Result<(), Vec<ScannerError>> {
        let mut under_test = Scanner::new("for class variable_name1");
        let tokens = under_test.scan_tokens()?;
        assert!(tokens.contains(&Token::new(TokenType::For, "for", 1)));
        assert!(tokens.contains(&Token::new(TokenType::Class, "class", 1)));
        assert!(tokens.contains(&Token::new(TokenType::Identifier, "variable_name1", 1)));
        Ok(())
    }
}
