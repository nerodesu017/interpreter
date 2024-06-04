use crate::token::Token;
use super::Expr;
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: Box<Token>,
    pub right: Box<Expr>,
}
