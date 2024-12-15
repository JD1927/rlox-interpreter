use std::{borrow::BorrowMut, collections::HashMap};

use crate::{error::*, expr::*, interpreter::*, stmt::*, token::Token};

pub struct Resolver {
    pub interpreter: Interpreter,
    pub scopes: Vec<HashMap<String, bool>>,
}

impl Resolver {
    pub fn new(interpreter: Interpreter) -> Resolver {
        Resolver {
            interpreter,
            scopes: Vec::new(),
        }
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn resolve(&mut self, statements: &[Stmt]) -> Result<(), LoxErrorResult> {
        for statement in statements {
            self.resolve_statement(statement)?;
        }
        Ok(())
    }

    fn resolve_statement(&mut self, statement: &Stmt) -> Result<(), LoxErrorResult> {
        statement.accept(self)
    }

    fn resolve_expression(&mut self, expression: &Expr) -> Result<(), LoxErrorResult> {
        expression.accept(self)
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: &Token) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.lexeme.clone(), false);
        }
    }

    fn define(&mut self, name: &Token) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.lexeme.clone(), true);
        }
    }
}

impl StmtVisitor<Result<(), LoxErrorResult>> for Resolver {
    fn visit_block_stmt(&mut self, stmt: &BlockStmt) -> Result<(), LoxErrorResult> {
        self.begin_scope();
        self.resolve(&stmt.statements)?;
        self.end_scope();
        Ok(())
    }

    fn visit_expression_stmt(&mut self, stmt: &ExpressionStmt) -> Result<(), LoxErrorResult> {
        todo!()
    }

    fn visit_function_stmt(&mut self, stmt: &FunctionStmt) -> Result<(), LoxErrorResult> {
        todo!()
    }

    fn visit_if_stmt(&mut self, stmt: &IfStmt) -> Result<(), LoxErrorResult> {
        todo!()
    }

    fn visit_print_stmt(&mut self, stmt: &PrintStmt) -> Result<(), LoxErrorResult> {
        todo!()
    }

    fn visit_return_stmt(&mut self, stmt: &ReturnStmt) -> Result<(), LoxErrorResult> {
        todo!()
    }

    fn visit_var_stmt(&mut self, stmt: &VarStmt) -> Result<(), LoxErrorResult> {
        self.declare(&stmt.name);
        if let Some(init_value) = &stmt.initializer {
            self.resolve_expression(init_value)?;
        }
        self.define(&stmt.name);

        Ok(())
    }

    fn visit_while_stmt(&mut self, stmt: &WhileStmt) -> Result<(), LoxErrorResult> {
        todo!()
    }

    fn visit_break_stmt(&mut self, stmt: &BreakStmt) -> Result<(), LoxErrorResult> {
        todo!()
    }
}

impl ExprVisitor<Result<(), LoxErrorResult>> for Resolver {
    fn visit_assign_expr(&mut self, expr: &AssignExpr) -> Result<(), LoxErrorResult> {
        todo!()
    }

    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> Result<(), LoxErrorResult> {
        todo!()
    }

    fn visit_call_expr(&mut self, expr: &CallExpr) -> Result<(), LoxErrorResult> {
        todo!()
    }

    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> Result<(), LoxErrorResult> {
        todo!()
    }

    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> Result<(), LoxErrorResult> {
        todo!()
    }

    fn visit_logical_expr(&mut self, expr: &LogicalExpr) -> Result<(), LoxErrorResult> {
        todo!()
    }

    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> Result<(), LoxErrorResult> {
        todo!()
    }

    fn visit_ternary_expr(&mut self, expr: &TernaryExpr) -> Result<(), LoxErrorResult> {
        todo!()
    }

    fn visit_variable_expr(&mut self, expr: &VariableExpr) -> Result<(), LoxErrorResult> {
        todo!()
    }
}
