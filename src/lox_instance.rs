use crate::{error::LoxErrorResult, lox_class::LoxClass, object::Object, token::Token};
use std::{
    collections::HashMap,
    fmt::{self, Display, Formatter},
};

#[derive(Debug, Clone)]
pub struct LoxInstance {
    lox_class: LoxClass,
    fields: HashMap<String, Object>,
}

impl LoxInstance {
    pub fn new(lox_class: LoxClass) -> LoxInstance {
        LoxInstance {
            lox_class,
            fields: HashMap::new(),
        }
    }

    pub fn get(&self, name: &Token) -> Result<Object, LoxErrorResult> {
        match self.fields.get(&name.lexeme) {
            Some(result) => Ok(result.clone()),
            None => Err(LoxErrorResult::interpreter_error(
                name.line,
                &format!("Undefined property '{}'.", name.lexeme),
            )),
        }
    }
}

impl Display for LoxInstance {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "<class {} instance>", &self.lox_class.name)
    }
}
