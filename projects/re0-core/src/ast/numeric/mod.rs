use std::str::FromStr;

use re0_pest::{Pair, Rule};

use crate::{ast::Atom, Re0Error};

#[derive(Debug, Clone)]
pub struct NumberLiteral {
    // Either
    value: Atom,
    suffix: String,
}

impl NumberLiteral {
    pub fn get_i64(&self) -> i64 {
        match self.value {
            Atom::String(_) => unreachable!(),
            Atom::Integer(n) => n,
            Atom::Decimal(n) => n as i64,
        }
    }
    pub fn get_f64(&self) -> f64 {
        match self.value {
            Atom::String(_) => unreachable!(),
            Atom::Integer(n) => n as f64,
            Atom::Decimal(n) => n,
        }
    }
    pub fn get_f32(&self) -> f32 {
        match self.value {
            Atom::String(_) => unreachable!(),
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

    fn try_from(pairs: Pair<'i, Rule>) -> Result<Self, Self::Error> {
        let mut pairs = pairs.into_inner();
        let num = pairs.next().unwrap();
        let value = match num.as_rule() {
            Rule::Integer => Atom::from(i64::from_str(num.as_str())?),
            _ => unreachable!("{:?}", num.as_rule()),
        };
        let suffix = match pairs.next() {
            Some(s) => s.as_str(),
            None => "",
        };
        Ok(NumberLiteral { value, suffix: suffix.to_string() })
    }
}
