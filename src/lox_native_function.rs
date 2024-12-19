use std::fmt::{self, Display, Formatter};

use crate::{
    error::*, interpreter::Interpreter, lox_callable::LoxCallable, object::Object, token::Token,
};

#[derive(Debug, Clone, PartialEq)]
pub struct LoxNativeFunction {
    pub name: String,
    pub arity: usize,
    pub callable: fn(&mut Interpreter, Vec<Object>) -> Result<Object, LoxErrorResult>,
}

impl LoxCallable for LoxNativeFunction {
    fn call(
        &mut self,
        interpreter: &mut Interpreter,
        arguments: Vec<Object>,
    ) -> Result<Object, LoxErrorResult> {
        (self.callable)(interpreter, arguments)
    }

    fn arity(&self) -> usize {
        0
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

impl Display for LoxNativeFunction {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "<fn native {}>", self.name)
    }
}
