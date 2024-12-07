use crate::token::*;
use crate::object::*;
use crate::expr::*;

pub trait StmtVisitor<T> {
    fn visit_block_stmt(&mut self, stmt: &BlockStmt) -> T;
    fn visit_expression_stmt(&mut self, stmt: &ExpressionStmt) -> T;
    fn visit_if_stmt(&mut self, stmt: &IfStmt) -> T;
    fn visit_print_stmt(&mut self, stmt: &PrintStmt) -> T;
    fn visit_var_stmt(&mut self, stmt: &VarStmt) -> T;
    fn visit_while_stmt(&mut self, stmt: &WhileStmt) -> T;
}
#[derive(Debug, Clone)]
pub enum Stmt {
    Block(BlockStmt),
    Expression(ExpressionStmt),
    If(IfStmt),
    Print(PrintStmt),
    Var(VarStmt),
    While(WhileStmt),
}

#[derive(Debug, Clone)]
pub struct BlockStmt {
    pub statements: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct ExpressionStmt {
    pub expression: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct IfStmt {
    pub condition: Box<Expr>,
    pub then_branch: Box<Stmt>,
    pub else_branch: Option<Box<Stmt>>,
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

#[derive(Debug, Clone)]
pub struct WhileStmt {
    pub condition: Box<Expr>,
    pub body: Box<Stmt>,
}

impl Stmt {
    pub fn accept<T>(&self, visitor: &mut dyn StmtVisitor<T>) -> T {
        match self {
            Stmt::Block(block_stmt) => visitor.visit_block_stmt(block_stmt),
            Stmt::Expression(expression_stmt) => visitor.visit_expression_stmt(expression_stmt),
            Stmt::If(if_stmt) => visitor.visit_if_stmt(if_stmt),
            Stmt::Print(print_stmt) => visitor.visit_print_stmt(print_stmt),
            Stmt::Var(var_stmt) => visitor.visit_var_stmt(var_stmt),
            Stmt::While(while_stmt) => visitor.visit_while_stmt(while_stmt),
        }
    }
}


