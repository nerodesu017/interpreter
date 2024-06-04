use crate::token::Token;
use super::Expr;
pub struct UnaryExpr {
    pub operator: Box<Token>,
    pub right: Box<Expr>,
}
