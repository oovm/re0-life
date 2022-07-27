use std::{mem::take, ops::AddAssign, str::FromStr};

use re0_pest::{Pair, Parser, Re0Parser, Rule};

use crate::{
    world::{World, WorldConfig},
    Re0Error, Result,
};

struct ParseContext {
    errors: Vec<Re0Error>,
    documents: String,
}

impl Default for ParseContext {
    fn default() -> Self {
        Self { errors: vec![], documents: "".to_string() }
    }
}

#[test]
fn test() {
    let mut world = World::default();
    world.parse(include_str!("世界.re0")).unwrap()
}

macro_rules! debug_cases {
    ($i:ident) => {{
        println!("Rule::{:?}=>continue,", $i.as_rule());
        println!("Span: {:?}", $i.as_span());
        println!("Text: {}", $i.as_str());
        unreachable!();
    }};
}

impl World {
    pub fn parse(&mut self, input: &str) -> Result<()> {
        let mut state = ParseContext::default();
        self.parse_inner(input, &mut state)
    }
    fn parse_inner(&mut self, input: &str, ctx: &mut ParseContext) -> Result<()> {
        let parsed = Re0Parser::parse(Rule::program, input)?;
        for pair in parsed {
            match pair.as_rule() {
                Rule::declare_statement => self.declare_statement(pair, ctx)?,
                _ => debug_cases!(pair),
            }
        }
        todo!()
    }
}

impl World {
    fn declare_statement(&mut self, pairs: Pair<Rule>, ctx: &mut ParseContext) -> Result<()> {
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::SYMBOL => continue,
                _ => debug_cases!(pair),
            }
        }
        Ok(())
    }
}
