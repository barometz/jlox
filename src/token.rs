use std::fmt::Display;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TokenType {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semicolon,
    Slash,
    Star,

    // One or two character tokens
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    String(String),
    Number(f64),
}

impl From<&str> for Literal {
    fn from(string: &str) -> Self {
        Literal::String(string.into())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token<'source> {
    pub token_type: TokenType,
    pub lexeme: &'source str,
    pub literal: Option<Literal>,
    pub line: usize,
}

impl<'source> Token<'source> {
    pub fn new(token_type: TokenType, lexeme: &'source str, line: usize) -> Self {
        Token {
            token_type,
            lexeme,
            literal: None,
            line,
        }
    }

    pub fn new_literal(
        token_type: TokenType,
        lexeme: &'source str,
        literal: Literal,
        line: usize,
    ) -> Self {
        Token {
            token_type,
            lexeme,
            literal: Some(literal),
            line,
        }
    }
}

impl<'source> Display for Token<'source> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.literal {
            Some(literal) => write!(f, "{:?} {} {:?}", self.token_type, self.lexeme, literal),
            None => write!(f, "{:?} {}", self.token_type, self.lexeme),
        }
    }
}
