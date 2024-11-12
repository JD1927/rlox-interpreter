use crate::token::*;

pub trait ExprVisitor<T> {
    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> T;
    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> T;
    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> T;
    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> T;
}
#[derive(Debug, Clone)]
pub enum Expr {
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
}

#[derive(Debug, Clone)]
pub struct BinaryExpr {
    left: Box<Expr>,
    operator: Token,
    right: Box<Expr>,
}

impl BinaryExpr {
    fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> T {
        visitor.visit_binary_expr(self)
    }
}

#[derive(Debug, Clone)]
pub struct GroupingExpr {
    expression: Box<Expr>,
}

impl GroupingExpr {
    fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> T {
        visitor.visit_grouping_expr(self)
    }
}

#[derive(Debug, Clone)]
pub struct LiteralExpr {
    value: Object,
}

impl LiteralExpr {
    fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> T {
        visitor.visit_literal_expr(self)
    }
}

#[derive(Debug, Clone)]
pub struct UnaryExpr {
    operator: Token,
    right: Box<Expr>,
}

impl UnaryExpr {
    fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> T {
        visitor.visit_unary_expr(self)
    }
}

