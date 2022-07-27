use std::{mem::take, ops::AddAssign, str::FromStr};

use re0_pest::{Pair, Parser, Re0Parser, Rule};

use crate::{
    ast::{ASTKind, ASTNode, NumberLiteral},
    world::{World, WorldConfig},
    Re0Error, Result,
};
use crate::ast::Atom;

mod operators;

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
    let mut state = ParseContext::default();
    let ast = state.parse(include_str!("世界.re0")).unwrap();
    println!("{:#?}", ast);
    world.
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
                Rule::declare_statement => children.push(self.declare_statement(pair)?),
                Rule::LineComment => self.push_document(pair),
                _ => debug_cases!(pair),
            }
        }
        Ok(ASTNode::root(children))
    }
    fn push_document(&mut self, pairs: Pair<Rule>) {
        self.documents.push('\n');
        self.documents.push_str(pairs.as_str().trim_start_matches("///").trim_start_matches("、").trim());
    }
}

impl ParseContext {
    fn declare_statement(&mut self, pairs: Pair<Rule>) -> Result<ASTNode> {
        let mut pairs = pairs.into_inner();
        let kind = pairs.next().unwrap().as_str();
        let symbol = pairs.next().unwrap().as_str();
        let mut children = vec![];
        for pair in pairs {
            match pair.as_rule() {
                Rule::declare_block => children.push(self.declare_block(pair)?),
                _ => debug_cases!(pair),
            }
        }
        Ok(ASTNode::declare_statement(kind, symbol, vec![], children))
    }
    fn declare_block(&mut self, pairs: Pair<Rule>) -> Result<ASTNode> {
        let mut out = ASTNode::default();
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::declare_pair => out = self.declare_pair(pair)?,
                _ => debug_cases!(pair),
            }
        }
        Ok(out)
    }
    fn declare_pair(&mut self, pairs: Pair<Rule>) -> Result<ASTNode> {
        let mut pairs = pairs.into_inner();
        let mut key = self.key(pairs.next().unwrap())?;
        let mut value = ASTNode::default();
        for pair in pairs {
            match pair.as_rule() {
                Rule::Key => key = self.key(pair)?,
                Rule::declare_block => value = self.declare_block(pair)?,
                Rule::statement => value = self.parse_statements(pair)?,
                _ => debug_cases!(pair),
            }
        }
        Ok(ASTNode::pair(key, value))
    }
    fn parse_statements(&mut self, pairs: Pair<Rule>) -> Result<ASTNode> {
        let pair = pairs.into_inner().next().unwrap();
        let out = match pair.as_rule() {
            Rule::if_statement => self.if_statement(pair)?,
            Rule::expression => self.expression(pair),
            _ => debug_cases!(pair),
        };
        Ok(out)
    }
    fn if_statement(&mut self, pairs: Pair<Rule>) -> Result<ASTNode> {
        let mut children = vec![];
        let mut if_true = true;
        let mut cond = ASTNode::default();
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::kw_if => {
                    if let "若非" = pair.as_str() {
                        if_true = false
                    }
                }
                Rule::expression => cond = self.expression(pair),
                Rule::block => continue,
                _ => debug_cases!(pair),
            }
        }
        Ok(ASTNode::if_statement(if_true, cond, children))
    }
}

impl ParseContext {
    fn data(&mut self, pairs: Pair<Rule>) -> Result<ASTNode> {
        let pair = pairs.into_inner().next().unwrap();
        let symbol = match pair.as_rule() {
            Rule::SYMBOL => self.symbol(pair)?,
            Rule::Number => ASTNode::number(NumberLiteral::try_from(pair)?),
            _ => debug_cases!(pair),
        };
        Ok(symbol)
    }
    fn key(&mut self, pairs: Pair<Rule>) -> Result<Atom> {
        let head = pairs.into_inner().next().unwrap();
        let symbol = match head.as_rule() {
            Rule::SYMBOL => Atom::Symbol(head.as_str().to_string()),
            Rule::Integer => Atom::try_i64(head.as_str())?,
            _ => debug_cases!(head),
        };
        Ok(symbol)
    }
    fn symbol(&mut self, pairs: Pair<Rule>) -> Result<ASTNode> {
        Ok(ASTNode::symbol(pairs.as_str().trim_matches('`')))
    }
}
