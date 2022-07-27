use std::{
    f64,
    fmt::{Debug, Formatter},
    str::FromStr,
};
use std::fmt::Display;

use re0_pest::{Pair, Rule};

use crate::{Re0Error, Result};

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

impl Debug for NumberLiteral {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}{}", self.value, self.suffix)
    }
}

#[derive(Clone)]
pub struct NumberLiteral {
    value: Atom,
    suffix: String,
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
    pub fn try_i64(s: &str) -> Result<Self> {
        Ok(Self::Integer(i64::from_str(s)?))
    }
    pub fn try_f64(s: &str) -> Result<Self> {
        Ok(Self::Decimal(f64::from_str(s)?))
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

impl NumberLiteral {
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

impl<'i> TryFrom<Pair<'i, Rule>> for NumberLiteral {
    type Error = Re0Error;

    fn try_from(pairs: Pair<'i, Rule>) -> Result<Self> {
        let mut pairs = pairs.into_inner();
        let num = pairs.next().unwrap();
        let value = match num.as_rule() {
            Rule::Integer => Atom::try_i64(num.as_str())?,
            _ => unreachable!("{:?}", num.as_rule()),
        };
        let suffix = match pairs.next() {
            Some(s) => s.as_str(),
            None => "",
        };
        Ok(NumberLiteral { value, suffix: suffix.to_string() })
    }
}
