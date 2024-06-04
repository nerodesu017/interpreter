use crate::expr::*;
pub trait Visitor<R> {
    fn visit_variable_expr(&mut self, expr: &variable_expr::VariableExpr) -> R;
    fn visit_this_expr(&mut self, expr: &this_expr::ThisExpr) -> R;
    fn visit_super_expr(&mut self, expr: &super_expr::SuperExpr) -> R;
    fn visit_set_expr(&mut self, expr: &set_expr::SetExpr) -> R;
    fn visit_logical_expr(&mut self, expr: &logical_expr::LogicalExpr) -> R;
    fn visit_get_expr(&mut self, expr: &get_expr::GetExpr) -> R;
    fn visit_call_expr(&mut self, expr: &call_expr::CallExpr) -> R;
    fn visit_assign_expr(&mut self, expr: &assign_expr::AssignExpr) -> R;
    fn visit_unary_expr(&mut self, expr: &unary_expr::UnaryExpr) -> R;
    fn visit_literal_expr(&mut self, expr: &literal_expr::LiteralExpr) -> R;
    fn visit_grouping_expr(&mut self, expr: &grouping_expr::GroupingExpr) -> R;
    fn visit_binary_expr(&mut self, expr: &binary_expr::BinaryExpr) -> R;
}

pub struct AstPrinter {

}

impl AstPrinter {
    pub fn print(&mut self, expr: Expr) -> String {
        match expr {
            Expr::Binary(expr) => self.parenthesize(expr.operator.lexeme.clone(), vec![*expr.left, *expr.right]),
            Expr::Grouping(expr) => self.parenthesize("group".to_owned(), vec![*expr.expression]),
            Expr::Literal(expr) => if expr.value.is_none() {
                "nil".to_owned()   
            } else {
                expr.value.unwrap().to_string()
            },
            Expr::Unary(expr) => self.parenthesize(expr.operator.lexeme, vec![*expr.right]),
            Expr::Assign(_) => todo!(),
            Expr::Call(_) => todo!(),
            Expr::Get(_) => todo!(),
            Expr::Logical(_) => todo!(),
            Expr::Set(_) => todo!(),
            Expr::Super(_) => todo!(),
            Expr::This(_) => todo!(),
            Expr::Variable(_) => todo!(),
        }
    }

    fn parenthesize(&mut self, name: String, exprs: Vec<Expr>) -> String {
        let mut builder = String::new();

        builder.push('(');
        builder.push_str(name.as_str());

        for expr in exprs {
            builder.push(' ');
            builder.push_str(self.print(expr).as_str());
        }

        builder.push(')');

        builder
    }
}

impl Visitor<String> for AstPrinter {
    fn visit_variable_expr(&mut self, expr: &variable_expr::VariableExpr) -> String {
        todo!()
    }

    fn visit_this_expr(&mut self, expr: &this_expr::ThisExpr) -> String {
        todo!()
    }

    fn visit_super_expr(&mut self, expr: &super_expr::SuperExpr) -> String {
        todo!()
    }

    fn visit_set_expr(&mut self, expr: &set_expr::SetExpr) -> String {
        todo!()
    }

    fn visit_logical_expr(&mut self, expr: &logical_expr::LogicalExpr) -> String {
        todo!()
    }

    fn visit_get_expr(&mut self, expr: &get_expr::GetExpr) -> String {
        todo!()
    }

    fn visit_call_expr(&mut self, expr: &call_expr::CallExpr) -> String {
        todo!()
    }

    fn visit_assign_expr(&mut self, expr: &assign_expr::AssignExpr) -> String {
        todo!()
    }

    fn visit_unary_expr(&mut self, expr: &unary_expr::UnaryExpr) -> String {
        todo!()
    }

    fn visit_literal_expr(&mut self, expr: &literal_expr::LiteralExpr) -> String {
        todo!()
    }

    fn visit_grouping_expr(&mut self, expr: &grouping_expr::GroupingExpr) -> String {
        todo!()
    }

    fn visit_binary_expr(&mut self, expr: &binary_expr::BinaryExpr) -> String {
        todo!()
    }
}