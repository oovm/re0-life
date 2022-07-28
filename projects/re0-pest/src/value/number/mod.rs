use crate::value::Atom;
use std::fmt::{Debug, Formatter};

mod time;

#[derive(Clone)]
pub struct NumberLiteral {
    value: Atom,
    suffix: String,
}

impl Debug for NumberLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.value, self.suffix)
    }
}

impl NumberLiteral {
    pub fn new<S>(value: Atom, suffix: S) -> Self
    where
        S: Into<String>,
    {
        Self { value, suffix: suffix.into() }
    }

    pub fn get_i64(&self) -> i64 {
        match self.value {
            Atom::Symbol(_) => unreachable!(),
            Atom::Integer(n) => n,
            Atom::Decimal(n) => n as i64,
        }
    }
    pub fn get_f64(&self) -> f64 {
        match self.value {
            Atom::Symbol(_) => unreachable!(),
            Atom::Integer(n) => n as f64,
            Atom::Decimal(n) => n,
        }
    }
    pub fn get_f32(&self) -> f32 {
        match self.value {
            Atom::Symbol(_) => unreachable!(),
            Atom::Integer(n) => n as f32,
            Atom::Decimal(n) => n as f32,
        }
    }
    pub fn get_unit(&self) -> &str {
        self.suffix.as_str()
    }
}
