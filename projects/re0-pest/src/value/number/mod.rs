use std::fmt::{Debug, Formatter};

use crate::ast::{ASTKind, ASTNode};
use crate::value::Atom;

mod time;

impl ASTNode {
    pub fn atomic<N>(input: N) -> Self
    where
        N: Into<Atom>,
    {
        Self { kind: ASTKind::Value(input.into()) }
    }
    pub fn integer<S>(input: i64, suffix: S) -> Self
    where
        S: Into<String>,
    {
        let n = Atom::Integer(input, suffix.into());
        Self { kind: ASTKind::Value(n) }
    }
}

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
        self.value.as_i64()
    }
    pub fn get_f64(&self) -> f64 {
        self.value.as_f64()
    }
    pub fn get_f32(&self) -> f32 {
        self.value.as_f64() as f32
    }
    pub fn get_unit(&self) -> &str {
        self.suffix.as_str()
    }
}
