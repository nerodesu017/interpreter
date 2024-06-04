use crate::token::Token;
use super::Expr;
pub struct GetExpr {
    pub object: Box<Expr>,
    pub name: Box<Token>,
}
