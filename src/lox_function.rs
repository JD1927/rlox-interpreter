use std::fmt::{self, Display, Formatter};
use std::rc::Rc;

use crate::lox_instance::*;
use crate::{
    environment::*, error::*, interpreter::*, lox_callable::*, object::*, stmt::*, token::*,
};

#[derive(Debug, Clone)]
pub struct LoxFunction {
    declaration: Box<FunctionStmt>,
    closure: EnvironmentRef,
}

impl LoxFunction {
    pub fn new(declaration: &FunctionStmt, closure: EnvironmentRef) -> LoxFunction {
        LoxFunction {
            declaration: Box::new(declaration.clone()),
            closure,
        }
    }

    pub fn bind(&self, instance: LoxInstanceRef) -> LoxFunction {
        let environment = Environment::new_enclosing(self.closure.clone());

        environment
            .borrow_mut()
            .define("this".to_string(), Object::ClassInstance(instance));

        LoxFunction {
            declaration: self.declaration.clone(),
            closure: environment,
        }
    }
}

impl LoxCallable for LoxFunction {
    fn call(
        &mut self,
        interpreter: &mut Interpreter,
        arguments: Vec<Object>,
    ) -> Result<Object, LoxErrorResult> {
        let environment = Environment::new_enclosing(Rc::clone(&self.closure));
        for (idx, param) in self.declaration.params.iter().enumerate() {
            environment
                .borrow_mut()
                .define(param.lexeme.clone(), arguments[idx].clone());
        }
        match interpreter.execute_block(&self.declaration.body, environment) {
            Ok(_) => Ok(Object::Nil),
            Err(LoxErrorResult::ControlFlowReturn { value }) => Ok(value),
            Err(err) => Err(err),
        }
    }

    fn arity(&self) -> usize {
        self.declaration.params.len()
    }

    fn check_arity(&self, args_len: usize, current_token: &Token) -> Result<(), LoxErrorResult> {
        if args_len != self.arity() {
            return Err(LoxErrorResult::interpreter_error(
                current_token.line,
                &format!("Expected {} arguments but got {}.", self.arity(), args_len),
            ));
        }
        Ok(())
    }
}

impl Display for LoxFunction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "<fun {}>", &self.declaration.name.lexeme)
    }
}
