use std::collections::HashMap;

use crate::{error::*, expr::*, interpreter::*, stmt::*, token::Token};

pub struct Resolver<'a> {
    pub interpreter: &'a mut Interpreter,
    pub scopes: Vec<HashMap<String, bool>>,
}

impl Resolver<'_> {
    pub fn new(interpreter: &mut Interpreter) -> Resolver {
        Resolver {
            interpreter,
            scopes: Vec::new(),
        }
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn resolve(&mut self, statements: &[Stmt]) -> Result<(), LoxErrorResult> {
        for statement in statements {
            self.resolve_stmt(statement)?;
        }
        Ok(())
    }

    fn resolve_stmt(&mut self, statement: &Stmt) -> Result<(), LoxErrorResult> {
        statement.accept(self)
    }

    fn resolve_expr(&mut self, expression: &Expr) -> Result<(), LoxErrorResult> {
        expression.accept(self)
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn declare(&mut self, name: &Token) -> Result<(), LoxErrorResult> {
        if let Some(scope) = self.scopes.last_mut() {
            if scope.contains_key(&name.lexeme) {
                return Err(LoxErrorResult::resolver_error(
                    name.clone(),
                    "Already a variable with this name in this scope.",
                ));
            }
            scope.insert(name.lexeme(), false);
        }
        Ok(())
    }

    fn define(&mut self, name: &Token) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name.lexeme(), true);
        }
    }

    ///  We start at the innermost scope and work outwards, looking in each map for a matching name.
    /// If we find the variable, we resolve it, passing in the number of scopes between the current innermost scope and the scope where the variable was found.
    /// So, if the variable was found in the current scope, we pass in 0. If it’s in the immediately enclosing scope, 1. You get the idea.
    /// The order of iteration it is really important!
    fn resolve_local(&mut self, expression: &Expr, name: &Token) {
        for (idx, scope) in self.scopes.iter().enumerate().rev() {
            if scope.contains_key(&name.lexeme) {
                let depth = self.scopes.len() - 1 - idx;
                self.interpreter.resolve(expression, depth);
                return;
            }
        }
    }

    fn resolve_function(&mut self, function: &FunctionStmt) -> Result<(), LoxErrorResult> {
        self.begin_scope();
        for param in function.params.iter() {
            self.declare(param)?;
            self.define(param);
        }
        self.resolve(&function.body)?;
        self.end_scope();
        Ok(())
    }
}

impl StmtVisitor<Result<(), LoxErrorResult>> for Resolver<'_> {
    fn visit_block_stmt(&mut self, stmt: &BlockStmt) -> Result<(), LoxErrorResult> {
        self.begin_scope();
        self.resolve(&stmt.statements)?;
        self.end_scope();
        Ok(())
    }

    fn visit_expression_stmt(&mut self, stmt: &ExpressionStmt) -> Result<(), LoxErrorResult> {
        self.resolve_expr(&stmt.expression)
    }

    fn visit_function_stmt(&mut self, stmt: &FunctionStmt) -> Result<(), LoxErrorResult> {
        self.declare(&stmt.name)?;
        self.define(&stmt.name);

        self.resolve_function(stmt)?;
        Ok(())
    }

    fn visit_if_stmt(&mut self, stmt: &IfStmt) -> Result<(), LoxErrorResult> {
        self.resolve_expr(&stmt.condition)?;
        self.resolve_stmt(&stmt.then_branch)?;
        if let Some(else_branch) = &stmt.else_branch {
            self.resolve_stmt(else_branch)?;
        }
        Ok(())
    }

    fn visit_print_stmt(&mut self, stmt: &PrintStmt) -> Result<(), LoxErrorResult> {
        self.resolve_expr(&stmt.expression)
    }

    fn visit_return_stmt(&mut self, stmt: &ReturnStmt) -> Result<(), LoxErrorResult> {
        if let Some(value) = &stmt.value {
            self.resolve_expr(value)?;
        }
        Ok(())
    }

    fn visit_var_stmt(&mut self, stmt: &VarStmt) -> Result<(), LoxErrorResult> {
        self.declare(&stmt.name)?;
        if let Some(init_value) = &stmt.initializer {
            self.resolve_expr(init_value)?;
        }
        self.define(&stmt.name);

        Ok(())
    }

    fn visit_while_stmt(&mut self, stmt: &WhileStmt) -> Result<(), LoxErrorResult> {
        self.resolve_expr(&stmt.condition)?;
        self.resolve_stmt(&stmt.body)?;
        Ok(())
    }

    fn visit_break_stmt(&mut self, _stmt: &BreakStmt) -> Result<(), LoxErrorResult> {
        Ok(())
    }
}

impl ExprVisitor<Result<(), LoxErrorResult>> for Resolver<'_> {
    fn visit_assign_expr(&mut self, expr: &AssignExpr) -> Result<(), LoxErrorResult> {
        self.resolve_expr(&expr.value)?;
        self.resolve_local(&Expr::Assign(expr.clone()), &expr.name);
        Ok(())
    }

    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> Result<(), LoxErrorResult> {
        self.resolve_expr(&expr.left)?;
        self.resolve_expr(&expr.right)?;
        Ok(())
    }

    fn visit_call_expr(&mut self, expr: &CallExpr) -> Result<(), LoxErrorResult> {
        self.resolve_expr(&expr.callee)?;

        for argument in expr.arguments.iter() {
            self.resolve_expr(argument)?;
        }

        Ok(())
    }

    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> Result<(), LoxErrorResult> {
        self.resolve_expr(&expr.expression)
    }

    fn visit_literal_expr(&mut self, _expr: &LiteralExpr) -> Result<(), LoxErrorResult> {
        Ok(())
    }

    fn visit_logical_expr(&mut self, expr: &LogicalExpr) -> Result<(), LoxErrorResult> {
        self.resolve_expr(&expr.left)?;
        self.resolve_expr(&expr.right)?;
        Ok(())
    }

    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> Result<(), LoxErrorResult> {
        self.resolve_expr(&expr.right)?;
        Ok(())
    }

    fn visit_ternary_expr(&mut self, expr: &TernaryExpr) -> Result<(), LoxErrorResult> {
        self.resolve_expr(&expr.condition)?;
        self.resolve_expr(&expr.then_branch)?;
        self.resolve_expr(&expr.else_branch)?;
        Ok(())
    }

    fn visit_variable_expr(&mut self, expr: &VariableExpr) -> Result<(), LoxErrorResult> {
        if let Some(scope) = self.scopes.last() {
            if let Some(false) = scope.get(&expr.name.lexeme) {
                return Err(LoxErrorResult::resolver_error(
                    expr.name.clone(),
                    "Cannot read local variable in its own initializer.",
                ));
            }
        }

        self.resolve_local(&Expr::Variable(expr.clone()), &expr.name);

        Ok(())
    }
}
