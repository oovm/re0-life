use std::{mem::take, ops::AddAssign, str::FromStr};

use re0_pest::{Pair, Parser, Re0Parser, Rule};

use crate::{world::World, Result, Re0Error};
use crate::world::WorldConfig;

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

   include_str!("世界.re0")
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
    pub fn parse(&mut self, input:&str) -> Result<()> {
        let mut state = ParseContext::default();
        self.parse_inner(input, &mut state)
    }
    fn parse_inner(&mut self, input: &str, ctx: &mut ParseContext) -> Result<()> {
        let parsed = Re0Parser::parse(Rule::program, input)?;
        for pair in parsed {
            match pair.as_rule() {

                _ => debug_cases!(pair),
            }
        }
        todo!()
    }
}

impl ParseContext {
    // pub fn parse_schema_statement(&mut self, pairs: Pair<Rule>, node: &mut JssSchema) -> Result<()> {
    //     for pair in pairs.into_inner() {
    //         match pair.as_rule() {
    //             Rule::SYMBOL => node.set_name(pair.as_str()),
    //             Rule::block => self.parse_block(pair, node)?,
    //             _ => debug_cases!(pair),
    //         }
    //     }
    //     Ok(())
    // }
}

