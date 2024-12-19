use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone)]
pub struct LoxClass {
    pub name: String,
}

impl LoxClass {
    pub fn new(name: String) -> LoxClass {
        LoxClass { name }
    }
}

impl Display for LoxClass {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "<class {}>", self.name)
    }
}
