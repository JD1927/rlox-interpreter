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
            _ => Err(LoxError::interpreter_error(
                expr.operator.line,
                "Unsupported binary operator",
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
                _ => Ok(Object::Nil),
            },
            _ => Err(LoxError::interpreter_error(
                expr.operator.line,
                "Unsupported unary operator",
            )),
        }
    }

    fn visit_comma_expr(&mut self, expr: &CommaExpr) -> Result<Object, LoxError> {
        todo!()
    }

    fn visit_ternary_expr(&mut self, expr: &TernaryExpr) -> Result<Object, LoxError> {
        todo!()
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

    fn make_literal_string(str_val: String) -> Box<Expr> {
        Box::new(Expr::Literal(LiteralExpr {
            value: Object::String(str_val),
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
            left: make_literal_string("Hello, ".to_string()),
            operator: make_token_operator(TokenType::Plus, "+"),
            right: make_literal_string("Rust".to_string()),
        };

        // Act
        let result = interpreter.visit_binary_expr(&binary_expr);
        // Assert
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::String("Hello, Rust".to_string())));
    }

    #[test]
    fn test_unary_minus() {
        // Arrange
        let mut interpreter = Interpreter::new();
        let unary_expr = UnaryExpr {
            operator: make_token_operator(TokenType::Minus, "-"),
            right: make_literal_number(123.0),
        };

        // Act
        let result = interpreter.visit_unary_expr(&unary_expr);
        // Assert
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Number(-123.0)));
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
}
