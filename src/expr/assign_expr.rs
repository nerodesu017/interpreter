use crate::{token::Token, visitor::Visitor};
use super::Expr;
pub struct AssignExpr {
    pub name: Box<Token>,
    pub value: Box<Expr>,
}