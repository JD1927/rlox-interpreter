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
    pub super_class: Option<Box<LoxClass>>,
    pub methods: HashMap<String, LoxFunction>,
}

impl LoxClass {
    pub fn new(
        name: String,
        super_class: Option<Box<LoxClass>>,
        methods: HashMap<String, LoxFunction>,
    ) -> LoxClass {
        LoxClass {
            name,
            super_class,
            methods,
        }
    }
    pub fn find_method(&self, name: &str) -> Option<LoxFunction> {
        self.methods.get(name).cloned()
    }
}

impl Display for LoxClass {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut methods = Vec::new();
        for method in self.methods.keys() {
            methods.push(method.as_str());
        }
        write!(
            f,
            "<class {}> {{ methods: {{ {} }} }}>",
            self.name,
            methods.join(", ")
        )
    }
}

impl LoxCallable for LoxClass {
    fn arity(&self) -> usize {
        if let Some(initializer) = self.find_method("init") {
            initializer.arity()
        } else {
            0
        }
    }

    fn call(
        &mut self,
        interpreter: &mut Interpreter,
        arguments: Vec<Object>,
    ) -> Result<Object, LoxErrorResult> {
        let instance = LoxInstance::new(self.clone());
        if let Some(initializer) = self.find_method("init") {
            initializer
                .bind(instance.clone())
                .call(interpreter, arguments)?;
        }
        Ok(Object::ClassInstance(instance))
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
