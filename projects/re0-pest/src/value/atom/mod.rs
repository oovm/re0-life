use std::fmt::{Debug, Display, Formatter};

#[derive(Debug, Clone)]
pub enum Atom {
    Symbol(String),
    Integer(i64),
    Decimal(f64),
}

impl Display for Atom {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Atom::Symbol(s) => f.write_str(s),
            Atom::Integer(s) => write!(f, "{}", s),
            Atom::Decimal(s) => write!(f, "{}", s),
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
        Self::Integer(n)
    }
}

impl From<f64> for Atom {
    fn from(n: f64) -> Self {
        Self::Decimal(n)
    }
}

impl Atom {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Symbol(s) => s.as_str(),
            Self::Integer(_) => unreachable!(),
            Self::Decimal(_) => unreachable!(),
        }
    }
    pub fn as_i64(&self) -> i64 {
        match self {
            Self::Symbol(s) => s.parse::<i64>().unwrap(),
            Self::Integer(n) => *n,
            Self::Decimal(_) => unreachable!(),
        }
    }
}
