use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone)]
pub enum Atom {
    Null,
    Boolean(bool),
    Symbol(String),
    Integer(i64, String),
    Decimal(f64, String),
}

impl Display for Atom {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Atom::Null => write!(f, "null"),
            Atom::Boolean(v) => write!(f, "{}", v),
            Atom::Symbol(v) => write!(f, "{}", v),
            Atom::Integer(v, s) => write!(f, "{}{}", v, s),
            Atom::Decimal(v, s) => write!(f, "{}{}", v, s),
        }
    }
}

impl From<&str> for Atom {
    fn from(key: &str) -> Self {
        Self::Symbol(key.to_string())
    }
}

impl From<i64> for Atom {
    fn from(n: i64) -> Self {
        Self::Integer(n, String::new())
    }
}

impl From<f64> for Atom {
    fn from(n: f64) -> Self {
        Self::Decimal(n, String::new())
    }
}

impl Atom {
    pub fn as_string(&self) -> String {
        match self {
            Self::Symbol(v) => v.clone(),
            Self::Integer(_, v) => v.to_string(),
            Self::Decimal(_, v) => v.to_string(),
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
