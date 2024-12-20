use std::collections::HashMap;

use crate::{error::*, expr::*, interpreter::*, stmt::*, token::Token};

#[derive(Debug, Clone)]
pub struct VariableInfo {
    is_defined: bool,
    is_used: bool,
    token: Option<Token>,
}

impl VariableInfo {
    pub fn new(is_defined: bool, is_used: bool, token: Option<Token>) -> VariableInfo {
        VariableInfo {
            is_defined,
            is_used,
            token,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FunctionType {
    None,
    Function,
    Method,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClassType {
    None,
    Class,
}

pub struct Resolver<'a> {
    pub interpreter: &'a mut Interpreter,
    pub scopes: Vec<HashMap<String, VariableInfo>>,
    pub had_error: bool,
    current_function: FunctionType,
    current_class: ClassType,
    in_loop: bool,
}

impl Resolver<'_> {
    pub fn new(interpreter: &mut Interpreter) -> Resolver {
        Resolver {
            interpreter,
            scopes: Vec::new(),
            current_function: FunctionType::None,
            current_class: ClassType::None,
            had_error: false,
            in_loop: false,
        }
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn resolve(&mut self, statements: &[Stmt]) {
        for statement in statements {
            self.resolve_stmt(statement);
        }
    }

    fn resolve_stmt(&mut self, statement: &Stmt) {
        statement.accept(self);
    }

    fn resolve_expr(&mut self, expression: &Expr) {
        expression.accept(self);
    }

    fn end_scope(&mut self) {
        if let Some(scope) = self.scopes.pop() {
            for (_, variable_info) in scope {
                if !variable_info.is_used {
                    if let Some(token) = variable_info.token {
                        LoxErrorResult::warning(token, "Variable is declared but never used.");
                    }
                }
            }
        }
    }

    fn declare(&mut self, name: &Token) {
        if let Some(scope) = self.scopes.last_mut() {
            if scope.contains_key(&name.lexeme) {
                LoxErrorResult::resolver_error(
                    name.clone(),
                    "Already a variable with this name in this scope.",
                );
                self.had_error = true;
            }
            scope.insert(
                name.lexeme(),
                VariableInfo::new(false, false, Some(name.clone())),
            );
        }
    }

    fn define(&mut self, name: &Token) {
        if let Some(scope) = self.scopes.last_mut() {
            if let Some(info) = scope.get_mut(&name.lexeme) {
                info.is_defined = true;
            }
        }
    }

    ///  We start at the innermost scope and work outwards, looking in each map for a matching name.
    /// If we find the variable, we resolve it, passing in the number of scopes between the current innermost scope and the scope where the variable was found.
    /// So, if the variable was found in the current scope, we pass in 0. If it’s in the immediately enclosing scope, 1. You get the idea.
    /// The order of iteration it is really important!
    fn resolve_local(&mut self, expression: &Expr, name: &Token) {
        for (idx, scope) in self.scopes.iter_mut().enumerate().rev() {
            if let Some(info) = scope.get_mut(&name.lexeme) {
                // Mark variable as used!
                info.is_used = true;
                // Resolve the variable
                let depth = self.scopes.len() - 1 - idx;
                self.interpreter.resolve(expression, depth);
                return;
            }
        }
    }

    fn resolve_function(&mut self, function: &FunctionStmt, function_type: FunctionType) {
        let enclosing_function = self.current_function.clone();
        self.current_function = function_type;
        self.begin_scope();
        for param in function.params.iter() {
            self.declare(param);
            self.define(param);
        }
        self.resolve(&function.body);
        self.end_scope();
        self.current_function = enclosing_function;
    }
}

impl StmtVisitor<()> for Resolver<'_> {
    fn visit_block_stmt(&mut self, stmt: &BlockStmt) {
        self.begin_scope();
        self.resolve(&stmt.statements);
        self.end_scope();
    }

    fn visit_expression_stmt(&mut self, stmt: &ExpressionStmt) {
        self.resolve_expr(&stmt.expression)
    }

    fn visit_function_stmt(&mut self, stmt: &FunctionStmt) {
        self.declare(&stmt.name);
        self.define(&stmt.name);

        self.resolve_function(stmt, FunctionType::Function);
    }

    fn visit_if_stmt(&mut self, stmt: &IfStmt) {
        self.resolve_expr(&stmt.condition);
        self.resolve_stmt(&stmt.then_branch);
        if let Some(else_branch) = &stmt.else_branch {
            self.resolve_stmt(else_branch);
        }
    }

    fn visit_print_stmt(&mut self, stmt: &PrintStmt) {
        self.resolve_expr(&stmt.expression)
    }

    fn visit_return_stmt(&mut self, stmt: &ReturnStmt) {
        if self.current_function == FunctionType::None {
            LoxErrorResult::resolver_error(
                stmt.keyword.clone(),
                "Cannot return from top-level code.",
            );
            self.had_error = true;
        }
        if let Some(value) = &stmt.value {
            self.resolve_expr(value);
        }
    }

    fn visit_var_stmt(&mut self, stmt: &VarStmt) {
        self.declare(&stmt.name);
        if let Some(init_value) = &stmt.initializer {
            self.resolve_expr(init_value);
        }
        self.define(&stmt.name);
    }

    fn visit_while_stmt(&mut self, stmt: &WhileStmt) {
        let nesting_loop = self.in_loop;
        self.in_loop = true;
        self.resolve_expr(&stmt.condition);
        self.resolve_stmt(&stmt.body);
        self.in_loop = nesting_loop;
    }

    fn visit_break_stmt(&mut self, stmt: &BreakStmt) {
        if !self.in_loop {
            LoxErrorResult::resolver_error(
                stmt.keyword.clone(),
                "'break' can only be used inside loops.",
            );
            self.had_error = true;
        }
    }

    fn visit_class_stmt(&mut self, stmt: &ClassStmt) {
        let enclosing_class = self.current_class.clone();
        self.current_class = ClassType::Class;

        self.declare(&stmt.name);
        self.define(&stmt.name);

        self.begin_scope();

        if let Some(scope) = self.scopes.last_mut() {
            scope.insert("this".to_string(), VariableInfo::new(true, false, None));
        }

        for stmt in &stmt.methods {
            match stmt {
                Stmt::Function(method) => {
                    let declaration = FunctionType::Method;
                    self.resolve_function(method, declaration);
                }
                _ => panic!("Not a method!"),
            }
        }

        self.current_class = enclosing_class;

        self.end_scope();
    }
}

impl ExprVisitor<()> for Resolver<'_> {
    fn visit_assign_expr(&mut self, expr: &AssignExpr) {
        self.resolve_expr(&expr.value);
        self.resolve_local(&Expr::Assign(expr.clone()), &expr.name);
    }

    fn visit_binary_expr(&mut self, expr: &BinaryExpr) {
        self.resolve_expr(&expr.left);
        self.resolve_expr(&expr.right);
    }

    fn visit_call_expr(&mut self, expr: &CallExpr) {
        self.resolve_expr(&expr.callee);

        for argument in expr.arguments.iter() {
            self.resolve_expr(argument);
        }
    }

    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) {
        self.resolve_expr(&expr.expression)
    }

    fn visit_literal_expr(&mut self, _expr: &LiteralExpr) {}

    fn visit_logical_expr(&mut self, expr: &LogicalExpr) {
        self.resolve_expr(&expr.left);
        self.resolve_expr(&expr.right);
    }

    fn visit_unary_expr(&mut self, expr: &UnaryExpr) {
        self.resolve_expr(&expr.right);
    }

    fn visit_ternary_expr(&mut self, expr: &TernaryExpr) {
        self.resolve_expr(&expr.condition);
        self.resolve_expr(&expr.then_branch);
        self.resolve_expr(&expr.else_branch);
    }

    fn visit_variable_expr(&mut self, expr: &VariableExpr) {
        if let Some(scope) = self.scopes.last() {
            if let Some(variable_info) = scope.get(&expr.name.lexeme) {
                if !variable_info.is_defined {
                    LoxErrorResult::resolver_error(
                        expr.name.clone(),
                        "Cannot read local variable in its own initializer.",
                    );
                    self.had_error = true;
                }
            }
        }

        self.resolve_local(&Expr::Variable(expr.clone()), &expr.name);
    }

    fn visit_get_expr(&mut self, expr: &GetExpr) {
        self.resolve_expr(&expr.object);
    }

    fn visit_set_expr(&mut self, expr: &SetExpr) {
        self.resolve_expr(&expr.value);
        self.resolve_expr(&expr.object);
    }

    fn visit_this_expr(&mut self, expr: &ThisExpr) {
        if self.current_class == ClassType::None {
            LoxErrorResult::resolver_error(
                expr.keyword.clone(),
                "Cannot use 'this' outside of a class",
            );
            self.had_error = true;
            return;
        }
        self.resolve_local(&Expr::This(expr.clone()), &expr.keyword);
    }
}
