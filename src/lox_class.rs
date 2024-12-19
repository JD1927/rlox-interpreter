use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
};

use crate::{
    error::LoxErrorResult, interpreter::Interpreter, lox_callable::LoxCallable,
    lox_function::LoxFunction, lox_instance::LoxInstance, object::Object, token::Token,
};

#[derive(Debug, Clone)]
pub struct LoxClass {
    pub name: String,
    pub methods: HashMap<String, LoxFunction>,
}

impl LoxClass {
    pub fn new(name: String, methods: HashMap<String, LoxFunction>) -> LoxClass {
        LoxClass { name, methods }
    }
    pub fn find_method(&self, name: &str) -> Option<LoxFunction> {
        self.methods.get(name).cloned()
    }
}

impl Display for LoxClass {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "<class {}>", self.name)
    }
}

impl LoxCallable for LoxClass {
    fn arity(&self) -> usize {
        0
    }

    fn call(
        &mut self,
        _interpreter: &mut Interpreter,
        _arguments: Vec<Object>,
    ) -> Result<Object, LoxErrorResult> {
        Ok(Object::ClassInstance(LoxInstance::new(self.clone())))
    }

    fn check_arity(&self, args_len: usize, current_token: &Token) -> Result<(), LoxErrorResult> {
        if args_len != self.arity() {
            return Err(LoxErrorResult::interpreter_error(
                current_token.line,
                &format!(
                    "Expected {} arguments in class initializer but got {}.",
                    self.arity(),
                    args_len
                ),
            ));
        }
        Ok(())
    }
}
