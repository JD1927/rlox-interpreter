use crate::{
    error::LoxError, interpreter::Interpreter, lox_callable::LoxCallable, object::Object,
    token::Token,
};

#[derive(Debug, Clone, PartialEq)]
pub struct NativeFunction {
    pub name: String,
    pub arity: usize,
    pub callable: fn(&mut Interpreter, Vec<Object>) -> Result<Object, LoxError>,
}

impl LoxCallable for NativeFunction {
    fn call(
        &mut self,
        interpreter: &mut Interpreter,
        arguments: Vec<Object>,
    ) -> Result<Object, LoxError> {
        (self.callable)(interpreter, arguments)
    }

    fn arity(&self) -> usize {
        0
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
        format!("<fn native {}>", &self.name)
    }
}
