use crate::token::*;
use crate::object::*;
use std::hash::Hash;

pub trait ExprVisitor<T> {
    fn visit_assign_expr(&mut self, expr: &AssignExpr) -> T;
    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> T;
    fn visit_call_expr(&mut self, expr: &CallExpr) -> T;
    fn visit_get_expr(&mut self, expr: &GetExpr) -> T;
    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> T;
    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> T;
    fn visit_logical_expr(&mut self, expr: &LogicalExpr) -> T;
    fn visit_set_expr(&mut self, expr: &SetExpr) -> T;
    fn visit_this_expr(&mut self, expr: &ThisExpr) -> T;
    fn visit_super_expr(&mut self, expr: &SuperExpr) -> T;
    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> T;
    fn visit_ternary_expr(&mut self, expr: &TernaryExpr) -> T;
    fn visit_variable_expr(&mut self, expr: &VariableExpr) -> T;
}
#[derive(Debug, Clone)]
pub enum Expr {
    Assign(AssignExpr),
    Binary(BinaryExpr),
    Call(CallExpr),
    Get(GetExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Logical(LogicalExpr),
    Set(SetExpr),
    This(ThisExpr),
    Super(SuperExpr),
    Unary(UnaryExpr),
    Ternary(TernaryExpr),
    Variable(VariableExpr),
}

#[derive(Debug, Clone)]
pub struct AssignExpr {
    pub uid: usize,
    pub name: Token,
    pub value: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct BinaryExpr {
    pub uid: usize,
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct CallExpr {
    pub uid: usize,
    pub callee: Box<Expr>,
    pub paren: Token,
    pub arguments: Vec<Expr>,
}

#[derive(Debug, Clone)]
pub struct GetExpr {
    pub uid: usize,
    pub object: Box<Expr>,
    pub name: Token,
}

#[derive(Debug, Clone)]
pub struct GroupingExpr {
    pub uid: usize,
    pub expression: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct LiteralExpr {
    pub uid: usize,
    pub value: Object,
}

#[derive(Debug, Clone)]
pub struct LogicalExpr {
    pub uid: usize,
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct SetExpr {
    pub uid: usize,
    pub object: Box<Expr>,
    pub name: Token,
    pub value: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct ThisExpr {
    pub uid: usize,
    pub keyword: Token,
}

#[derive(Debug, Clone)]
pub struct SuperExpr {
    pub uid: usize,
    pub keyword: Token,
    pub method: Token,
}

#[derive(Debug, Clone)]
pub struct UnaryExpr {
    pub uid: usize,
    pub operator: Token,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct TernaryExpr {
    pub uid: usize,
    pub condition: Box<Expr>,
    pub then_branch: Box<Expr>,
    pub else_branch: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct VariableExpr {
    pub uid: usize,
    pub name: Token,
}

impl Expr {
    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> T {
        match self {
            Expr::Assign(assign_expr) => visitor.visit_assign_expr(assign_expr),
            Expr::Binary(binary_expr) => visitor.visit_binary_expr(binary_expr),
            Expr::Call(call_expr) => visitor.visit_call_expr(call_expr),
            Expr::Get(get_expr) => visitor.visit_get_expr(get_expr),
            Expr::Grouping(grouping_expr) => visitor.visit_grouping_expr(grouping_expr),
            Expr::Literal(literal_expr) => visitor.visit_literal_expr(literal_expr),
            Expr::Logical(logical_expr) => visitor.visit_logical_expr(logical_expr),
            Expr::Set(set_expr) => visitor.visit_set_expr(set_expr),
            Expr::This(this_expr) => visitor.visit_this_expr(this_expr),
            Expr::Super(super_expr) => visitor.visit_super_expr(super_expr),
            Expr::Unary(unary_expr) => visitor.visit_unary_expr(unary_expr),
            Expr::Ternary(ternary_expr) => visitor.visit_ternary_expr(ternary_expr),
            Expr::Variable(variable_expr) => visitor.visit_variable_expr(variable_expr),
        }
    }
    fn get_uid(&self) -> usize {
        match self {
            Expr::Assign(expr) => expr.uid,
            Expr::Binary(expr) => expr.uid,
            Expr::Call(expr) => expr.uid,
            Expr::Get(expr) => expr.uid,
            Expr::Grouping(expr) => expr.uid,
            Expr::Literal(expr) => expr.uid,
            Expr::Logical(expr) => expr.uid,
            Expr::Set(expr) => expr.uid,
            Expr::This(expr) => expr.uid,
            Expr::Super(expr) => expr.uid,
            Expr::Unary(expr) => expr.uid,
            Expr::Ternary(expr) => expr.uid,
            Expr::Variable(expr) => expr.uid,
        }
    }
}

impl PartialEq for Expr {
    fn eq(&self, other: &Self) -> bool {
        self.get_uid() == other.get_uid()
    }
}

impl Eq for Expr {}

impl Hash for Expr {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.get_uid().hash(state);
    }
}


