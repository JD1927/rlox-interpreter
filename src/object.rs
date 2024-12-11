use std::{cmp::Ordering, fmt, ops::*};

use crate::{lox_function::Function, lox_native_function::NativeFunction};

#[derive(Debug, Clone)]
pub enum Object {
    String(String),
    Number(f64),
    Bool(bool),
    Function(Function),
    NativeFunction(NativeFunction),
    Nil,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Number(num) => write!(f, "{num}"),
            Object::String(val) => write!(f, "\"{val}\""),
            Object::Bool(val) => write!(f, "{val}"),
            Object::Nil => write!(f, "nil"),
            Object::Function(_val) => write!(f, "nil"),
            Object::NativeFunction(_native_function) => write!(f, "nil"),
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
            (Object::String(left), Object::Number(right)) => {
                Ok(Object::String(format!("{left}{right}")))
            }
            (Object::Number(left), Object::String(right)) => {
                Ok(Object::String(format!("{left}{right}")))
            }
            _ => Err("Operands must be strings or numbers for '+' operation.".to_string()),
        }
    }
}

impl PartialOrd for Object {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Object::Number(left), Object::Number(right)) => left.partial_cmp(right),
            (Object::String(left), Object::String(right)) => left.partial_cmp(right),
            _ => None,
        }
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Object::String(left), Object::String(right)) => left == right,
            (Object::Number(left), Object::Number(right)) => left == right,
            (Object::Bool(left), Object::Bool(right)) => left == right,
            (Object::Nil, Object::Nil) => true,
            _ => false,
        }
    }
}
