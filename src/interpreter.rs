use crate::{environment::*, error::LoxError, expr::*, object::*, stmt::*, token::*};

pub struct Interpreter {
    environment: Environment,
}

impl StmtVisitor<Result<(), LoxError>> for Interpreter {
    fn visit_expression_stmt(&mut self, stmt: &ExpressionStmt) -> Result<(), LoxError> {
        self.evaluate(&stmt.expression)?;
        Ok(())
    }

    fn visit_print_stmt(&mut self, stmt: &PrintStmt) -> Result<(), LoxError> {
        let value = self.evaluate(&stmt.expression)?;
        println!("{value}");
        Ok(())
    }

    fn visit_var_stmt(&mut self, stmt: &VarStmt) -> Result<(), LoxError> {
        let initializer = self.evaluate(&stmt.initializer)?;

        self.environment
            .define(stmt.name.lexeme.clone(), initializer);
        Ok(())
    }

    fn visit_block_stmt(&mut self, stmt: &BlockStmt) -> Result<(), LoxError> {
        self.execute_block(
            &stmt.statements,
            Environment::new_enclosing(self.environment.clone()),
        )
    }

    fn visit_if_stmt(&mut self, stmt: &IfStmt) -> Result<(), LoxError> {
        let condition = self.evaluate(&stmt.condition)?;
        if self.is_truthy(condition) {
            self.execute(&stmt.then_branch)
        } else if let Some(else_branch) = &stmt.else_branch {
            self.execute(else_branch)
        } else {
            Ok(())
        }
    }
}

impl ExprVisitor<Result<Object, LoxError>> for Interpreter {
    fn visit_binary_expr(&mut self, expr: &BinaryExpr) -> Result<Object, LoxError> {
        let left = self.evaluate(&expr.left)?;
        let right = self.evaluate(&expr.right)?;

        match expr.operator.token_type {
            TokenType::Minus => match left - right {
                Ok(result) => Ok(result),
                Err(message) => Err(LoxError::interpreter_error(expr.operator.line, &message)),
            },
            TokenType::Slash => match left / right {
                Ok(result) => Ok(result),
                Err(message) => Err(LoxError::interpreter_error(expr.operator.line, &message)),
            },
            TokenType::Star => match left * right {
                Ok(result) => Ok(result),
                Err(message) => Err(LoxError::interpreter_error(expr.operator.line, &message)),
            },
            TokenType::Plus => match left + right {
                Ok(result) => Ok(result),
                Err(message) => Err(LoxError::interpreter_error(expr.operator.line, &message)),
            },
            TokenType::Greater => match (left, right) {
                (Object::Number(left), Object::Number(right)) => Ok(Object::Bool(left > right)),
                _ => Err(LoxError::interpreter_error(
                    expr.operator.line,
                    "Operands must be numbers for '>' operation.",
                )),
            },
            TokenType::GreaterEqual => match (left, right) {
                (Object::Number(left), Object::Number(right)) => Ok(Object::Bool(left >= right)),
                _ => Err(LoxError::interpreter_error(
                    expr.operator.line,
                    "Operands must be numbers for '>=' operation.",
                )),
            },
            TokenType::Less => match (left, right) {
                (Object::Number(left), Object::Number(right)) => Ok(Object::Bool(left < right)),
                _ => Err(LoxError::interpreter_error(
                    expr.operator.line,
                    "Operands must be numbers for '<' operation.",
                )),
            },
            TokenType::LessEqual => match (left, right) {
                (Object::Number(left), Object::Number(right)) => Ok(Object::Bool(left <= right)),
                _ => Err(LoxError::interpreter_error(
                    expr.operator.line,
                    "Operands must be numbers for '<=' operation.",
                )),
            },
            TokenType::BangEqual => Ok(Object::Bool(left != right)),
            TokenType::EqualEqual => Ok(Object::Bool(left == right)),
            _ => Err(LoxError::interpreter_error(
                expr.operator.line,
                "Unsupported binary operation.",
            )),
        }
    }

    fn visit_grouping_expr(&mut self, expr: &GroupingExpr) -> Result<Object, LoxError> {
        self.evaluate(&expr.expression)
    }

    fn visit_literal_expr(&mut self, expr: &LiteralExpr) -> Result<Object, LoxError> {
        Ok(expr.value.clone())
    }

    fn visit_unary_expr(&mut self, expr: &UnaryExpr) -> Result<Object, LoxError> {
        let right = self.evaluate(&expr.right)?;

        match expr.operator.token_type {
            TokenType::Bang => Ok(Object::Bool(!self.is_truthy(right))),
            TokenType::Minus => match right {
                Object::Number(val) => Ok(Object::Number(-val)),
                _ => Err(LoxError::interpreter_error(
                    expr.operator.line,
                    "Operand must be a number.",
                )),
            },
            _ => Err(LoxError::interpreter_error(
                expr.operator.line,
                "Unsupported unary operator",
            )),
        }
    }

    fn visit_comma_expr(&mut self, expr: &CommaExpr) -> Result<Object, LoxError> {
        self.evaluate(&expr.left)?;
        self.evaluate(&expr.right)
    }

    fn visit_ternary_expr(&mut self, expr: &TernaryExpr) -> Result<Object, LoxError> {
        let condition = self.evaluate(&expr.condition)?;
        match self.is_truthy(condition) {
            true => self.evaluate(&expr.then_branch),
            false => self.evaluate(&expr.else_branch),
        }
    }

    fn visit_variable_expr(&mut self, expr: &VariableExpr) -> Result<Object, LoxError> {
        self.environment.get(&expr.name)
    }

    fn visit_assign_expr(&mut self, expr: &AssignExpr) -> Result<Object, LoxError> {
        let value = self.evaluate(&expr.value)?;
        self.environment.assign(&expr.name, value.clone())?;
        Ok(value)
    }
}

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter {
            environment: Environment::new(),
        }
    }

    pub fn interpret(&mut self, statements: &[Stmt]) -> Result<(), LoxError> {
        for statement in statements {
            self.execute(statement)?
        }
        Ok(())
    }

    fn execute(&mut self, stmt: &Stmt) -> Result<(), LoxError> {
        stmt.accept(self)
    }

    fn execute_block(
        &mut self,
        statements: &[Stmt],
        environment: Environment,
    ) -> Result<(), LoxError> {
        // Stores current env until this point
        let previous_env: Environment = environment.clone();
        // Update current env with the same one to be updated
        self.environment = environment;
        // Executes each statement until it reaches an error
        let statement_fn = |statement| self.execute(statement);
        let result: Result<(), LoxError> = statements.iter().try_for_each(statement_fn);
        // Get back the previous environment;
        self.environment = previous_env;

        result
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Object, LoxError> {
        expr.accept(self)
    }

    fn is_truthy(&mut self, value: Object) -> bool {
        match value {
            Object::Nil => false,
            Object::Bool(val) => val,
            _ => true,
        }
    }
}

#[cfg(test)]
mod interpreter_tests {
    use super::*;

    fn make_literal(obj: Object) -> Box<Expr> {
        Box::new(Expr::Literal(LiteralExpr { value: obj }))
    }

    fn make_literal_number(num: f64) -> Box<Expr> {
        Box::new(Expr::Literal(LiteralExpr {
            value: Object::Number(num),
        }))
    }

    fn make_literal_string(str_val: &str) -> Box<Expr> {
        Box::new(Expr::Literal(LiteralExpr {
            value: Object::String(str_val.to_string()),
        }))
    }

    fn make_literal_bool(value: bool) -> Box<Expr> {
        Box::new(Expr::Literal(LiteralExpr {
            value: Object::Bool(value),
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
            } else if let Some(err) = result.err() {
                assert!(err.message.contains(&token.lexeme), "{}", &message_for_err);
                assert!(
                    err.message.contains("Operands must be"),
                    "{}",
                    &message_for_err
                );
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
            })),
            then_branch: make_literal_string("Ohhh yeaahhh!"),
            else_branch: make_literal_string(":c"),
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
    fn test_comma_operator() {
        // Arrange
        let mut interpreter = Interpreter::new();
        let ternary: CommaExpr = CommaExpr {
            left: make_literal_number(69.0),
            right: make_literal_string("Ohhh yeaahhh!"),
        };

        // Act
        let result = interpreter.visit_comma_expr(&ternary);
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
        };
        let unary_expr_2 = UnaryExpr {
            operator: make_token_operator(TokenType::Minus, "-"),
            right: make_literal_string("Coffee"),
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
            initializer,
        };

        // Act
        let result = interpreter.visit_var_stmt(&var_stmt);
        // Assert
        assert!(result.is_ok());
        assert!(interpreter.environment.get(&name).is_ok());
    }

    #[test]
    fn test_var_statement_nil() {
        // Arrange
        let mut interpreter = Interpreter::new();
        let initializer = make_literal(Object::Nil);
        let name = make_token_identifier("my_variable");
        let var_stmt = VarStmt {
            name: name.clone(),
            initializer,
        };

        // Act
        let result = interpreter.visit_var_stmt(&var_stmt);
        // Assert
        assert!(result.is_ok());
        assert!(interpreter.environment.get(&name).is_ok());
        assert_eq!(interpreter.environment.get(&name).ok(), Some(Object::Nil));
    }

    #[test]
    fn test_var_expression_defined() {
        // Arrange
        let mut interpreter = Interpreter::new();
        let name = make_token_identifier("my_variable");
        let initializer = make_literal_number(123.0);
        let var_stmt = VarStmt {
            name: name.clone(),
            initializer,
        };
        let var_expr = VariableExpr { name: name.clone() };

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
        let var_expr = VariableExpr { name };

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
            initializer,
        };

        let value = make_literal_number(321.0);
        let assign_expr = AssignExpr { name, value };

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
        let assign_expr = AssignExpr { name, value };

        // Act
        let result = interpreter.visit_assign_expr(&assign_expr);
        // Assert
        assert!(result.is_err());
    }
}
