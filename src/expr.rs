pub mod binary_expr;
pub mod grouping_expr;
pub mod literal_expr;
pub mod unary_expr;
pub mod assign_expr;
pub mod call_expr;
pub mod get_expr;
pub mod logical_expr;
pub mod set_expr;
pub mod super_expr;
pub mod this_expr;
pub mod variable_expr;
pub enum Expr {
    Binary(binary_expr::BinaryExpr),
    Grouping(grouping_expr::GroupingExpr),
    Literal(literal_expr::LiteralExpr),
    Unary(unary_expr::UnaryExpr),
    Assign(assign_expr::AssignExpr),
    Call(call_expr::CallExpr),
    Get(get_expr::GetExpr),
    Logical(logical_expr::LogicalExpr),
    Set(set_expr::SetExpr),
    Super(super_expr::SuperExpr),
    This(this_expr::ThisExpr),
    Variable(variable_expr::VariableExpr),
}
