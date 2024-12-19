use crate::{error::*, interpreter::Interpreter, object::Object, token::Token};

pub trait LoxCallable {
    fn arity(&self) -> usize;
    fn call(
        &mut self,
        interpreter: &mut Interpreter,
        arguments: Vec<Object>,
    ) -> Result<Object, LoxErrorResult>;
    fn check_arity(
        &self,
        arguments_len: usize,
        current_token: &Token,
    ) -> Result<(), LoxErrorResult>;
}
