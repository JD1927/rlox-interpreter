use crate::{
    error::LoxError, interpreter::Interpreter, lox_callable::LoxCallable, object::Object,
    token::Token,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Function;

impl LoxCallable for Function {
    fn call(
        &mut self,
        interpreter: &mut Interpreter,
        arguments: Vec<Object>,
    ) -> Result<Object, LoxError> {
        Ok(Object::Nil)
    }

    fn arity(&self) -> usize {
        todo!()
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
}
