use std::{any::Any, ops::Deref, ptr::null};

use super::token::{Token, TokenType};

#[derive(Debug)]
pub struct Scanner<'a> {
    pub(crate) source: &'a str,
    tokens: &'a Vec<Token<'a>>,
    start: &'a usize,
    current: &'a mut usize,
    line: &'a i32,
}

impl Scanner<'_> {
    fn scan_tokens(&self) -> Result<Vec<Token>, &str> {
        while !self.is_at_end() {
            scan_token();
        }

        self.tokens.push(Token {
            token_type: &TokenType::EOF,
            lexeme: "",
            line: self.line,
            literal: None,
        });

        Ok(vec![Token {
            token_type: todo!(),
            lexeme: todo!(),
            literal: todo!(),
            line: todo!(),
        }])
    }

    fn report() {}

    fn is_at_end(&self) -> bool {
        (*self.current as usize) >= self.source.chars().count()
    }

    fn add_token(&self, token_type: TokenType) {
        self.add_token_with_literal(token_type, None)
    }

    fn add_token_with_literal(&self, token_type: TokenType, literal: Option<Box<dyn Any>>) {
        let text = &self.source[*self.start..*self.current];

        self.tokens.push(Token {
            token_type: &token_type,
            lexeme: text,
            literal: literal,
            line: self.line,
        });
    }

    fn advance(&self) -> char {
        *self.current = (*self.current) + 1;
        return self.source.chars().nth((*self.current) as usize).unwrap();
    }

    fn scan_token(&self) {
        let c = self.advance();
        match c {
            '(' => self.add_token(TokenType::LeftParen),
            ')' => self.add_token(TokenType::RightParen),
            '{' => self.add_token(TokenType::LeftBrace),
            '}' => self.add_token(TokenType::RightBrace),
            ',' => self.add_token(TokenType::Comma),
            '.' => self.add_token(TokenType::Dot),
            '-' => self.add_token(TokenType::Minus),
            '+' => self.add_token(TokenType::Plus),
            ';' => self.add_token(TokenType::Semicolon),
            '*' => self.add_token(TokenType::Star),
        }
    }
}
