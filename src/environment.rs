use std::collections::HashMap;

use crate::{error::*, object::*, token::*};

pub struct Environment {
    values: HashMap<String, Object>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            values: HashMap::new(),
        }
    }

    pub fn define(&mut self, name: String, value: Object) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Result<Object, LoxError> {
        if let Some(value) = self.values.get(&name.lexeme) {
            Ok(value.clone())
        } else {
            Err(LoxError::interpreter_error(
                name.line,
                &format!("Undefined variable '{}'.", name.lexeme),
            ))
        }
    }

    pub fn assign(&mut self, name: &Token, value: Object) -> Result<Object, LoxError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value);
            return Ok(Object::Nil);
        }
        Err(LoxError::interpreter_error(
            name.line,
            &format!("Undefined variable '{}'.", name.lexeme),
        ))
    }
}

#[cfg(test)]
mod environment_test {
    use crate::token::TokenType;

    use super::*;

    fn make_token_identifier(identifier: &str) -> Token {
        Token::new(
            TokenType::Identifier,
            identifier.to_string(),
            Object::Nil,
            1,
        )
    }

    #[test]
    fn test_can_define_a_variable() {
        // Arrange
        let mut env = Environment::new();
        // Act
        env.define("my_variable".to_string(), Object::Number(123.0));
        // Assert
        assert!(env.values.contains_key("my_variable"));
        assert_eq!(env.values.get("my_variable"), Some(&Object::Number(123.0)));
    }

    #[test]
    fn test_can_redefine_a_variable() {
        // Arrange
        let mut env = Environment::new();
        // Act
        env.define("my_variable".to_string(), Object::Number(123.0));
        env.define("my_variable".to_string(), Object::Bool(true));
        // Assert
        assert!(env.values.contains_key("my_variable"));
        assert_eq!(env.values.get("my_variable"), Some(&Object::Bool(true)));
    }

    #[test]
    fn test_can_get_variable() {
        // Arrange
        let mut env = Environment::new();
        env.define("my_variable".to_string(), Object::Number(123.0));
        let token = make_token_identifier("my_variable");
        // Act
        let result = env.get(&token);
        // Assert
        assert!(result.is_ok());
        assert_eq!(result.ok().unwrap(), Object::Number(123.0));
    }

    #[test]
    fn test_cannot_get_variable() {
        // Arrange
        let env = Environment::new();
        let token = make_token_identifier("my_variable");
        // Act
        let result = env.get(&token);
        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn test_can_assign_value_to_variable() {
        // Arrange
        let mut env = Environment::new();
        env.define("my_variable".to_string(), Object::Number(123.0));
        let token = make_token_identifier("my_variable");
        // Act
        let result = env.assign(&token, Object::Bool(true));
        // Assert
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Nil));
        assert_eq!(env.get(&token).unwrap(), Object::Bool(true));
    }
    #[test]
    fn test_cannot_assign_value_to_undefined_variable() {
        // Arrange
        let mut env = Environment::new();
        let token = make_token_identifier("my_variable");
        // Act
        let result = env.assign(&token, Object::Bool(true));
        // Assert
        assert!(result.is_err());
    }
}
