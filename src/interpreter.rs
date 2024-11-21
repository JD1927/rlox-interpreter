use crate::{error::LoxError, expr::*, object::*, token::*};

pub struct Interpreter;

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
}

impl Interpreter {
    fn new() -> Interpreter {
        Interpreter {}
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

    fn make_literal_nil() -> Box<Expr> {
        Box::new(Expr::Literal(LiteralExpr { value: Object::Nil }))
    }

    fn make_token_operator(token_type: TokenType, operator: &str) -> Token {
        Token::new(token_type, operator.to_string(), Object::Nil, 1)
    }

    #[test]
    fn test_subtraction() {
        // Arrange
        let mut interpreter = Interpreter::new();
        let binary_expr = BinaryExpr {
            left: make_literal_number(69.0),
            operator: make_token_operator(TokenType::Minus, "-"),
            right: make_literal_number(71.0),
        };

        // Act
        let result = interpreter.visit_binary_expr(&binary_expr);
        // Assert
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Number(-2.0)));
    }

    #[test]
    fn test_division() {
        // Arrange
        let mut interpreter = Interpreter::new();
        let binary_expr = BinaryExpr {
            left: make_literal_number(12.0),
            operator: make_token_operator(TokenType::Slash, "/"),
            right: make_literal_number(4.0),
        };

        // Act
        let result = interpreter.visit_binary_expr(&binary_expr);
        // Assert
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Number(3.0)));
    }

    #[test]
    fn test_multiplication() {
        // Arrange
        let mut interpreter = Interpreter::new();
        let binary_expr = BinaryExpr {
            left: make_literal_number(12.0),
            operator: make_token_operator(TokenType::Star, "*"),
            right: make_literal_number(4.0),
        };

        // Act
        let result = interpreter.visit_binary_expr(&binary_expr);
        // Assert
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Number(48.0)));
    }

    #[test]
    fn test_addition() {
        // Arrange
        let mut interpreter = Interpreter::new();
        let binary_expr = BinaryExpr {
            left: make_literal_number(12.0),
            operator: make_token_operator(TokenType::Plus, "+"),
            right: make_literal_number(4.0),
        };

        // Act
        let result = interpreter.visit_binary_expr(&binary_expr);
        // Assert
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Number(16.0)));
    }

    #[test]
    fn test_concatenation() {
        // Arrange
        let mut interpreter = Interpreter::new();
        let binary_expr = BinaryExpr {
            left: make_literal_string("Hello, "),
            operator: make_token_operator(TokenType::Plus, "+"),
            right: make_literal_string("Rust"),
        };

        // Act
        let result = interpreter.visit_binary_expr(&binary_expr);
        // Assert
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::String("Hello, Rust".to_string())));
    }

    #[test]
    fn test_greater_than_operator() {
        // Arrange
        let mut interpreter = Interpreter::new();
        let binary_expr_1: BinaryExpr = BinaryExpr {
            left: make_literal_number(2.0),
            operator: make_token_operator(TokenType::Greater, ">"),
            right: make_literal_number(3.0),
        };
        let binary_expr_2: BinaryExpr = BinaryExpr {
            left: make_literal_number(3.0),
            operator: make_token_operator(TokenType::Greater, ">"),
            right: make_literal_number(0.0),
        };
        let binary_expr_3: BinaryExpr = BinaryExpr {
            left: make_literal_number(3.0),
            operator: make_token_operator(TokenType::Greater, ">"),
            right: make_literal_number(3.0),
        };
        let binary_expr_4: BinaryExpr = BinaryExpr {
            left: make_literal_string("3.0"),
            operator: make_token_operator(TokenType::Greater, ">"),
            right: make_literal_number(3.0),
        };

        // Act
        let result_1 = interpreter.visit_binary_expr(&binary_expr_1);
        let result_2 = interpreter.visit_binary_expr(&binary_expr_2);
        let result_3 = interpreter.visit_binary_expr(&binary_expr_3);
        let result_4 = interpreter.visit_binary_expr(&binary_expr_4);
        // Assert
        assert!(result_1.is_ok());
        assert_eq!(result_1.ok(), Some(Object::Bool(false)));

        assert!(result_2.is_ok());
        assert_eq!(result_2.ok(), Some(Object::Bool(true)));

        assert!(result_3.is_ok());
        assert_eq!(result_3.ok(), Some(Object::Bool(false)));

        assert!(result_4.is_err());
    }

    #[test]
    fn test_greater_than_or_equal_operator() {
        // Arrange
        let mut interpreter = Interpreter::new();
        let binary_expr_1: BinaryExpr = BinaryExpr {
            left: make_literal_number(2.0),
            operator: make_token_operator(TokenType::GreaterEqual, ">="),
            right: make_literal_number(3.0),
        };
        let binary_expr_2: BinaryExpr = BinaryExpr {
            left: make_literal_number(3.0),
            operator: make_token_operator(TokenType::GreaterEqual, ">="),
            right: make_literal_number(0.0),
        };
        let binary_expr_3: BinaryExpr = BinaryExpr {
            left: make_literal_number(3.0),
            operator: make_token_operator(TokenType::GreaterEqual, ">="),
            right: make_literal_number(3.0),
        };
        let binary_expr_4: BinaryExpr = BinaryExpr {
            left: make_literal_string("3.0"),
            operator: make_token_operator(TokenType::GreaterEqual, ">="),
            right: make_literal_number(3.0),
        };

        // Act
        let result_1 = interpreter.visit_binary_expr(&binary_expr_1);
        let result_2 = interpreter.visit_binary_expr(&binary_expr_2);
        let result_3 = interpreter.visit_binary_expr(&binary_expr_3);
        let result_4 = interpreter.visit_binary_expr(&binary_expr_4);
        // Assert
        assert!(result_1.is_ok());
        assert_eq!(result_1.ok(), Some(Object::Bool(false)));

        assert!(result_2.is_ok());
        assert_eq!(result_2.ok(), Some(Object::Bool(true)));

        assert!(result_3.is_ok());
        assert_eq!(result_3.ok(), Some(Object::Bool(true)));

        assert!(result_4.is_err());
    }

    #[test]
    fn test_less_than_operator() {
        // Arrange
        let mut interpreter = Interpreter::new();
        let binary_expr_1: BinaryExpr = BinaryExpr {
            left: make_literal_number(2.0),
            operator: make_token_operator(TokenType::Less, "<"),
            right: make_literal_number(3.0),
        };
        let binary_expr_2: BinaryExpr = BinaryExpr {
            left: make_literal_number(3.0),
            operator: make_token_operator(TokenType::Less, "<"),
            right: make_literal_number(0.0),
        };
        let binary_expr_3: BinaryExpr = BinaryExpr {
            left: make_literal_number(3.0),
            operator: make_token_operator(TokenType::Less, "<"),
            right: make_literal_number(3.0),
        };
        let binary_expr_4: BinaryExpr = BinaryExpr {
            left: make_literal_number(3.0),
            operator: make_token_operator(TokenType::Less, "<"),
            right: make_literal_string("3.0"),
        };

        // Act
        let result_1 = interpreter.visit_binary_expr(&binary_expr_1);
        let result_2 = interpreter.visit_binary_expr(&binary_expr_2);
        let result_3 = interpreter.visit_binary_expr(&binary_expr_3);
        let result_4 = interpreter.visit_binary_expr(&binary_expr_4);
        // Assert
        assert!(result_1.is_ok());
        assert_eq!(result_1.ok(), Some(Object::Bool(true)));

        assert!(result_2.is_ok());
        assert_eq!(result_2.ok(), Some(Object::Bool(false)));

        assert!(result_3.is_ok());
        assert_eq!(result_3.ok(), Some(Object::Bool(false)));

        assert!(result_4.is_err());
    }

    #[test]
    fn test_less_than_or_equal_operator() {
        // Arrange
        let mut interpreter = Interpreter::new();
        let binary_expr_1: BinaryExpr = BinaryExpr {
            left: make_literal_number(2.0),
            operator: make_token_operator(TokenType::LessEqual, "<="),
            right: make_literal_number(3.0),
        };
        let binary_expr_2: BinaryExpr = BinaryExpr {
            left: make_literal_number(3.0),
            operator: make_token_operator(TokenType::LessEqual, "<="),
            right: make_literal_number(0.0),
        };
        let binary_expr_3: BinaryExpr = BinaryExpr {
            left: make_literal_number(3.0),
            operator: make_token_operator(TokenType::LessEqual, "<="),
            right: make_literal_number(3.0),
        };
        let binary_expr_4: BinaryExpr = BinaryExpr {
            left: make_literal_number(3.0),
            operator: make_token_operator(TokenType::LessEqual, "<="),
            right: make_literal_string("3.0"),
        };

        // Act
        let result_1 = interpreter.visit_binary_expr(&binary_expr_1);
        let result_2 = interpreter.visit_binary_expr(&binary_expr_2);
        let result_3 = interpreter.visit_binary_expr(&binary_expr_3);
        let result_4 = interpreter.visit_binary_expr(&binary_expr_4);
        // Assert
        assert!(result_1.is_ok());
        assert_eq!(result_1.ok(), Some(Object::Bool(true)));

        assert!(result_2.is_ok());
        assert_eq!(result_2.ok(), Some(Object::Bool(false)));

        assert!(result_3.is_ok());
        assert_eq!(result_3.ok(), Some(Object::Bool(true)));

        assert!(result_4.is_ok());
    }

    #[test]
    fn test_bang_equal_operator() {
        // Arrange
        let mut interpreter = Interpreter::new();
        let binary_expr_1: BinaryExpr = BinaryExpr {
            left: make_literal_number(2.0),
            operator: make_token_operator(TokenType::BangEqual, "!="),
            right: make_literal_number(3.0),
        };
        let binary_expr_2: BinaryExpr = BinaryExpr {
            left: make_literal_number(3.0),
            operator: make_token_operator(TokenType::BangEqual, "!="),
            right: make_literal_number(3.0),
        };
        let binary_expr_3: BinaryExpr = BinaryExpr {
            left: make_literal_nil(),
            operator: make_token_operator(TokenType::BangEqual, "!="),
            right: make_literal_nil(),
        };
        let binary_expr_4: BinaryExpr = BinaryExpr {
            left: make_literal_bool(true),
            operator: make_token_operator(TokenType::BangEqual, "!="),
            right: make_literal_bool(false),
        };
        let binary_expr_5: BinaryExpr = BinaryExpr {
            left: make_literal_string("Hello"),
            operator: make_token_operator(TokenType::BangEqual, "!="),
            right: make_literal_string("World"),
        };

        // Act
        let result_1 = interpreter.visit_binary_expr(&binary_expr_1);
        let result_2 = interpreter.visit_binary_expr(&binary_expr_2);
        let result_3 = interpreter.visit_binary_expr(&binary_expr_3);
        let result_4 = interpreter.visit_binary_expr(&binary_expr_4);
        let result_5 = interpreter.visit_binary_expr(&binary_expr_5);
        // Assert
        assert!(result_1.is_ok());
        assert_eq!(result_1.ok(), Some(Object::Bool(true)));

        assert!(result_2.is_ok());
        assert_eq!(result_2.ok(), Some(Object::Bool(false)));

        assert!(result_3.is_ok());
        assert_eq!(result_3.ok(), Some(Object::Bool(false)));

        assert!(result_4.is_ok());
        assert_eq!(result_4.ok(), Some(Object::Bool(true)));

        assert!(result_5.is_ok());
        assert_eq!(result_5.ok(), Some(Object::Bool(true)));
    }

    #[test]
    fn test_equal_equal_operator() {
        // Arrange
        let mut interpreter = Interpreter::new();
        let binary_expr_1: BinaryExpr = BinaryExpr {
            left: make_literal_number(2.0),
            operator: make_token_operator(TokenType::EqualEqual, "=="),
            right: make_literal_number(3.0),
        };
        let binary_expr_2: BinaryExpr = BinaryExpr {
            left: make_literal_number(3.0),
            operator: make_token_operator(TokenType::EqualEqual, "=="),
            right: make_literal_number(3.0),
        };
        let binary_expr_3: BinaryExpr = BinaryExpr {
            left: make_literal_nil(),
            operator: make_token_operator(TokenType::EqualEqual, "=="),
            right: make_literal_nil(),
        };
        let binary_expr_4: BinaryExpr = BinaryExpr {
            left: make_literal_bool(true),
            operator: make_token_operator(TokenType::EqualEqual, "=="),
            right: make_literal_bool(false),
        };
        let binary_expr_5: BinaryExpr = BinaryExpr {
            left: make_literal_string("Hello"),
            operator: make_token_operator(TokenType::EqualEqual, "=="),
            right: make_literal_string("World"),
        };

        // Act
        let result_1 = interpreter.visit_binary_expr(&binary_expr_1);
        let result_2 = interpreter.visit_binary_expr(&binary_expr_2);
        let result_3 = interpreter.visit_binary_expr(&binary_expr_3);
        let result_4 = interpreter.visit_binary_expr(&binary_expr_4);
        let result_5 = interpreter.visit_binary_expr(&binary_expr_5);
        // Assert
        assert!(result_1.is_ok());
        assert_eq!(result_1.ok(), Some(Object::Bool(false)));

        assert!(result_2.is_ok());
        assert_eq!(result_2.ok(), Some(Object::Bool(true)));

        assert!(result_3.is_ok());
        assert_eq!(result_3.ok(), Some(Object::Bool(true)));

        assert!(result_4.is_ok());
        assert_eq!(result_4.ok(), Some(Object::Bool(false)));

        assert!(result_5.is_ok());
        assert_eq!(result_5.ok(), Some(Object::Bool(false)));
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
    fn test_arithmetic_error_for_subtraction() {
        // Arrange
        let mut interpreter = Interpreter::new();
        let binary_expr = BinaryExpr {
            left: make_literal_bool(true),
            operator: make_token_operator(TokenType::Minus, "-"),
            right: make_literal_number(71.0),
        };

        // Act
        let result = interpreter.visit_binary_expr(&binary_expr);
        // Assert
        assert!(result.is_err());
        println!("{}", result.err().unwrap());
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
}
