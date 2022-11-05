use thiserror::Error;

use crate::token;

#[derive(Error, Debug)]
#[error("{line}: {message}")]
pub struct ScannerError {
    line: usize,
    message: String,
}

pub struct Scanner<'source> {
    source: &'source str,
    tokens: Vec<token::Token<'source>>,

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
            tokens: Vec::<token::Token<'source>>::new(),
            start: 0,
            current: 0,
            line: 1,
        }
    }

    pub fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    pub fn scan_tokens(&mut self) -> Result<Vec<token::Token<'source>>, ScannerError> {
        while !self.is_at_end() {
            self.start = self.current;
            let token = self.scan_token()?;
            self.tokens.push(token);
        }

        Ok(self.tokens.clone())
    }

    fn scan_token(&mut self) -> Result<token::Token<'source>, ScannerError> {
        Err(ScannerError { line: self.line, message: String::from("Not implemented") })
    }
}
