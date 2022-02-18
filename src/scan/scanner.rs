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
                None => continue
            }
        }

        tokens
    }

    fn is_at_end(&self) -> bool {
        (self.current as usize) >= self.source.chars().count()
    }

    fn get_next_token(&mut self) -> Option<Token> {
        let c = self.advance();

        let token_type = match c {
            '(' => TokenType::LeftParen,
            _ => TokenType::None,
        };

        if matches!(token_type, TokenType::None) {
            return None
        }

        let lexeme = String::from(&self.source[self.start..self.current]);

        Some(Token {
            token_type,
            lexeme,
            literal: None,
            line: self.line,
        })
    }

    fn advance(&mut self) -> char {
        self.current += 1;
        return self.source.chars().nth(self.current as usize).unwrap();
    }
}

impl<'a> Scanner<'a> {
    pub fn scan_tokens(&self) -> Result<Vec<Token>, String> {
        let mut innerScanner = InnerScanner {
            source: self.source,
            start: 1,
            current: 1,
            line: 1,
        };

        let tokens = innerScanner.get_tokens();

        Ok(vec![Token {
            token_type: todo!(),
            lexeme: todo!(),
            literal: todo!(),
            line: todo!(),
        }])
    }
}
