use crate::{error::LoxError, interpreter::Interpreter, object::Object, token::Token};

pub trait LoxCallable {
    fn arity(&self) -> usize;
    fn call(
        &mut self,
        interpreter: &mut Interpreter,
        arguments: Vec<Object>,
    ) -> Result<Object, LoxError>;
    fn check_arity(&self, arguments_len: usize, current_token: &Token) -> Result<(), LoxError>;
    fn to_string(&self) -> String;
}
