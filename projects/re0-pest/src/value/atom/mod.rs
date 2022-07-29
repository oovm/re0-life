use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Clone)]
pub enum Value {
    Null,
    Boolean(bool),
    Symbol(String),
    String(String),
    Integer(i64, String),
    Decimal(f64, String),
}

impl Debug for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::Boolean(v) => write!(f, "{}", v),
            Value::Symbol(v) => write!(f, "{}", v),
            Value::Integer(v, s) => write!(f, "{}{}", v, s),
            Value::Decimal(v, s) => write!(f, "{}{}", v, s),
            Value::String(v) => write!(f, "{:?}", v),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl From<&str> for Value {
    fn from(key: &str) -> Self {
        Self::String(key.to_string())
    }
}

impl From<String> for Value {
    fn from(s: String) -> Self {
        Self::String(s)
    }
}

impl Hash for Value {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Value::Null => {}
            Value::Boolean(v) => state.write_u8(*v as u8),
            Value::Symbol(v) => {
                state.write_str(v);
            }
            Value::String(v) => {
                state.write_str(v);
            }
            Value::Integer(v, s) => {
                state.write_i64(*v);
                state.write_str(s);
            }
            Value::Decimal(v, s) => {
                state.write(&v.to_le_bytes());
                state.write_str(s);
            }
        }
    }
}

impl Eq for Value {}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Boolean(lhs), Value::Boolean(rhs)) => lhs == rhs,
            (Value::Symbol(lhs), Value::Symbol(rhs)) => lhs == rhs,
            (Value::String(lhs), Value::String(rhs)) => lhs == rhs,
            (Value::Integer(lhs, ls), Value::Integer(rhs, rs)) => lhs == rhs && ls == rs,
            (Value::Decimal(lhs, ls), Value::Decimal(rhs, rs)) => lhs == rhs && ls == rs,
            _ => false,
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, _other: &Self) -> Option<Ordering> {
        todo!()
    }

    fn lt(&self, _other: &Self) -> bool {
        todo!()
    }

    fn le(&self, _other: &Self) -> bool {
        todo!()
    }

    fn gt(&self, _other: &Self) -> bool {
        todo!()
    }

    fn ge(&self, _other: &Self) -> bool {
        todo!()
    }
}

impl Add<Self> for Value {
    type Output = Value;

    fn add(self, _rhs: Self) -> Self::Output {
        todo!()
    }
}

impl AddAssign<Self> for Value {
    fn add_assign(&mut self, _rhs: Self) {
        todo!()
    }
}

impl Sub<Self> for Value {
    type Output = Value;

    fn sub(self, _rhs: Self) -> Self::Output {
        todo!()
    }
}

impl SubAssign<Self> for Value {
    fn sub_assign(&mut self, _rhs: Self) {
        todo!()
    }
}
