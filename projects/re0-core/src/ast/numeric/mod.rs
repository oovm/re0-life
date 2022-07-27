use re0_pest::{Pair, Rule};
use crate::ast::KeyLiteral;

#[derive(Debug, Clone)]
pub struct NumberLiteral {
    // Either
    value: KeyLiteral,
    suffix: String,
}

impl NumberLiteral {
    pub fn get_i64(&self) -> i64 {
        match self.value {
            KeyLiteral::String(_) => unreachable!(),
            KeyLiteral::Integer(n) => {n}
            KeyLiteral::Decimal(n) => {n as i64}
        }
    }
    pub fn get_f64(&self) -> f64 {
        match self.value {
            KeyLiteral::String(_) => unreachable!(),
            KeyLiteral::Integer(n) => {n as f64}
            KeyLiteral::Decimal(n) => {n}
        }
    }
    pub fn get_f32(&self) -> f32 {
        match self.value {
            KeyLiteral::String(_) => unreachable!(),
            KeyLiteral::Integer(n) => {n as f32}
            KeyLiteral::Decimal(n) => {n as f32}
        }
    }
    pub fn get_unit(&self) -> &str {
        self.suffix.as_str()
    }
}

impl<'i> From<Pair<'i, Rule>> for NumberLiteral {
    fn from(pairs: Pair<Rule>) -> Self {
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                _ => unreachable!("{:?}", pair.as_rule()),
            }
        }
        todo!()
    }
}

