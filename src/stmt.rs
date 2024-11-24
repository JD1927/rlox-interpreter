use crate::token::*;
use crate::object::*;
use crate::expr::*;

pub trait StmtVisitor<T> {
    fn visit_expression_stmt(&mut self, stmt: &ExpressionStmt) -> T;
    fn visit_print_stmt(&mut self, stmt: &PrintStmt) -> T;
    fn visit_var_stmt(&mut self, stmt: &VarStmt) -> T;
}
#[derive(Debug, Clone)]
pub enum Stmt {
    Expression(ExpressionStmt),
    Print(PrintStmt),
    Var(VarStmt),
}

#[derive(Debug, Clone)]
pub struct ExpressionStmt {
    pub expression: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct PrintStmt {
    pub expression: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct VarStmt {
    pub name: Token,
    pub initializer: Box<Expr>,
}

impl Stmt {
    pub fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> T {
        match self {
            Stmt::Expression(expression_stmt) => visitor.visit_expression_stmt(expression_stmt),
            Stmt::Print(print_stmt) => visitor.visit_print_stmt(print_stmt),
            Stmt::Var(var_stmt) => visitor.visit_var_stmt(var_stmt),
        }
    }
}


