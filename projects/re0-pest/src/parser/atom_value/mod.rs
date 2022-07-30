use std::{f64, str::FromStr};

use pest::iterators::Pair;

use crate::parser::ParseContext;
use crate::value::{error_span, Value};
use crate::{Result, Rule};

impl Value {
    pub(crate) fn ast_i64(s: Pair<Rule>, suffix: &str) -> Result<Self> {
        match i64::from_str(s.as_str()) {
            Ok(o) => Ok(Value::Integer(o, suffix.to_string())),
            Err(e) => error_span(s, e.to_string()),
        }
    }
    pub(crate) fn ast_f64(s: Pair<Rule>, suffix: &str) -> Result<Self> {
        match f64::from_str(s.as_str()) {
            Ok(o) => Ok(Value::Decimal(o, suffix.to_string())),
            Err(e) => error_span(s, e.to_string()),
        }
    }
}

impl ParseContext {
    pub(crate) fn number(&mut self, pairs: Pair<Rule>) -> Result<Value> {
        let mut pairs = pairs.into_inner();
        let num = pairs.next().unwrap();
        let suffix = match pairs.next() {
            Some(s) => s.as_str(),
            None => "",
        };
        match num.as_rule() {
            Rule::Integer => Value::ast_i64(num, suffix),
            _ => unreachable!("{:?}", num.as_rule()),
        }
    }
}
