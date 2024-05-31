use crate::{literal::Literal, token::{Token, TokenType}};

pub struct Scanner {
  source: String,
  tokens: Vec<Token>,

  start: usize,
  current: usize,
  line: usize
}

impl Scanner {
  pub fn new(source: String) -> Scanner {
    Scanner {
      source,
      tokens: vec![],

      current: 0,
      start: 0,
      line: 1
    }
  }

  pub fn scan_tokens(&mut self) -> Vec<Token> {
    while !self.is_at_end() {
      self.start = self.current;
      self.scan_token();
    }

    self.tokens.push(Token::new(TokenType::Eof, String::from(""), None, self.line));
    self.tokens
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
      _ => unimplemented!()
    }
  }

  fn add_token(&mut self, ttype: TokenType, literal: Option<Literal>) {
    let text = self.source.as_str()[self.start..self.current].to_owned();
    self.tokens.push(Token::new(ttype, text, literal, self.line));
  }

  fn advance(&mut self) -> char {
    let c = self.source.chars().nth(self.current).unwrap_or_else(|| panic!("error when searching for char at index {} - string length: {}", self.current, self.source.len()));
    self.current += 1;
    c
  }

  fn is_at_end(&self) -> bool {
    self.current >= self.source.len()
  }
}