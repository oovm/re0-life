use crate::ast::{ASTKind, ASTNode};
use crate::value::Value;

mod time;

impl From<i64> for Value {
    fn from(n: i64) -> Self {
        Self::Integer(n, String::new())
    }
}

impl From<f64> for Value {
    fn from(n: f64) -> Self {
        Self::Decimal(n, String::new())
    }
}

impl Value {
    pub fn as_unit(&self) -> &str {
        match self {
            Self::Integer(_, v) => v.as_str(),
            Self::Decimal(_, v) => v.as_str(),
            _ => unreachable!(),
        }
    }
    pub fn as_i64(&self) -> i64 {
        match self {
            Self::Integer(n, _) => *n as i64,
            Self::Decimal(n, _) => *n as i64,
            Self::Boolean(n) => match *n {
                true => 0,
                false => -1,
            },
            _ => unreachable!(),
        }
    }
    pub fn as_f64(&self) -> f64 {
        match self {
            Self::Integer(n, _) => *n as f64,
            Self::Decimal(n, _) => *n as f64,
            Self::Boolean(n) => match *n {
                true => 0.0,
                false => -1.0,
            },
            _ => unreachable!(),
        }
    }
}

impl ASTNode {
    pub fn atomic<N>(input: N) -> Self
    where
        N: Into<Value>,
    {
        Self { kind: ASTKind::Value(input.into()) }
    }
    pub fn integer<S>(input: i64, suffix: S) -> Self
    where
        S: Into<String>,
    {
        let n = Value::Integer(input, suffix.into());
        Self { kind: ASTKind::Value(n) }
    }
}
