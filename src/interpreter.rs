use std::{
    collections::HashMap,
    rc::Rc,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    environment::*, error::*, expr::*, lox_callable::*, lox_class::LoxClass,
    lox_function::LoxFunction, lox_native_function::*, object::*, stmt::*, token::*,
};

#[derive(Debug, Clone)]
pub struct Interpreter {
    environment: EnvironmentRef,
    pub globals: EnvironmentRef,
    pub locals: HashMap<Expr, usize>,
}

impl Interpreter {
    pub fn new() -> Interpreter {
        let globals = Environment::new();
        globals.borrow_mut().define(
            "clock".to_string(),
            Object::NativeFunction(LoxNativeFunction {
                name: "clock".to_string(),
                arity: 0,
                callable: |_, _| match SystemTime::now().duration_since(UNIX_EPOCH) {
                    Ok(timestamp) => Ok(Object::Number(timestamp.as_millis() as f64)),
                    Err(err) => Err(LoxErrorResult::system_error(&format!(
                        "Clock returned an invalid duration: {}",
                        &err.to_string()
                    ))),
                },
            }),
        );
        Interpreter {
            environment: globals.clone(),
            globals,
            locals: HashMap::new(),
        }
    }

    pub fn interpret(&mut self, statements: &[Stmt]) {
        for statement in statements {
            match self.execute(statement) {
                Ok(_) => (),
                Err(err) => err.report(),
            }
        }
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<(), LoxErrorResult> {
        stmt.accept(self)
    }

    pub fn resolve(&mut self, expression: &Expr, depth: usize) {
        self.locals.insert(expression.clone(), depth);
    }

    pub fn execute_block(
        &mut self,
        statements: &[Stmt],
        new_env: EnvironmentRef,
    ) -> Result<(), LoxErrorResult> {
        // Stores current env until this point
        let previous_env = Rc::clone(&self.environment);

        // Update the interpreter's environment to the new one
        self.environment = new_env;
        // Executes each statement until it reaches an error
        let result = statements.iter().try_for_each(|stmt| self.execute(stmt));
        // Get back the previous environment;
        self.environment = previous_env;
        result
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Object, LoxErrorResult> {
        expr.accept(self)
    }

    fn is_truthy(&mut self, value: Object) -> bool {
        match value {
            Object::Nil => false,
            Object::Bool(val) => val,
            _ => true,
        }
    }

    fn look_up_variable(&mut self, name: &Token, expr: &Expr) -> Result<Object, LoxErrorResult> {
        if let Some(distance) = self.locals.get(expr) {
            self.environment.borrow().get_at(*distance, name)
        } else {
            self.globals.borrow().get(name)
        }
    }
}

impl StmtVisitor<Result<(), LoxErrorResult>> for Interpreter {
    fn visit_expression_stmt(&mut self, stmt: &ExpressionStmt) -> Result<(), LoxErrorResult> {
        self.evaluate(&stmt.expression)?;
        Ok(())
    }

    fn visit_print_stmt(&mut self, stmt: &PrintStmt) -> Result<(), LoxErrorResult> {
        let value = self.evaluate(&stmt.expression)?;
        println!("{value}");
        Ok(())
    }

    fn visit_var_stmt(&mut self, stmt: &VarStmt) -> Result<(), LoxErrorResult> {
        let initializer = if let Some(init_value) = &stmt.initializer {
            self.evaluate(init_value)?
        } else {
            Object::Nil
        };

        self.environment
            .borrow_mut()
            .define(stmt.name.lexeme(), initializer);
        Ok(())
    }

    fn visit_block_stmt(&mut self, stmt: &BlockStmt) -> Result<(), LoxErrorResult> {
        let new_env = Environment::new_enclosing(Rc::clone(&self.environment));
        self.execute_block(&stmt.statements, new_env)
    }

    fn visit_if_stmt(&mut self, stmt: &IfStmt) -> Result<(), LoxErrorResult> {
        let condition = self.evaluate(&stmt.condition)?;
        if self.is_truthy(condition) {
            self.execute(&stmt.then_branch)
        } else if let Some(else_branch) = &stmt.else_branch {
            self.execute(else_branch)
        } else {
            Ok(())
        }
    }

    fn visit_while_stmt(&mut self, stmt: &WhileStmt) -> Result<(), LoxErrorResult> {
        loop {
            let condition_is_truthy = {
                let condition = self.evaluate(&stmt.condition)?;
                self.is_truthy(condition)
            };
            // Break the loop when the condition is false
            if !condition_is_truthy {
                break;
            }
            // Execute the body of the loop
            // If there is an error or break statement it does an exit
            if let Err(err) = self.execute(&stmt.body) {
                if err.is_control_break() {
                    break;
                }
                return Err(err);
            };
        }
        Ok(())
    }

    fn visit_break_stmt(&mut self, _stmt: &BreakStmt) -> Result<(), LoxErrorResult> {
        Err(LoxErrorResult::break_signal())
    }

    fn visit_function_stmt(&mut self, stmt: &FunctionStmt) -> Result<(), LoxErrorResult> {
        let function = LoxFunction::new(stmt, Rc::clone(&self.environment));
        self.environment
            .borrow_mut()
            .define(stmt.name.lexeme(), Object::Function(function));
        Ok(())
    }

    fn visit_return_stmt(&mut self, stmt: &ReturnStmt) -> Result<(), LoxErrorResult> {
        let return_value = if let Some(value) = &stmt.value {
            self.evaluate(value)?
        } else {
            Object::Nil
        };
        Err(LoxErrorResult::return_signal(return_value))
    }

    fn visit_class_stmt(&mut self, stmt: &ClassStmt) -> Result<(), LoxErrorResult> {
        self.environment
            .borrow_mut()
            .define(stmt.name.lexeme(), Object::Nil);
        let class = LoxClass::new(stmt.name.lexeme());
        self.environment
            .borrow_mut()
            .assign(&stmt.name, Object::Class(class))?;
        Ok(())
    }
}

impl ExprVisitor<Result<Object, LoxErrorResult>> for Interpreter {
    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> Result<Object, LoxErrorResult> {
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;

        match expr.operator.token_type {
            TokenType::Minus => match left - right {
                Ok(result) => Ok(result),
                Err(message) => Err(LoxErrorResult::interpreter_error(
                    expr.operator.line,
                    &message,
                )),
            },
            TokenType::Slash => match left / right {
                Ok(result) => Ok(result),
                Err(message) => Err(LoxErrorResult::interpreter_error(
                    expr.operator.line,
                    &message,
                )),
            },
            TokenType::Star => match left * right {
                Ok(result) => Ok(result),
                Err(message) => Err(LoxErrorResult::interpreter_error(
                    expr.operator.line,
                    &message,
                )),
            },
            TokenType::Plus => match left + right {
                Ok(result) => Ok(result),
                Err(message) => Err(LoxErrorResult::interpreter_error(
                    expr.operator.line,
                    &message,
                )),
            },
            TokenType::Greater => match (left, right) {
                (Object::Number(left), Object::Number(right)) => Ok(Object::Bool(left > right)),
                _ => Err(LoxErrorResult::interpreter_error(
                    expr.operator.line,
                    "Operands must be numbers for '>' operation.",
                )),
            },
            TokenType::GreaterEqual => match (left, right) {
                (Object::Number(left), Object::Number(right)) => Ok(Object::Bool(left >= right)),
                _ => Err(LoxErrorResult::interpreter_error(
                    expr.operator.line,
                    "Operands must be numbers for '>=' operation.",
                )),
            },
            TokenType::Less => match (left, right) {
                (Object::Number(left), Object::Number(right)) => Ok(Object::Bool(left < right)),
                _ => Err(LoxErrorResult::interpreter_error(
                    expr.operator.line,
                    "Operands must be numbers for '<' operation.",
                )),
            },
            TokenType::LessEqual => match (left, right) {
                (Object::Number(left), Object::Number(right)) => Ok(Object::Bool(left <= right)),
                _ => Err(LoxErrorResult::interpreter_error(
                    expr.operator.line,
                    "Operands must be numbers for '<=' operation.",
                )),
            },
            TokenType::BangEqual => Ok(Object::Bool(left != right)),
            TokenType::EqualEqual => Ok(Object::Bool(left == right)),
            _ => Err(LoxErrorResult::interpreter_error(
                expr.operator.line,
                "Unsupported binary operation.",
            )),
        }
    }

    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> Result<Object, LoxErrorResult> {
        self.evaluate(&expr.expression)
    }

    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> Result<Object, LoxErrorResult> {
        Ok(expr.value.clone())
    }

    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> Result<Object, LoxErrorResult> {
        let right = self.evaluate(&expr.right)?;

        match expr.operator.token_type {
            TokenType::Bang => Ok(Object::Bool(!self.is_truthy(right))),
            TokenType::Minus => match right {
                Object::Number(val) => Ok(Object::Number(-val)),
                _ => Err(LoxErrorResult::interpreter_error(
                    expr.operator.line,
                    "Operand must be a number.",
                )),
            },
            _ => Err(LoxErrorResult::interpreter_error(
                expr.operator.line,
                "Unsupported unary operator",
            )),
        }
    }

    fn visit_ternary_expr(&mut self, expr: &TernaryExpr) -> Result<Object, LoxErrorResult> {
        let condition = self.evaluate(&expr.condition)?;
        match self.is_truthy(condition) {
            true => self.evaluate(&expr.then_branch),
            false => self.evaluate(&expr.else_branch),
        }
    }

    fn visit_variable_expr(&mut self, expr: &VariableExpr) -> Result<Object, LoxErrorResult> {
        self.look_up_variable(&expr.name, &Expr::Variable(expr.clone()))
    }

    fn visit_assign_expr(&mut self, expr: &AssignExpr) -> Result<Object, LoxErrorResult> {
        let value = self.evaluate(&expr.value)?;
        let local_value = self.locals.get(&Expr::Assign(expr.clone()));
        if let Some(distance) = local_value {
            self.environment
                .borrow_mut()
                .assign_at(*distance, &expr.name, &value);
        } else {
            self.globals
                .borrow_mut()
                .assign(&expr.name, value.clone())?;
        }
        Ok(value)
    }

    fn visit_logical_expr(&mut self, expr: &LogicalExpr) -> Result<Object, LoxErrorResult> {
        let left = self.evaluate(&expr.left)?;

        if expr.operator.is(TokenType::Or) {
            if self.is_truthy(left.clone()) {
                return Ok(left);
            }
        } else if !self.is_truthy(left.clone()) {
            return Ok(left);
        }

        self.evaluate(&expr.right)
    }

    fn visit_call_expr(&mut self, expr: &CallExpr) -> Result<Object, LoxErrorResult> {
        let callee = self.evaluate(&expr.callee)?;

        let mut arguments: Vec<Object> = Vec::new();

        for argument in &expr.arguments {
            arguments.push(self.evaluate(argument)?);
        }

        match callee {
            Object::Function(mut function) => {
                function.check_arity(arguments.len(), &expr.paren)?;
                function.call(self, arguments)
            }
            Object::NativeFunction(mut native_function) => {
                native_function.check_arity(arguments.len(), &expr.paren)?;
                native_function.call(self, arguments)
            }
            Object::Class(mut class) => {
                class.check_arity(arguments.len(), &expr.paren)?;
                class.call(self, arguments)
            }
            _ => Err(LoxErrorResult::interpreter_error(
                expr.paren.line,
                "Can only call functions and classes.",
            )),
        }
    }

    fn visit_get_expr(&mut self, expr: &GetExpr) -> Result<Object, LoxErrorResult> {
        let object = self.evaluate(&expr.object)?;

        match object {
            Object::ClassInstance(instance) => Ok(instance.get(&expr.name)?),
            _ => Err(LoxErrorResult::interpreter_error(
                expr.name.line,
                "Only instances have properties.",
            )),
        }
    }
}

#[cfg(test)]
mod interpreter_tests {
    use super::*;

    fn make_literal(obj: Object) -> Box<Expr> {
        Box::new(Expr::Literal(LiteralExpr { value: obj, uid: 0 }))
    }

    fn make_literal_number(num: f64) -> Box<Expr> {
        Box::new(Expr::Literal(LiteralExpr {
            value: Object::Number(num),
            uid: 0,
        }))
    }

    fn make_literal_string(str_val: &str) -> Box<Expr> {
        Box::new(Expr::Literal(LiteralExpr {
            value: Object::String(str_val.to_string()),
            uid: 0,
        }))
    }

    fn make_literal_bool(value: bool) -> Box<Expr> {
        Box::new(Expr::Literal(LiteralExpr {
            value: Object::Bool(value),
            uid: 0,
        }))
    }

    fn make_token_operator(token_type: TokenType, operator: &str) -> Token {
        Token::new(token_type, operator.to_string(), Object::Nil, 1)
    }

    fn make_token_identifier(identifier: &str) -> Token {
        Token::new(
            TokenType::Identifier,
            identifier.to_string(),
            Object::Nil,
            1,
        )
    }

    fn run_binary_test(
        token: Token,
        operands: Vec<(Object, Object)>,
        results: Vec<(bool, Object)>,
    ) {
        let mut interpreter = Interpreter::new();

        println!("[Test]: Testing '{}' operator", &token.lexeme);
        for (idx, operand) in operands.iter().enumerate() {
            println!(
                "({:?} {} {:?})",
                operand.0.clone(),
                &token.lexeme,
                operand.1.clone()
            );
            let binary_expr: BinaryExpr = BinaryExpr {
                left: make_literal(operand.0.to_owned()),
                operator: token.to_owned(),
                right: make_literal(operand.1.to_owned()),
                uid: 0,
            };
            // Act
            let result = interpreter.visit_binary_expr(&binary_expr);
            // Assert
            let message_for_ok = format!(
                "Found an issue while testing => ({:?} {} {:?}) operation",
                operand.0.clone(),
                &token.lexeme,
                operand.1.clone()
            );
            let message_for_err = format!(
                "Wrong error message at => ({:?} {} {:?}) operation",
                operand.0.clone(),
                &token.lexeme,
                operand.1.clone()
            );
            assert_eq!(result.is_ok(), results[idx].0, "{}", &message_for_ok);
            if result.is_ok() {
                assert_eq!(
                    result.ok(),
                    Some(results[idx].1.to_owned()),
                    "{}",
                    &message_for_ok
                );
            } else if let Some(LoxErrorResult::Interpreter { line: _, message }) = result.err() {
                assert!(message.contains(&token.lexeme), "{}", &message_for_err);
                assert!(message.contains("Operands must be"), "{}", &message_for_err);
            }
        }
    }

    fn get_test_number_operands() -> Vec<(Object, Object)> {
        // (left, right) values
        vec![
            (Object::Number(2.0), Object::Number(3.0)),
            (Object::Number(3.0), Object::Number(1.0)),
            (Object::Number(3.0), Object::Number(3.0)),
            // Errors
            (Object::String("4.0".to_string()), Object::Nil),
            (Object::Nil, Object::String("2.0".to_string())),
            (Object::Bool(true), Object::String("2.0".to_string())),
            (Object::Bool(true), Object::Number(3.0)),
            (Object::Bool(true), Object::Bool(false)),
        ]
    }

    fn get_test_string_operands() -> Vec<(Object, Object)> {
        // (left, right) values
        vec![
            (
                Object::String("Hi, ".to_string()),
                Object::String("Rusty".to_string()),
            ),
            (
                Object::String("To".to_string()),
                Object::String("gether".to_string()),
            ),
            (
                Object::String("Split".to_string()),
                Object::String(" two".to_string()),
            ),
            (Object::String("4.0".to_string()), Object::Number(3.0)),
            (Object::Number(3.0), Object::String("2.0".to_string())),
            // Errors
            (Object::Bool(true), Object::String("2.0".to_string())),
            (Object::Bool(true), Object::Number(3.0)),
            (Object::Bool(true), Object::Bool(false)),
        ]
    }

    fn get_test_cmp_operands() -> Vec<(Object, Object)> {
        // (left, right) values
        vec![
            // True
            (Object::Number(3.0), Object::Number(3.0)),
            (
                Object::String("4.0".to_string()),
                Object::String("4.0".to_string()),
            ),
            (Object::Bool(true), Object::Bool(true)),
            (Object::Bool(false), Object::Bool(false)),
            (Object::Nil, Object::Nil),
            // False
            (Object::Bool(false), Object::Bool(true)),
            (Object::Number(2.0), Object::Number(3.0)),
            (Object::String("4.0".to_string()), Object::Number(4.0)),
            (Object::Number(3.0), Object::String("3.0".to_string())),
            (Object::Bool(true), Object::String("2.0".to_string())),
            (Object::Bool(true), Object::Number(3.0)),
            (Object::Bool(true), Object::Bool(false)),
        ]
    }

    #[test]
    fn test_subtraction() {
        let token = make_token_operator(TokenType::Minus, "-");
        // Operands and results
        let operands: Vec<(Object, Object)> = get_test_number_operands();
        let results: Vec<(bool, Object)> = vec![
            (true, Object::Number(-1.0)), // 2.0 , 3.0
            (true, Object::Number(2.0)),  // 3.0 , 1.0
            (true, Object::Number(0.0)),  // 3.0 , 3.0
            // Errors
            (false, Object::Nil),
            (false, Object::Nil),
            (false, Object::Nil),
            (false, Object::Nil),
            (false, Object::Nil),
        ];
        run_binary_test(token, operands, results);
    }

    #[test]
    fn test_division() {
        let token = make_token_operator(TokenType::Slash, "/");
        // Operands and results
        let operands: Vec<(Object, Object)> = get_test_number_operands();
        let results: Vec<(bool, Object)> = vec![
            (true, Object::Number(2.0 / 3.0)), // 2.0 , 3.0
            (true, Object::Number(3.0)),       // 3.0 , 1.0
            (true, Object::Number(1.0)),       // 3.0 , 3.0
            // Errors
            (false, Object::Nil),
            (false, Object::Nil),
            (false, Object::Nil),
            (false, Object::Nil),
            (false, Object::Nil),
        ];
        run_binary_test(token, operands, results);
    }

    #[test]
    fn test_multiplication() {
        let token = make_token_operator(TokenType::Star, "*");
        // Operands and results
        let operands: Vec<(Object, Object)> = get_test_number_operands();
        let results: Vec<(bool, Object)> = vec![
            (true, Object::Number(6.0)), // 2.0 , 3.0
            (true, Object::Number(3.0)), // 3.0 , 1.0
            (true, Object::Number(9.0)), // 3.0 , 3.0
            // Errors
            (false, Object::Nil),
            (false, Object::Nil),
            (false, Object::Nil),
            (false, Object::Nil),
            (false, Object::Nil),
        ];
        run_binary_test(token, operands, results);
    }

    #[test]
    fn test_addition() {
        let token = make_token_operator(TokenType::Plus, "+");
        // Operands and results
        let operands: Vec<(Object, Object)> = get_test_number_operands();
        let results: Vec<(bool, Object)> = vec![
            (true, Object::Number(5.0)), // 2.0 , 3.0
            (true, Object::Number(4.0)), // 3.0 , 1.0
            (true, Object::Number(6.0)), // 3.0 , 3.0
            // Errors
            (false, Object::String("43".to_string())),
            (false, Object::Nil),
            (false, Object::Nil),
            (false, Object::Nil),
            (false, Object::Nil),
        ];
        run_binary_test(token, operands, results);
    }

    #[test]
    fn test_concatenation() {
        let token = make_token_operator(TokenType::Plus, "+");
        // Operands and results
        let operands: Vec<(Object, Object)> = get_test_string_operands();
        let results: Vec<(bool, Object)> = vec![
            (true, Object::String("Hi, Rusty".to_string())),
            (true, Object::String("Together".to_string())),
            (true, Object::String("Split two".to_string())),
            (true, Object::String("4.03".to_string())),
            (true, Object::String("32.0".to_string())),
            // Errors
            (false, Object::Nil),
            (false, Object::Nil),
            (false, Object::Nil),
        ];
        run_binary_test(token, operands, results);
    }

    #[test]
    fn test_greater_than_operator() {
        let token = make_token_operator(TokenType::Greater, ">");
        // Operands and results
        let operands: Vec<(Object, Object)> = get_test_number_operands();
        let results: Vec<(bool, Object)> = vec![
            (true, Object::Bool(false)), // 2.0 , 3.0
            (true, Object::Bool(true)),  // 3.0 , 1.0
            (true, Object::Bool(false)), // 3.0 , 3.0
            // Errors
            (false, Object::Nil),
            (false, Object::Nil),
            (false, Object::Nil),
            (false, Object::Nil),
            (false, Object::Nil),
        ];
        run_binary_test(token, operands, results);
    }

    #[test]
    fn test_greater_than_or_equal_operator() {
        let token = make_token_operator(TokenType::GreaterEqual, ">=");
        // Operands and results
        let operands: Vec<(Object, Object)> = get_test_number_operands();
        let results: Vec<(bool, Object)> = vec![
            (true, Object::Bool(false)), // 2.0 , 3.0
            (true, Object::Bool(true)),  // 3.0 , 1.0
            (true, Object::Bool(true)),  // 3.0 , 3.0
            // Errors
            (false, Object::Nil),
            (false, Object::Nil),
            (false, Object::Nil),
            (false, Object::Nil),
            (false, Object::Nil),
        ];
        run_binary_test(token, operands, results);
    }

    #[test]
    fn test_less_than_operator() {
        let token = make_token_operator(TokenType::Less, "<");
        // Operands and results
        let operands: Vec<(Object, Object)> = get_test_number_operands();
        let results: Vec<(bool, Object)> = vec![
            (true, Object::Bool(true)),  // 2.0 , 3.0
            (true, Object::Bool(false)), // 3.0 , 1.0
            (true, Object::Bool(false)), // 3.0 , 3.0
            // Errors
            (false, Object::Nil),
            (false, Object::Nil),
            (false, Object::Nil),
            (false, Object::Nil),
            (false, Object::Nil),
        ];
        run_binary_test(token, operands, results);
    }

    #[test]
    fn test_less_than_or_equal_operator() {
        let token = make_token_operator(TokenType::LessEqual, "<=");
        // Operands and results
        let operands: Vec<(Object, Object)> = get_test_number_operands();
        let results: Vec<(bool, Object)> = vec![
            (true, Object::Bool(true)),  // 2.0 , 3.0
            (true, Object::Bool(false)), // 3.0 , 1.0
            (true, Object::Bool(true)),  // 3.0 , 3.0
            (false, Object::Nil),
            (false, Object::Nil),
            (false, Object::Nil),
            (false, Object::Nil),
            (false, Object::Nil),
            (false, Object::Nil),
            (false, Object::Nil),
            (false, Object::Nil),
            (false, Object::Nil),
        ];
        run_binary_test(token, operands, results);
    }

    #[test]
    fn test_bang_equal_operator() {
        let token = make_token_operator(TokenType::BangEqual, "!=");
        // Operands and results
        let operands: Vec<(Object, Object)> = get_test_cmp_operands();
        let results: Vec<(bool, Object)> = vec![
            (true, Object::Bool(false)),
            (true, Object::Bool(false)),
            (true, Object::Bool(false)),
            (true, Object::Bool(false)),
            (true, Object::Bool(false)),
            (true, Object::Bool(true)),
            (true, Object::Bool(true)),
            (true, Object::Bool(true)),
            (true, Object::Bool(true)),
            (true, Object::Bool(true)),
            (true, Object::Bool(true)),
            (true, Object::Bool(true)),
        ];
        run_binary_test(token, operands, results);
    }

    #[test]
    fn test_equal_equal_operator() {
        let token = make_token_operator(TokenType::EqualEqual, "==");
        // Operands and results
        let operands: Vec<(Object, Object)> = get_test_cmp_operands();
        let results: Vec<(bool, Object)> = vec![
            (true, Object::Bool(true)),
            (true, Object::Bool(true)),
            (true, Object::Bool(true)),
            (true, Object::Bool(true)),
            (true, Object::Bool(true)),
            (true, Object::Bool(false)),
            (true, Object::Bool(false)),
            (true, Object::Bool(false)),
            (true, Object::Bool(false)),
            (true, Object::Bool(false)),
            (true, Object::Bool(false)),
            (true, Object::Bool(false)),
        ];
        run_binary_test(token, operands, results);
    }

    #[test]
    fn test_ternary_operator() {
        // Arrange
        let mut interpreter = Interpreter::new();
        let ternary: TernaryExpr = TernaryExpr {
            condition: Box::new(Expr::Binary(BinaryExpr {
                left: make_literal_number(69.0),
                operator: make_token_operator(TokenType::EqualEqual, "=="),
                right: make_literal_number(69.0),
                uid: 0,
            })),
            then_branch: make_literal_string("Ohhh yeaahhh!"),
            else_branch: make_literal_string(":c"),
            uid: 0,
        };

        // Act
        let result = interpreter.visit_ternary_expr(&ternary);
        // Assert
        assert!(result.is_ok());
        assert_eq!(
            result.ok(),
            Some(Object::String("Ohhh yeaahhh!".to_string()))
        );
    }

    #[test]
    fn test_unary_minus() {
        // Arrange
        let mut interpreter = Interpreter::new();
        let unary_expr_1 = UnaryExpr {
            operator: make_token_operator(TokenType::Minus, "-"),
            right: make_literal_number(123.0),
            uid: 0,
        };
        let unary_expr_2 = UnaryExpr {
            operator: make_token_operator(TokenType::Minus, "-"),
            right: make_literal_string("Coffee"),
            uid: 0,
        };

        // Act
        let result_1 = interpreter.visit_unary_expr(&unary_expr_1);
        let result_2 = interpreter.visit_unary_expr(&unary_expr_2);
        // Assert
        assert!(result_1.is_ok());
        assert_eq!(result_1.ok(), Some(Object::Number(-123.0)));

        assert!(result_2.is_err());
        println!("{:?}", result_2.err());
    }

    #[test]
    fn test_unary_bang() {
        // Arrange
        let mut interpreter = Interpreter::new();
        let unary_expr = UnaryExpr {
            operator: make_token_operator(TokenType::Bang, "!"),
            right: make_literal_bool(false),
            uid: 0,
        };

        // Act
        let result = interpreter.visit_unary_expr(&unary_expr);
        // Assert
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Bool(true)));
    }

    #[test]
    fn test_var_statement_defined() {
        // Arrange
        let mut interpreter = Interpreter::new();
        let initializer = make_literal_number(123.0);
        let name = make_token_identifier("my_variable");
        let var_stmt = VarStmt {
            name: name.clone(),
            initializer: Some(initializer),
        };

        // Act
        let result = interpreter.visit_var_stmt(&var_stmt);
        // Assert
        assert!(result.is_ok());
        assert!(interpreter.environment.borrow_mut().get(&name).is_ok());
    }

    #[test]
    fn test_var_statement_nil() {
        // Arrange
        let mut interpreter = Interpreter::new();
        let initializer = make_literal(Object::Nil);
        let name = make_token_identifier("my_variable");
        let var_stmt = VarStmt {
            name: name.clone(),
            initializer: Some(initializer),
        };

        // Act
        let result = interpreter.visit_var_stmt(&var_stmt);
        // Assert
        assert!(result.is_ok());
        assert!(interpreter.environment.borrow_mut().get(&name).is_ok());
        assert_eq!(
            interpreter.environment.borrow_mut().get(&name).ok(),
            Some(Object::Nil)
        );
    }

    #[test]
    fn test_var_expression_defined() {
        // Arrange
        let mut interpreter = Interpreter::new();
        let name = make_token_identifier("my_variable");
        let initializer = make_literal_number(123.0);
        let var_stmt = VarStmt {
            name: name.clone(),
            initializer: Some(initializer),
        };
        let var_expr = VariableExpr {
            name: name.clone(),
            uid: 0,
        };

        // Act
        let result = interpreter.visit_var_stmt(&var_stmt);
        assert!(result.is_ok());

        let result = interpreter.visit_variable_expr(&var_expr);
        // Assert
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Number(123.0)));
    }

    #[test]
    fn test_var_expression_undefined() {
        // Arrange
        let mut interpreter = Interpreter::new();
        let name = make_token_identifier("my_variable");
        let var_expr = VariableExpr { name, uid: 0 };

        // Act
        let result = interpreter.visit_variable_expr(&var_expr);
        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn test_assign_value_to_existing_variable() {
        // Arrange
        let mut interpreter = Interpreter::new();

        let name = make_token_identifier("my_variable");
        let initializer = make_literal_number(123.0);
        let var_stmt = VarStmt {
            name: name.clone(),
            initializer: Some(initializer),
        };

        let value = make_literal_number(321.0);
        let assign_expr = AssignExpr {
            name,
            value,
            uid: 0,
        };

        // Act
        let result = interpreter.visit_var_stmt(&var_stmt);
        assert!(result.is_ok());

        let result = interpreter.visit_assign_expr(&assign_expr);
        // Assert
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Number(321.0)));
    }

    #[test]
    fn test_error_assign_value_to_undefined_variable() {
        // Arrange
        let mut interpreter = Interpreter::new();

        let name = make_token_identifier("my_variable");
        let value = make_literal_number(321.0);
        let assign_expr = AssignExpr {
            name,
            value,
            uid: 0,
        };

        // Act
        let result = interpreter.visit_assign_expr(&assign_expr);
        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn test_logic_or() {
        // Arrange
        let mut interpreter = Interpreter::new();
        let left = make_literal_bool(false);
        let operator = make_token_operator(TokenType::Or, "or");
        let right = make_literal_bool(true);
        let logical_expr = LogicalExpr {
            left,
            operator,
            right,
            uid: 0,
        };
        // Act
        let result = interpreter.visit_logical_expr(&logical_expr);
        // Assert
        assert!(result.is_ok());
        assert_eq!(result.ok().unwrap(), Object::Bool(true));
    }

    #[test]
    fn test_logic_and() {
        // Arrange
        let mut interpreter = Interpreter::new();
        let left = make_literal_bool(false);
        let operator = make_token_operator(TokenType::And, "and");
        let right = make_literal_bool(true);
        let logical_expr = LogicalExpr {
            left,
            operator,
            right,
            uid: 0,
        };
        // Act
        let result = interpreter.visit_logical_expr(&logical_expr);
        // Assert
        assert!(result.is_ok());
        assert_eq!(result.ok().unwrap(), Object::Bool(false));
    }
}
