use crate::{expr::*, object::*};

pub struct AstPrinter;

impl AstPrinter {
    pub fn new() -> AstPrinter {
        AstPrinter {}
    }
    pub fn string_value(&mut self, expr: &Expr) -> String {
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
            Object::String(value) => format!("\"{value}\""),
            Object::Number(value) => value.to_string(),
            Object::Bool(value) => value.to_string(),
            Object::Nil => String::from("nil"),
            Object::Function(_function) => todo!(),
            Object::NativeFunction(_native_function) => todo!(),
            Object::Class(lox_class) => todo!(),
            Object::ClassInstance(lox_instance) => todo!(),
        }
    }
    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> String {
        self.parenthesize(&expr.operator.lexeme, vec![&expr.right])
    }

    fn visit_ternary_expr(&mut self, expr: &TernaryExpr) -> String {
        format!(
            "({} ? {} : {})",
            expr.condition.accept(self),
            expr.then_branch.accept(self),
            expr.else_branch.accept(self)
        )
    }

    fn visit_variable_expr(&mut self, expr: &VariableExpr) -> String {
        todo!()
    }

    fn visit_assign_expr(&mut self, expr: &AssignExpr) -> String {
        todo!()
    }

    fn visit_logical_expr(&mut self, expr: &LogicalExpr) -> String {
        todo!()
    }

    fn visit_call_expr(&mut self, expr: &CallExpr) -> String {
        todo!()
    }

    fn visit_get_expr(&mut self, expr: &GetExpr) -> String {
        todo!()
    }

    fn visit_set_expr(&mut self, expr: &SetExpr) -> String {
        todo!()
    }

    fn visit_this_expr(&mut self, expr: &ThisExpr) -> String {
        todo!()
    }

    fn visit_super_expr(&mut self, expr: &SuperExpr) -> String {
        todo!()
    }
}

#[cfg(test)]
mod ast_printer_tests {
    use crate::token::*;

    use super::*;
    #[test]
    pub fn test_ast_print() {
        let binary_expr = BinaryExpr {
            left: Box::new(Expr::Unary(UnaryExpr {
                operator: Token {
                    token_type: TokenType::Minus,
                    lexeme: String::from("-"),
                    literal: Object::Nil,
                    line: 1,
                },
                right: Box::new(Expr::Literal(LiteralExpr {
                    value: Object::Number(123.0),
                    uid: 0,
                })),
                uid: 0,
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
                    uid: 0,
                })),
                uid: 0,
            })),
            uid: 0,
        };
        let binary_expr = binary_expr;
        let expression = Expr::Binary(binary_expr);

        let mut ast_printer = AstPrinter {};
        println!("{}", ast_printer.string_value(&expression))
    }
}
