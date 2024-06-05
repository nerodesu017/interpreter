use crate::{
    error::{ParserError, ParserErrorTrait},
    expr::*,
    literal::Literal,
    lox::Lox,
    token::{self, Token, TokenType},
};

pub struct Parser<'a> {
    pub tokens: Vec<Token>,
    pub current: i64,
    lox: &'a mut Lox,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token>, lox: &'a mut Lox) -> Self {
        Parser {
            tokens,
            current: 0,
            lox,
        }
    }

    pub fn parse(&mut self) -> Result<Expr, ParserError> {
        self.expression()
    }

    fn expression(&mut self) -> Result<Expr, ParserError> {
        self.equality()
    }

    fn equality(&mut self) -> Result<Expr,ParserError> {
        let mut expr = match self.comparison() {
            Ok(val) => val,
            Err(e) => return Err(e),
        };;

        while self.mmatch(vec![
            token::TokenType::BangEqual,
            token::TokenType::EqualEqual,
        ]) {
            let operator = self.previous();
            let right = match self.comparison() {
                Ok(val) => val,
                Err(e) => return Err(e),
            };;
            expr = Expr::Binary(binary_expr::BinaryExpr {
                left: Box::new(expr),
                operator: Box::new(operator),
                right: Box::new(right),
            })
        }

        Ok(expr)
    }

    fn mmatch(&mut self, ttypes: Vec<TokenType>) -> bool {
        for token_type in ttypes {
            if self.check(token_type) {
                self.advance();
                return true;
            }
        }
        return false;
    }

    fn check(&self, ttype: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        matches!(self.peek().ttype, ttype)
    }

    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        return self.previous();
    }

    fn is_at_end(&self) -> bool {
        matches!(self.peek().ttype, TokenType::Eof)
    }

    fn peek(&self) -> Token {
        self.tokens[self.current as usize].clone()
    }

    fn previous(&self) -> Token {
        self.tokens[self.current as usize - 1].clone()
    }

    fn comparison(&mut self) -> Result<Expr, ParserError> {
        let mut expr = match self.term() {
            Ok(val) => val,
            Err(e) => return Err(e),
        };

        while self.mmatch(vec![
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator = self.previous();
            let right = match self.term() {
                Ok(val) => val,
                Err(e) => return Err(e),
            };

            expr = Expr::Binary(binary_expr::BinaryExpr {
                left: Box::new(expr),
                operator: Box::new(operator),
                right: Box::new(right),
            })
        }

        Ok(expr)
    }

    fn term(&mut self) -> Result<Expr, ParserError> {
        let mut expr = match self.factor() {
            Ok(val) => val,
            Err(e) => return Err(e),
        };

        while self.mmatch(vec![TokenType::Minus, TokenType::Plus]) {
            let operator = self.previous();
            let right = match self.factor() {
                Ok(val) => val,
                Err(e) => return Err(e),
            };
            
            expr = Expr::Binary(binary_expr::BinaryExpr {
                left: Box::new(expr),
                operator: Box::new(operator),
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn factor(&mut self) -> Result<Expr, ParserError> {
        let mut expr = match self.unary() {
            Ok(val) => val,
            Err(e) => return Err(e),
        };

        while self.mmatch(vec![TokenType::Star, TokenType::Slash]) {
            let operator = self.previous();
            let right = match self.unary() {
                Ok(val) => val,
                Err(e) => return Err(e),
            };;
            expr = Expr::Binary(binary_expr::BinaryExpr {
                left: Box::new(expr),
                operator: Box::new(operator),
                right: Box::new(right),
            });
        }

        Ok(expr)
    }

    fn unary(&mut self) -> Result<Expr, ParserError> {
        if self.mmatch(vec![TokenType::Bang, TokenType::Minus]) {
            let operator = self.previous();
            let right = match self.unary() {
                Ok(val) => val,
                Err(e) => return Err(e),
            };;
            return Ok(Expr::Unary(unary_expr::UnaryExpr {
                operator: Box::new(operator),
                right: Box::new(right),
            }));
        }

        self.primary()
    }

    fn primary(&mut self) -> Result<Expr, ParserError> {
        if self.mmatch(vec![TokenType::False]) {
            return Ok(Expr::Literal(literal_expr::LiteralExpr {
                value: Some(Literal::Boolean(false)),
            }));
        }
        if self.mmatch(vec![TokenType::True]) {
            return Ok(Expr::Literal(literal_expr::LiteralExpr {
                value: Some(Literal::Boolean(true)),
            }));
        }

        if self.mmatch(vec![TokenType::Nil]) {
            return Ok(Expr::Literal(literal_expr::LiteralExpr {
                value: Some(Literal::Nil),
            }));
        }

        if self.mmatch(vec![TokenType::Number, TokenType::String]) {
            return Ok(Expr::Literal(literal_expr::LiteralExpr {
                value: self.previous().literal,
            }));
        }

        if self.mmatch(vec![TokenType::LeftParen]) {
            let expr = match self.expression() {
                Ok(val) => val,
                Err(e) => return Err(e),
            };
            self.consume(
                TokenType::RightParen,
                "Expect ')' after expression.".to_owned(),
            );
            return Ok(Expr::Grouping(grouping_expr::GroupingExpr {
                expression: Box::new(expr),
            }));
        }

        Err(self.error(self.peek(), "Expect expression.".to_owned()))
    }

    fn consume(&mut self, ttype: TokenType, message: String) -> Result<Token, ParserError> {
        if self.check(ttype) {
            return Ok(self.advance());
        }

        Err(self.error(self.peek(), message))
    }

    fn error(&mut self, token: Token, message: String) -> ParserError {
        self.lox.parser_error(token.clone(), message.clone());
        ParserError {}
    }

    fn synchronisze(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if matches!(self.previous().ttype, TokenType::Semicolon) {
                return;
            }

            match self.peek().ttype {
                TokenType::Class
                | TokenType::Fun
                | TokenType::Var
                | TokenType::For
                | TokenType::If
                | TokenType::While
                | TokenType::Print
                | TokenType::Return => {
                    return;
                }
                _ => {}
            }
        }

        self.advance();
    }
}
