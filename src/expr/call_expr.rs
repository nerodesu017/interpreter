use crate::token::Token;
use super::Expr;
pub struct CallExpr {
    pub callee: Box<Expr>,
    pub paren: Box<Token>,
    pub arguments: Vec<Expr>,
}
