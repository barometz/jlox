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
    Interro,
    Colon,

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
    True,
    False,
    Nil,

    // Keywords
    And,
    Class,
    Else,
    Fun,
    For,
    If,
    Or,
    Print,
    Return,
    Super,
    This,
    Var,
    While,

    Eof,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Literal {
    String(String),
    Number(f64),
    Bool(bool),
    Nil(),
}

impl From<&str> for Literal {
    fn from(string: &str) -> Self {
        Literal::String(string.into())
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Token {
    pub token_type: TokenType,
    // Fun Fact™: In a previous iteration, `lexeme` was a &str slice of the
    // source text, but it turns out that's really annoying - everything
    // contains tokens or references to them, including errors which outlive
    // the source string.
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: usize,
}

impl Token {
    pub fn new(token_type: TokenType, lexeme: &str, line: usize) -> Self {
        Token {
            token_type,
            lexeme: lexeme.into(),
            literal: None,
            line,
        }
    }

    pub fn new_literal(token_type: TokenType, lexeme: &str, literal: Literal, line: usize) -> Self {
        Token {
            token_type,
            lexeme: lexeme.into(),
            literal: Some(literal),
            line,
        }
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.literal {
            Some(literal) => write!(f, "{:?} {} {:?}", self.token_type, self.lexeme, literal),
            None => write!(f, "{:?} {}", self.token_type, self.lexeme),
        }
    }
}
