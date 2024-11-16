use crate::{expr::*, token::*};

pub struct AstPrinter;

impl AstPrinter {
    pub fn new() -> AstPrinter {
        AstPrinter {}
    }
    pub fn print(&mut self, expr: &Expr) -> String {
        expr.accept(self)
    }

    fn parenthesize(&mut self, name: &str, expressions: Vec<&Expr>) -> String {
        let mut builder = String::from("(");

        builder.push_str(name);
        for expr in expressions {
            builder.push(' ');
            builder.push_str(&expr.accept(self));
        }
        builder.push(')');

        builder
    }
}

impl ExprVisitor<String> for AstPrinter {
    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> String {
        self.parenthesize(&expr.operator.lexeme, vec![&expr.left, &expr.right])
    }
    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> String {
        self.parenthesize("group", vec![&expr.expression])
    }
    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> String {
        match &expr.value {
            Object::String(value) => value.to_string(),
            Object::Number(value) => value.to_string(),
            Object::Bool(value) => value.to_string(),
            Object::Nil => String::from("nil"),
        }
    }
    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> String {
        self.parenthesize(&expr.operator.lexeme, vec![&expr.right])
    }
}

#[test]
pub fn test_ast_print() {
    let expression = Expr::Binary(BinaryExpr {
        left: Box::new(Expr::Unary(UnaryExpr {
            operator: Token {
                token_type: TokenType::Minus,
                lexeme: String::from("-"),
                literal: Object::Nil,
                line: 1,
            },
            right: Box::new(Expr::Literal(LiteralExpr {
                value: Object::Number(123.0),
            })),
        })),
        operator: Token {
            token_type: TokenType::Star,
            lexeme: String::from("*"),
            literal: Object::Nil,
            line: 1,
        },
        right: Box::new(Expr::Grouping(GroupingExpr {
            expression: Box::new(Expr::Literal(LiteralExpr {
                value: Object::Number(45.67),
            })),
        })),
    });

    let mut ast_printer = AstPrinter {};
    println!("{}", ast_printer.print(&expression))
}
