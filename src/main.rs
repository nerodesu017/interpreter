use expr::{
    binary_expr::BinaryExpr, grouping_expr::GroupingExpr, literal_expr::LiteralExpr,
    unary_expr::UnaryExpr, Expr,
};
use literal::Literal;
use token::{Token, TokenType};
use visitor::AstPrinter;

mod error;
mod expr;
mod literal;
mod lox;
mod scanner;
mod token;
mod visitor;
mod parser;

fn main() {
    let expression = Expr::Binary(BinaryExpr {
        left: Box::new(Expr::Unary(UnaryExpr {
            operator: Box::new(Token::new(TokenType::Minus, "-".to_owned(), None, 1)),
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Literal::Float(123.0)),
            })),
        })),
        operator: Box::new(Token::new(TokenType::Star, "*".to_owned(), None, 1)),
        right: Box::new(Expr::Grouping(GroupingExpr {
            expression: Box::new(Expr::Literal(LiteralExpr {
                value: Some(Literal::Float(45.67)),
            })),
        })),
    });

    let mut printer = AstPrinter {};
    println!("{}", printer.print(expression));

    let mut args = std::env::args().collect::<Vec<String>>();
    args.remove(0);

    let mut lox = lox::Lox { had_error: false };

    if args.len() > 1 {
        println!("Usage: jlox [script]");
        std::process::exit(64);
    } else if args.len() == 1 {
        lox.run_file(args[0].to_owned());
    } else {
        lox.run_prompt();
    }
}
