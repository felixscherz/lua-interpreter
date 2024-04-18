use std::fmt;

use crate::vm::ExeState;

#[derive(Clone, PartialEq)]
pub enum Value {
    Nil,
    String(String),
    Function(fn(&mut ExeState) -> i32),
    Integer(i64),
    Float(f64),
    Boolean(bool),
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            Value::Nil => write!(f, "nil"),
            Value::String(s) => write!(f, "{s}"),
            Value::Function(_) => write!(f, "function"),
            Value::Integer(i) => write!(f, "{i}"),
            Value::Float(v) => write!(f, "{v:?}"),
            Value::Boolean(b) => write!(f, "{b}"),
        }
    }
}
