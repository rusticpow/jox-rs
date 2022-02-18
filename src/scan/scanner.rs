use std::any::Any;

use super::token::{Token, TokenType};

#[derive(Debug)]
pub struct Scanner<'a> {
    pub(crate) source: &'a str,
}

struct InnerScanner<'a> {
    source: &'a str,
    start: usize,
    current: usize,
    line: i32,
}

impl<'a> InnerScanner<'a> {
    pub fn get_tokens(&mut self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();

        while !self.is_at_end() {
            let token = self.get_next_token();

            match token {
                Some(result) => tokens.push(result),
                None => continue,
            }
        }

        tokens
    }

    fn is_at_end(&self) -> bool {
        (self.current as usize) >= self.source.chars().count()
    }

    fn get_next_token(&mut self) -> Option<Token> {
        let c = self.advance();

        let mut literal: Option<Box<dyn Any>> = None;
        let token_type = match c {
            '(' => TokenType::LeftParen,
            ')' => TokenType::RightParen,
            '}' => TokenType::RightBrace,
            ',' => TokenType::Comma,
            '.' => TokenType::Dot,
            '-' => TokenType::Minus,
            '+' => TokenType::Plus,
            ';' => TokenType::Semicolon,
            '*' => TokenType::Star,
            '!' => {
                if self.match_next('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                }
            }
            '=' => {
                if self.match_next('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                }
            }
            '<' => {
                if self.match_next('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                }
            }
            '>' => {
                if self.match_next('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                }
            }
            '/' => {
                if self.match_next('/') {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                }

                TokenType::None
            }
            ' ' | '\r' | '\t' => TokenType::None,
            '\n' => {
                self.line += 1;

                TokenType::None
            }
            '"' => {
                let result = self.string();
                match result {
                    Some(token) => {
                        literal = Some(token.1);

                        token.0
                    }
                    None => TokenType::None,
                }
            }
            // Reserved Words and Identifiers ...
            _ => {
                if self.is_digit(c) {
                    let number_result = self.number();
                    literal = Some(number_result.1);

                    number_result.0
                } else {
                    TokenType::None
                }
            }
        };

        if matches!(token_type, TokenType::None) {
            return None;
        }

        let lexeme = String::from(&self.source[self.start..self.current]);

        Some(Token {
            token_type,
            lexeme,
            literal: literal,
            line: self.line,
        })
    }

    fn is_digit(&mut self, c: char) -> bool {
        c >= '0' && c <= '9'
    }

    fn number(&mut self) -> (TokenType, Box<dyn Any>) {
        while self.is_digit(self.peek()) {
            self.advance();
        }
        // Look for a fractional part.

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            // Consume the "."
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        let double = &self.source[self.start..self.current];

        (
            TokenType::Number,
            Box::new(double.parse::<f32>().expect("Float parse failed")),
        )
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }

        let char = self
            .source
            .chars()
            .nth(self.current + 1)
            .expect("Char get not found");
        char
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        return self.source.chars().nth(self.current as usize).unwrap();
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }

        return self.source.chars().nth(self.current).unwrap();
    }

    fn match_next(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }

        self.current += 1;
        return true;
    }

    fn string(&mut self) -> Option<(TokenType, Box<dyn Any>)> {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }

            self.advance();
        }

        if self.is_at_end() {
            println!("Unterminated string. {:?}", self.line);

            return None;
        }

        self.advance();

        let value = &self.source[self.start + 1..self.current - 1];

        Some((TokenType::String, Box::new(value.to_owned())))
    }
}

impl<'a> Scanner<'a> {
    pub fn scan_tokens(&self) -> Result<Vec<Token>, String> {
        let mut inner_scanner = InnerScanner {
            source: self.source,
            start: 1,
            current: 1,
            line: 1,
        };

        let tokens = inner_scanner.get_tokens();

        Ok(tokens)
    }
}
