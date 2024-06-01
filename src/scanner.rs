use std::collections::HashMap;

use crate::{
    error::ScannerError,
    literal::Literal,
    lox::Lox,
    token::{Token, TokenType},
};

pub struct Scanner<'a> {
    source: String,
    tokens: Vec<Token>,
    lox: &'a mut Lox,

    start: usize,
    current: usize,
    line: usize,

    keywords: HashMap<&'a str, TokenType>
}

impl Scanner<'_> {
    pub fn new(source: String, lox: &mut Lox) -> Scanner {
        let mut keywords = HashMap::new();
        keywords.insert("and", TokenType::And);
        keywords.insert("class",  TokenType::Class);
        keywords.insert("else",   TokenType::Else);
        keywords.insert("false",  TokenType::False);
        keywords.insert("for",    TokenType::For);
        keywords.insert("fun",    TokenType::Fun);
        keywords.insert("if",     TokenType::If);
        keywords.insert("nil",    TokenType::Nil);
        keywords.insert("or",     TokenType::Or);
        keywords.insert("print",  TokenType::Print);
        keywords.insert("return", TokenType::Return);
        keywords.insert("super",  TokenType::Super);
        keywords.insert("this",   TokenType::This);
        keywords.insert("true",   TokenType::True);
        keywords.insert("var",    TokenType::Var);
        keywords.insert("while",  TokenType::While);
        Scanner {
            source,
            tokens: vec![],
            lox,

            current: 0,
            start: 0,
            line: 1,

            keywords
        }
    }

    pub fn scan_tokens(&mut self) -> Vec<Token> {
        while !self.is_at_end() {
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token::new(
            TokenType::Eof,
            String::from(""),
            None,
            self.line,
        ));
        self.tokens.clone()
    }

    fn scan_token(&mut self) {
        let c = self.advance();

        match c {
            '(' => self.add_token(TokenType::LeftParen, None),
            ')' => self.add_token(TokenType::RightParen, None),
            '{' => self.add_token(TokenType::LeftBrace, None),
            '}' => self.add_token(TokenType::RightBrace, None),
            ',' => self.add_token(TokenType::Comma, None),
            '.' => self.add_token(TokenType::Dot, None),
            '-' => self.add_token(TokenType::Minus, None),
            '+' => self.add_token(TokenType::Plus, None),
            ';' => self.add_token(TokenType::Semicolon, None),
            '*' => self.add_token(TokenType::Star, None),
            '!' => {
                let token = if self.mmatch('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(token, None);
            }
            '=' => {
                let token = if self.mmatch('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };

                self.add_token(token, None);
            }
            '<' => {
                let token = if self.mmatch('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(token, None);
            }
            '>' => {
                let token = if self.mmatch('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };

                self.add_token(token, None);
            }

            '/' => {
                let is_comment = self.mmatch('/');

                if is_comment {
                    while self.peek() != '\n' && !self.is_at_end() {
                        self.advance();
                    }
                } else {
                    self.add_token(TokenType::Slash, None)
                }
            }
            ' ' | '\r' | '\t' => {}
            '\n' => {
                self.line += 1;
            }
            '"' => {
                self.string();
            }
            _ => {
                if self.is_digit(c) {
                    self.number();
                } else if self.is_alpha(c) {
                    self.identifier();
                } else {
                    Lox::error(self.lox, self.line, String::from("Unexpected character."));
                }
            }
        }
    }

    fn identifier(&mut self) {
      while self.is_alpha_numeric(self.peek()) {
        self.advance();
      }

      let text = self.source.as_str()[self.start..self.current].to_owned();
      let ttype = self.keywords.get(text.as_str());

      if ttype.is_none() {
        // it is NOT a keyword
        self.add_token(TokenType::Identifier, None);
      } else {
        // it is a keyword
        self.add_token(ttype.unwrap().clone(), None);
      }
    }

    fn is_alpha(&self, c: char) -> bool {
        (c >= 'a' && c <= 'z') || (c >= 'A' && c <= 'Z') || c == '_'
    }

    fn is_alpha_numeric(&self, c: char) -> bool {
      self.is_alpha(c) || self.is_digit(c)
    }

    fn number(&mut self) {
        while self.is_digit(self.peek()) {
            self.advance();
        }

        if self.peek() == '.' && self.is_digit(self.peek_next()) {
            self.advance();

            while self.is_digit(self.peek()) {
                self.advance();
            }
        }

        self.add_token(
            TokenType::Number,
            Some(Literal::Float(
                self.source.as_str()[self.start..self.current]
                    .parse::<f64>()
                    .unwrap(),
            )),
        );
    }

    fn peek_next(&self) -> char {
        if self.current + 1 >= self.source.len() {
            return '\0';
        }

        self.source.chars().nth(self.current + 1).unwrap()
    }

    fn is_digit(&self, c: char) -> bool {
        return c >= '0' && c <= '9';
    }

    fn string(&mut self) {
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
                self.advance();
            }
        }

        if self.is_at_end() {
            Lox::error(self.lox, self.line, String::from("Unterminated string."));
            return;
        }

        // We didn't yet consume the closing double quote
        self.advance();

        // Now our string is inside double quotes, so substring it!
        let value = self.source.as_str()[self.start + 1..self.current - 1].to_owned();

        self.add_token(TokenType::String, Some(Literal::SString(value)));
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        return self.source.chars().nth(self.current).unwrap();
    }

    fn mmatch(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }

        self.current += 1;
        return true;
    }

    fn add_token(&mut self, ttype: TokenType, literal: Option<Literal>) {
        let text = self.source.as_str()[self.start..self.current].to_owned();
        self.tokens
            .push(Token::new(ttype, text, literal, self.line));
    }

    fn advance(&mut self) -> char {
        let c = self.source.chars().nth(self.current).unwrap_or_else(|| {
            panic!(
                "error when searching for char at index {} - string length: {}",
                self.current,
                self.source.len()
            )
        });
        self.current += 1;
        c
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
