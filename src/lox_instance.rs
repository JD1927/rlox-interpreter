use crate::lox_class::LoxClass;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone)]
pub struct LoxInstance {
    lox_class: LoxClass,
}

impl LoxInstance {
    pub fn new(lox_class: LoxClass) -> LoxInstance {
        LoxInstance { lox_class }
    }
}

impl Display for LoxInstance {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "<class {} instance>", &self.lox_class.name)
    }
}
