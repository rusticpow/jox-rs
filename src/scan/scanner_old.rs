use std::{any::Any, ops::Deref, ptr::null};

use super::token::{Token, TokenType};
// сделать очевидным места изменения и чтения
#[derive(Debug)]
pub struct ScannerOld<'a> {
    source: &'a str,
    start: usize,
    current: usize,
    line: i32,
}

impl<'a> ScannerOld<'a> {
    pub fn new(input: &'a str) -> ScannerOld<'a> {
        ScannerOld {
            source: todo!(),
            start: todo!(),
            current: todo!(),
            line: todo!(),
        }
    }

    pub(crate) fn scan_tokens(&'a self) -> Result<Vec<Token>, &str> {
        while !self.is_at_end() {
            self.scan_token();
        }

        let mut tokens: Vec<Token> = Vec::new();

        let token: &'a TokenType = &TokenType::EOF;
        let line = self.line;
        {
            let mut mutSelf = self;
            tokens.push(Token {
                token_type: *token,
                lexeme: "",
                line: line,
                literal: None,
            });
        }

        Ok(vec![Token {
            token_type: todo!(),
            lexeme: todo!(),
            literal: todo!(),
            line: todo!(),
        }])
    }

    fn report() {}

    fn is_at_end(&'a self) -> bool {
        ((self.current as usize) >= self.source.chars().count())
    }

    fn add_token(&'a mut self, token_type: &'a TokenType) {
        self.add_token_with_literal(token_type, None)
    }

    fn add_token_with_literal(
        &'a mut self,
        tokens: &'a mut Vec<Token<'a>>,
        token_type: &'a TokenType,
        literal: Option<Box<dyn Any>>,
    ) {
        let text = &self.source[self.start..self.current];

        let line1 = self.line;

        tokens.push(Token {
            token_type: *token_type,
            lexeme: text,
            literal: literal,
            line: self.line,
        });
    }

    fn advance(&'a mut self) -> char {
        self.current += 1;
        return self.source.chars().nth(self.current as usize).unwrap();
    }

    fn scan_token(&'a self) -> Vec<TokenType> {
        let c = self.advance();

        let mut tokens: Vec<TokenType> = Vec::new();
        match c {
            '(' => tokens.push(TokenType::LeftParen),
            // ')' => self.add_token(&TokenType::RightParen),
            // '{' => self.add_token(&TokenType::LeftBrace),
            // '}' => self.add_token(&TokenType::RightBrace),
            // ',' => self.add_token(&TokenType::Comma),
            // '.' => self.add_token(&TokenType::Dot),
            // '-' => self.add_token(&TokenType::Minus),
            // '+' => self.add_token(&TokenType::Plus),
            // ';' => self.add_token(&TokenType::Semicolon),
            // '*' => self.add_token(&TokenType::Star),
            //         '!' => self.add_token(if self.match_next('=') {
            //     &TokenType::BangEqual
            // } else {
            //     &TokenType::Bang
            // }),
            // '=' => self.add_token(if self.match_next('=') {
            //     &TokenType::EqualEqual
            // } else {
            //     &TokenType::Equal
            // }),
            // '<' => self.add_token(if self.match_next('=') {
            //     &TokenType::LessEqual
            // } else {
            //     &TokenType::Less
            // }),
            // '>' => self.add_token(if self.match_next('=') {
            //    &TokenType::GreaterEqual
            // } else {
            //    &TokenType::Greater
            // }),
            // '/' => {
            //     if self.match_next('/') {
            //         while *self.peek() != '\n' && !self.is_at_end() {
            //             self.advance();
            //         }
            //     }
            // }
            // ' ' | '\r' | '\t' => {}
            // '\n' => {
            //     *self.line += 1;
            // }
            // '"' => {
            //     self.string();
            // }
            _ => println!("Unexpected character: line: '{}'", (*self).line),
        }

        tokens
    }

    fn match_next(&'a mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }

        self.current += 1;
        return true;
    }

    fn peek(&'a mut self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        return self.source.chars().nth(self.current).unwrap();
    }

    fn string(&'a mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }

            self.advance();
        }

        if self.is_at_end() {
            println!("Unterminated string. {:?}", self.line);
            return;
        }

        self.advance();

        let value = &self.source[self.start + 1..self.current - 1];
        self.add_token_with_literal(&TokenType::String, Some(Box::new(value.to_owned())));
    }
}
