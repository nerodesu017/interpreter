use crate::token::Token;
use super::Expr;
pub struct LogicalExpr {
    pub left: Box<Expr>,
    pub operator: Box<Token>,
    pub right: Box<Expr>,
}
