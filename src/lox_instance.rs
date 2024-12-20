use crate::{error::LoxErrorResult, lox_class::LoxClass, object::Object, token::Token};
use std::{
    cell::RefCell,
    collections::HashMap,
    fmt::{self, Display, Formatter},
    rc::Rc,
};

#[derive(Debug, Clone)]
pub struct LoxInstance {
    class: LoxClass,
    fields: HashMap<String, Object>,
}

pub type LoxInstanceRef = Rc<RefCell<LoxInstance>>;

impl LoxInstance {
    pub fn new(lox_class: LoxClass) -> LoxInstanceRef {
        Rc::new(RefCell::new(LoxInstance {
            class: lox_class,
            fields: HashMap::new(),
        }))
    }

    pub fn get(&self, name: &Token) -> Result<Object, LoxErrorResult> {
        if let Some(result) = self.fields.get(&name.lexeme) {
            return Ok(result.clone());
        }

        if let Some(function) = self.class.find_method(&name.lexeme) {
            return Ok(Object::Function(function));
        }

        Err(LoxErrorResult::interpreter_error(
            name.line,
            &format!("Undefined property '{}'.", name.lexeme),
        ))
    }

    pub fn set(&mut self, name: &Token, value: Object) {
        self.fields.insert(name.lexeme(), value);
    }
}

impl Display for LoxInstance {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        let mut fields = Vec::new();
        for field in self.fields.keys() {
            fields.push(field.as_str());
        }
        let mut methods = Vec::new();
        for method in self.class.methods.keys() {
            methods.push(method.as_str());
        }
        write!(
            f,
            "<{} instance> {{ props: {{ {} }}, methods: {{ {} }} }}>",
            &self.class.name,
            fields.join(", "),
            methods.join(", ")
        )
    }
}
