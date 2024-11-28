use std::collections::HashMap;

use crate::{error::*, object::*, token::*};

#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    values: HashMap<String, Object>,
    enclosing: Option<Box<Environment>>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            values: HashMap::new(),
            enclosing: None,
        }
    }

    pub fn new_enclosing(enclosing: Environment) -> Self {
        Environment {
            values: HashMap::new(),
            enclosing: Some(Box::new(enclosing)),
        }
    }

    pub fn define(&mut self, name: String, value: Object) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Result<Object, LoxError> {
        if let Some(value) = self.values.get(&name.lexeme) {
            return Ok(value.clone());
        }

        if let Some(env) = &self.enclosing {
            return env.get(name);
        }

        Err(LoxError::interpreter_error(
            name.line,
            &format!("Undefined variable '{}'.", name.lexeme),
        ))
    }

    pub fn assign(&mut self, name: &Token, value: Object) -> Result<Object, LoxError> {
        if self.values.contains_key(&name.lexeme) {
            self.values.insert(name.lexeme.clone(), value);
            return Ok(Object::Nil);
        }

        if let Some(env) = self.enclosing.as_mut() {
            return env.assign(name, value);
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

    #[test]
    fn test_can_enclose_an_environment() {
        // Arrange
        let enclosing = Environment::new();
        let env = Environment::new_enclosing(enclosing.clone());
        // Act
        // Assert
        assert_eq!(env.enclosing.clone().unwrap(), Box::new(enclosing.clone()));
        assert_eq!(
            env.enclosing.clone().unwrap().values,
            Box::new(enclosing.clone()).values
        );
    }

    #[test]
    fn test_can_read_from_enclosed_environment() {
        // Arrange
        let mut enclosing = Environment::new();
        enclosing.define("my_variable".to_string(), Object::Number(123.0));
        let token = make_token_identifier("my_variable");
        let env = Environment::new_enclosing(enclosing.clone());
        // Act
        let result = env.get(&token);
        // Assert
        assert!(result.is_ok(), "Expected 'Object' but got 'LoxError'.");
        assert_eq!(result.ok().unwrap(), Object::Number(123.0));
    }

    #[test]
    fn test_cannot_read_from_enclosed_environment() {
        // Arrange
        let enclosing = Environment::new();
        let token = make_token_identifier("my_variable");
        let env = Environment::new_enclosing(enclosing.clone());
        // Act
        let result = env.get(&token);
        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn test_can_assign_value_to_variable_in_enclosing_environment() {
        // Arrange
        let token = make_token_identifier("my_variable");
        let mut enclosing = Environment::new();
        enclosing.define("my_variable".to_string(), Object::Number(123.0));
        let mut env = Environment::new_enclosing(enclosing);
        // Act
        let result = env.assign(&token, Object::Bool(true));
        // Assert
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Nil));
        assert_eq!(env.get(&token).unwrap(), Object::Bool(true));
        assert_eq!(
            env.enclosing.unwrap().get(&token).unwrap(),
            Object::Bool(true),
        );
    }
}
