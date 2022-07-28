use std::cmp::Ordering;
use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, AddAssign, Sub, SubAssign};

#[derive(Debug, Clone)]
pub enum Value {
    Null,
    Boolean(bool),
    Symbol(String),
    Integer(i64, String),
    Decimal(f64, String),
}

impl Display for Value {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::Boolean(v) => write!(f, "{}", v),
            Value::Symbol(v) => write!(f, "{}", v),
            Value::Integer(v, s) => write!(f, "{}{}", v, s),
            Value::Decimal(v, s) => write!(f, "{}{}", v, s),
        }
    }
}

impl From<&str> for Value {
    fn from(key: &str) -> Self {
        Self::Symbol(key.to_string())
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (_, _) => {
                todo!()
            }
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        todo!()
    }

    fn lt(&self, other: &Self) -> bool {
        todo!()
    }

    fn le(&self, other: &Self) -> bool {
        todo!()
    }

    fn gt(&self, other: &Self) -> bool {
        todo!()
    }

    fn ge(&self, other: &Self) -> bool {
        todo!()
    }
}

impl Add<Self> for Value {
    type Output = Value;

    fn add(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl AddAssign<Self> for Value {
    fn add_assign(&mut self, rhs: Self) {
        todo!()
    }
}

impl Sub<Self> for Value {
    type Output = Value;

    fn sub(self, rhs: Self) -> Self::Output {
        todo!()
    }
}

impl SubAssign<Self> for Value {
    fn sub_assign(&mut self, rhs: Self) {
        todo!()
    }
}
