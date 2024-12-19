use crate::{error::LoxErrorResult, lox_class::LoxClass, object::Object, token::Token};
use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::{self, Display, Formatter},
    rc::Rc,
};

#[derive(Debug, Clone)]
pub struct LoxInstance {
    lox_class: LoxClass,
    fields: HashMap<String, Object>,
}

pub type LoxInstanceRef = Rc<RefCell<LoxInstance>>;

impl LoxInstance {
    pub fn new(lox_class: LoxClass) -> LoxInstanceRef {
        Rc::new(RefCell::new(LoxInstance {
            lox_class,
            fields: HashMap::new(),
        }))
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

    pub fn set(&mut self, name: &Token, value: Object) {
        self.fields.insert(name.lexeme(), value);
    }
}

impl Display for LoxInstance {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "<class {} instance>", &self.lox_class.name)
    }
}
