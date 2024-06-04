use crate::token::Token;
use super::Expr;
pub struct SetExpr {
    pub object: Box<Expr>,
    pub name: Box<Token>,
    pub value: Box<Expr>,
}
