use std::{f64, str::FromStr};

use pest::iterators::Pair;

use crate::ast::parser::ParseContext;
use crate::value::NumberLiteral;
use crate::value::{error_span, Atom};
use crate::{Result, Rule};

impl Atom {
    pub(crate) fn try_as_i64(s: Pair<Rule>) -> Result<Self> {
        match i64::from_str(s.as_str()) {
            Ok(o) => Ok(Atom::Integer(o, String::new())),
            Err(e) => error_span(s, e.to_string()),
        }
    }
    pub(crate) fn try_f64(s: Pair<Rule>) -> Result<Self> {
        match f64::from_str(s.as_str()) {
            Ok(o) => Ok(Atom::Decimal(o, String::new())),
            Err(e) => error_span(s, e.to_string()),
        }
    }
}

impl ParseContext {
    pub(super) fn number(&mut self, pairs: Pair<Rule>) -> Result<NumberLiteral> {
        let mut pairs = pairs.into_inner();
        let num = pairs.next().unwrap();
        let value = match num.as_rule() {
            Rule::Integer => Atom::try_as_i64(num)?,
            _ => unreachable!("{:?}", num.as_rule()),
        };
        let suffix = match pairs.next() {
            Some(s) => s.as_str(),
            None => "",
        };
        Ok(NumberLiteral::new(value, suffix))
    }
}
