use std::{fmt, ops::*};

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    String(String),
    Number(f64),
    Bool(bool),
    Nil,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Number(num) => write!(f, "{num}"),
            Object::String(val) => write!(f, "\"{val}\""),
            Object::Bool(val) => write!(f, "{val}"),
            Object::Nil => write!(f, "nil"),
        }
    }
}

impl Sub for Object {
    type Output = Result<Object, String>;

    fn sub(self, other: Self) -> Self::Output {
        match (self, other) {
            (Object::Number(left), Object::Number(right)) => Ok(Object::Number(left - right)),
            _ => Err("Operands must be numbers for '-' operation.".to_string()),
        }
    }
}
impl Div for Object {
    type Output = Result<Object, String>;

    fn div(self, other: Self) -> Self::Output {
        match (self, other) {
            (Object::Number(left), Object::Number(right)) => {
                let result = left / right;
                match result.is_infinite() || result.is_nan() {
                    true => Err("Illegal expression. Division by zero is not allowed.".to_string()),
                    false => Ok(Object::Number(result)),
                }
            }
            _ => Err("Operands must be numbers for '/' operation.".to_string()),
        }
    }
}
impl Mul for Object {
    type Output = Result<Object, String>;

    fn mul(self, other: Self) -> Self::Output {
        match (self, other) {
            (Object::Number(left), Object::Number(right)) => Ok(Object::Number(left * right)),
            _ => Err("Operands must be numbers for '*' operation.".to_string()),
        }
    }
}

impl Add for Object {
    type Output = Result<Object, String>;

    fn add(self, other: Self) -> Self::Output {
        match (self, other) {
            (Object::Number(left), Object::Number(right)) => Ok(Object::Number(left + right)),
            (Object::String(left), Object::String(right)) => {
                Ok(Object::String(format!("{left}{right}")))
            }
            _ => Err("Operands must be numbers or strings for '+' operation.".to_string()),
        }
    }
}