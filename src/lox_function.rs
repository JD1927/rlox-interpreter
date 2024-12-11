use crate::{
    environment::Environment, error::LoxError, interpreter::Interpreter, lox_callable::*,
    object::Object, stmt::*, token::Token,
};

#[derive(Debug, Clone)]
pub struct LoxFunction {
    declaration: Box<FunctionStmt>,
}

impl LoxFunction {
    pub fn new(declaration: &FunctionStmt) -> Self {
        LoxFunction {
            declaration: Box::new(declaration.clone()),
        }
    }
}

impl LoxCallable for LoxFunction {
    fn call(
        &mut self,
        interpreter: &mut Interpreter,
        arguments: Vec<Object>,
    ) -> Result<Object, LoxError> {
        let environment = Environment::new_enclosing(interpreter.globals.clone());
        for (idx, param) in self.declaration.params.iter().enumerate() {
            environment
                .borrow_mut()
                .define(param.lexeme.clone(), arguments[idx].clone());
        }
        interpreter.execute_block(&self.declaration.body, environment)?;
        Ok(Object::Nil)
    }

    fn arity(&self) -> usize {
        self.declaration.params.len()
    }

    fn check_arity(&self, args_len: usize, current_token: &Token) -> Result<(), LoxError> {
        if args_len != self.arity() {
            return Err(LoxError::interpreter_error(
                current_token.line,
                &format!("Expected {} arguments but got {}.", self.arity(), args_len),
            ));
        }
        Ok(())
    }

    fn to_string(&self) -> String {
        format!("<fn {}>", &self.declaration.name.lexeme)
    }
}
