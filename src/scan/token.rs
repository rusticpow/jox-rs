use std::any::Any;

#[derive(Debug)]
pub struct Token<'a> {
    pub(crate) token_type: TokenType,
    pub(crate) lexeme: &'a str,
    pub(crate) literal: Option<Box<dyn Any>>,
    pub(crate) line: &'a i32,
}

impl<'a> Token<'a> {
    pub(crate) fn new(token :&'a Token) -> Token<'a> {
        Token {
            token_type: token.token_type,
            lexeme: token.lexeme,
            literal: token.literal,
            line: token.line,
        }
    }
}

#[derive(Debug)]
pub enum TokenType {
    // Single-character tokens.
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

    // One or two character tokens.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier,
    String,
    Number,

    // Keywords.
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

    EOF,
}
