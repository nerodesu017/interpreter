use crate::token::Token;
use super::Expr;
pub struct AssignExpr {
    pub name: Box<Token>,
    pub value: Box<Expr>,
}