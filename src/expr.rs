use crate::token::*;
use crate::object::*;

pub trait ExprVisitor<T> {
    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> T;
    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> T;
    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> T;
    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> T;
    fn visit_comma_expr(&mut self, expr: &CommaExpr) -> T;
    fn visit_ternary_expr(&mut self, expr: &TernaryExpr) -> T;
}
#[derive(Debug, Clone)]
pub enum Expr {
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
    Comma(CommaExpr),
    Ternary(TernaryExpr),
}

#[derive(Debug, Clone)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct GroupingExpr {
    pub expression: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct LiteralExpr {
    pub value: Object,
}

#[derive(Debug, Clone)]
pub struct UnaryExpr {
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct CommaExpr {
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct TernaryExpr {
    pub condition: Box<Expr>,
    pub then_branch: Box<Expr>,
    pub else_branch: Box<Expr>,
}

impl Expr {
    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> T {
        match self {
            Expr::Binary(binary_expr) => visitor.visit_binary_expr(binary_expr),
            Expr::Grouping(grouping_expr) => visitor.visit_grouping_expr(grouping_expr),
            Expr::Literal(literal_expr) => visitor.visit_literal_expr(literal_expr),
            Expr::Unary(unary_expr) => visitor.visit_unary_expr(unary_expr),
            Expr::Comma(comma_expr) => visitor.visit_comma_expr(comma_expr),
            Expr::Ternary(ternary_expr) => visitor.visit_ternary_expr(ternary_expr),
        }
    }
}


