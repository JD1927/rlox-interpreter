use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{error::*, object::*, token::*};

pub type EnvironmentRef = Rc<RefCell<Environment>>;

#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
    values: HashMap<String, Object>,
    pub enclosing: Option<EnvironmentRef>,
}

impl Environment {
    pub fn new() -> EnvironmentRef {
        Rc::new(RefCell::new(Environment {
            values: HashMap::new(),
            enclosing: None,
        }))
    }

    pub fn new_enclosing(enclosing: EnvironmentRef) -> EnvironmentRef {
        Rc::new(RefCell::new(Environment {
            values: HashMap::new(),
            enclosing: Some(enclosing),
        }))
    }

    pub fn define(&mut self, name: String, value: Object) {
        self.values.insert(name, value);
    }

    pub fn get(&self, name: &Token) -> Result<Object, LoxErrorResult> {
        if let Some(value) = self.values.get(&name.lexeme) {
            Ok(value.clone())
        } else if let Some(enclosing) = &self.enclosing {
            return enclosing.borrow().get(name);
        } else {
            Err(LoxErrorResult::interpreter_error(
                name.line,
                &format!("Undefined variable '{}'.", name.lexeme),
            ))
        }
    }

    pub fn assign(&mut self, name: &Token, value: Object) -> Result<Object, LoxErrorResult> {
        if self.values.contains_key(&name.lexeme) {
            self.define(name.lexeme(), value);
            return Ok(Object::Nil);
        }

        if let Some(env) = self.enclosing.as_mut() {
            return env.borrow_mut().assign(name, value);
        }

        Err(LoxErrorResult::interpreter_error(
            name.line,
            &format!("Undefined variable '{}'.", name.lexeme),
        ))
    }

    /// Gets the enclosing environment recursively until it hits the last enclosing one
    pub fn get_at(&self, distance: usize, name: &Token) -> Result<Object, LoxErrorResult> {
        // Base case
        if distance == 0 {
            return self.get(name);
        }
        if let Some(enclosing) = &self.enclosing {
            // Recursion
            return enclosing.borrow_mut().get_at(distance - 1, name);
        }
        panic!("Could not find local scope that variable belongs to!")
    }

    pub fn assign_at(&mut self, distance: usize, name: &Token, value: &Object) {
        if distance == 0 {
            self.define(name.lexeme(), value.clone());
            return;
        }
        if let Some(enclosing) = &self.enclosing {
            // Recursion
            enclosing.borrow_mut().assign_at(distance - 1, name, value);
            return;
        }
        panic!("Could not find local scope that variable belongs to!")
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
        let env = Environment::new();
        // Act
        env.borrow_mut()
            .define("my_variable".to_string(), Object::Number(123.0));
        // Assert
        assert!(env.borrow_mut().values.contains_key("my_variable"));
        assert_eq!(
            env.borrow_mut().values.get("my_variable"),
            Some(&Object::Number(123.0))
        );
    }

    #[test]
    fn test_can_redefine_a_variable() {
        // Arrange
        let env = Environment::new();
        // Act
        env.borrow_mut()
            .define("my_variable".to_string(), Object::Number(123.0));
        env.borrow_mut()
            .define("my_variable".to_string(), Object::Bool(true));
        // Assert
        assert!(env.borrow_mut().values.contains_key("my_variable"));
        assert_eq!(
            env.borrow_mut().values.get("my_variable"),
            Some(&Object::Bool(true))
        );
    }

    #[test]
    fn test_can_get_variable() {
        // Arrange
        let env = Environment::new();
        env.borrow_mut()
            .define("my_variable".to_string(), Object::Number(123.0));
        let token = make_token_identifier("my_variable");
        // Act
        let result = env.borrow_mut().get(&token);
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
        let result = env.borrow_mut().get(&token);
        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn test_can_assign_value_to_variable() {
        // Arrange
        let env = Environment::new();
        env.borrow_mut()
            .define("my_variable".to_string(), Object::Number(123.0));
        let token = make_token_identifier("my_variable");
        // Act
        let result = env.borrow_mut().assign(&token, Object::Bool(true));
        // Assert
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Nil));
        assert_eq!(env.borrow_mut().get(&token).unwrap(), Object::Bool(true));
    }

    #[test]
    fn test_cannot_assign_value_to_undefined_variable() {
        // Arrange
        let env = Environment::new();
        let token = make_token_identifier("my_variable");
        // Act
        let result = env.borrow_mut().assign(&token, Object::Bool(true));
        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn test_can_enclose_an_environment() {
        // Arrange
        let enclosing = Environment::new();
        let env = Environment::new_enclosing(Rc::clone(&enclosing));
        // Act
        // Assert
        assert_eq!(
            env.borrow_mut().enclosing.clone().unwrap().borrow().values,
            enclosing.borrow().values
        );
    }

    #[test]
    fn test_can_read_from_enclosed_environment() {
        // Arrange
        let enclosing = Environment::new();
        enclosing
            .borrow_mut()
            .define("my_variable".to_string(), Object::Number(123.0));
        let token = make_token_identifier("my_variable");
        let env = Environment::new_enclosing(Rc::clone(&enclosing));
        // Act
        let result = env.borrow_mut().get(&token);
        // Assert
        assert!(
            result.is_ok(),
            "Expected 'Object' but got 'LoxErrorResult'."
        );
        assert_eq!(result.ok().unwrap(), Object::Number(123.0));
    }

    #[test]
    fn test_cannot_read_from_enclosed_environment() {
        // Arrange
        let enclosing = Environment::new();
        let token = make_token_identifier("my_variable");
        let env = Environment::new_enclosing(Rc::clone(&enclosing));
        // Act
        let result = env.borrow_mut().get(&token);
        // Assert
        assert!(result.is_err());
    }

    #[test]
    fn test_can_assign_value_to_variable_in_enclosing_environment() {
        // Arrange
        let token = make_token_identifier("my_variable");
        let enclosing = Environment::new();
        enclosing
            .borrow_mut()
            .define("my_variable".to_string(), Object::Number(123.0));
        let env = Environment::new_enclosing(Rc::clone(&enclosing));
        // Act
        let result = env.borrow_mut().assign(&token, Object::Bool(true));
        // Assert
        assert!(result.is_ok());
        assert_eq!(result.ok(), Some(Object::Nil));
        assert_eq!(env.borrow_mut().get(&token).unwrap(), Object::Bool(true));
        assert_eq!(
            env.borrow_mut()
                .enclosing
                .clone()
                .unwrap()
                .borrow_mut()
                .get(&token)
                .unwrap(),
            Object::Bool(true),
        );
    }
}
