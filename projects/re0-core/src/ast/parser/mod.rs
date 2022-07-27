use std::{mem::take, ops::AddAssign, str::FromStr};

use re0_pest::{Pair, Parser, Re0Parser, Rule};

use crate::{
    world::{World, WorldConfig},
    Re0Error, Result,
};
use crate::ast::{ASTKind, ASTNode};

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
    let mut state = ParseContext::default();
    let ast = state.parse(include_str!("世界.re0")).unwrap();
    println!("{:#?}", ast);
}

macro_rules! debug_cases {
    ($i:ident) => {{
        println!("Rule::{:?}=>continue,", $i.as_rule());
        println!("Span: {:?}", $i.as_span());
        println!("Text: {}", $i.as_str());
        unreachable!();
    }};
}

impl ParseContext {
    fn parse(&mut self, input: &str) -> Result<ASTNode> {
        let parsed = Re0Parser::parse(Rule::program, input)?;
        let mut children = vec![];
        for pair in parsed {
            match pair.as_rule() {
                Rule::EOI => continue,
                Rule::declare_statement => {
                    children.push(self.declare_statement(pair)?)
                },
                _ => debug_cases!(pair),
            }
        }
        Ok(ASTNode::root(children))
    }
}

impl ParseContext {
    fn declare_statement(&mut self, pairs: Pair<Rule>) -> Result<ASTNode> {
        let mut pairs = pairs.into_inner();
        let kind = pairs.next().unwrap().as_str();
        let symbol = pairs.next().unwrap().as_str();
        for pair in pairs {
            match pair.as_rule() {
                Rule::SYMBOL => continue,
                Rule::declare_block => continue,
                _ => debug_cases!(pair),
            }
        }
        Ok(ASTNode::declare_statement(kind, symbol, vec![], vec![]))
    }
    fn declare_block(&mut self, pairs: Pair<Rule>) -> Result<()> {
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                _ => debug_cases!(pair),
            }
        }
        Ok(())
    }
}
