use std::ptr::null;

use pest::iterators::Pair;
use pest::Parser;

use crate::ast::DeclareStatement;
use crate::re0::Re0Parser;
use crate::value::Value;
use crate::{ast::ASTNode, Error, Result, Rule};

pub mod atom_value;
mod operators;

pub struct ParseContext {
    errors: Vec<Error<Rule>>,
    documents: String,
}

impl Default for ParseContext {
    fn default() -> Self {
        Self { errors: vec![], documents: "".to_string() }
    }
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
    pub fn parse(&mut self, input: &str) -> Result<ASTNode> {
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
        if !self.documents.is_empty() {
            self.documents.push('\n');
        }
        self.documents.push_str(pairs.as_str().trim_start_matches("///").trim_start_matches("、").trim());
    }
}

impl ParseContext {
    fn declare_statement(&mut self, pairs: Pair<Rule>) -> Result<DeclareStatement> {
        let mut pairs = pairs.into_inner();
        let kind = pairs.next().unwrap().as_str();
        let symbol = pairs.next().unwrap().as_str();
        let mut modifiers = vec![];
        let mut block = ASTNode::default();
        for pair in pairs {
            match pair.as_rule() {
                Rule::declare_block => block = self.declare_block(pair)?,
                Rule::modifiers => modifiers = self.parse_modifiers(pair),
                _ => debug_cases!(pair),
            }
        }
        Ok(DeclareStatement::new(kind, symbol, modifiers, block, &mut self.documents))
    }
    fn parse_modifiers(&mut self, pairs: Pair<Rule>) -> Vec<String> {
        let mut pairs = pairs.into_inner();
        let mut modifiers = vec![];
        for pair in pairs {
            match pair.as_rule() {
                Rule::SYMBOL => modifiers.push(raw_symbol(pair).to_string()),
                _ => unreachable!(),
            }
        }
        modifiers
    }

    fn list_block(&mut self, pairs: Pair<Rule>) -> Result<ASTNode> {
        let mut out = vec![];
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::statement => out.push(self.parse_statements(pair)?),
                _ => debug_cases!(pair),
            }
        }
        Ok(ASTNode::block(out))
    }
    fn declare_block(&mut self, pairs: Pair<Rule>) -> Result<ASTNode> {
        let mut out = vec![];
        for pair in pairs.into_inner() {
            match pair.as_rule() {
                Rule::declare_pair => out.push(self.declare_pair(pair)?),
                _ => debug_cases!(pair),
            }
        }
        Ok(ASTNode::dict(out))
    }
    fn declare_pair(&mut self, pairs: Pair<Rule>) -> Result<(Value, ASTNode)> {
        let mut pairs = pairs.into_inner();
        let mut key = self.key(pairs.next().unwrap())?;
        let mut value = ASTNode::default();
        for pair in pairs {
            match pair.as_rule() {
                Rule::Key => key = self.key(pair)?,
                Rule::declare_block => value = self.declare_block(pair)?,
                Rule::list_block => value = self.list_block(pair)?,
                Rule::statement => value = self.parse_statements(pair)?,
                _ => debug_cases!(pair),
            }
        }
        Ok((key, value))
    }
    fn parse_statements(&mut self, pairs: Pair<Rule>) -> Result<ASTNode> {
        let pair = pairs.into_inner().next().unwrap();
        let out = match pair.as_rule() {
            Rule::if_statement => self.if_statement(pair)?,
            Rule::expression => self.expression(pair),
            Rule::declare_block => self.declare_block(pair)?,
            _ => debug_cases!(pair),
        };
        Ok(out)
    }
    fn if_statement(&mut self, pairs: Pair<Rule>) -> Result<ASTNode> {
        let children = vec![];
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
        Ok(ASTNode::if_simple(if_true, cond, children))
    }
}

impl ParseContext {
    fn key(&mut self, pairs: Pair<Rule>) -> Result<Value> {
        let head = pairs.into_inner().next().unwrap();
        let symbol = match head.as_rule() {
            Rule::SYMBOL => Value::Symbol(head.as_str().to_string()),
            Rule::Integer => Value::ast_i64(head, "")?,
            _ => debug_cases!(head),
        };
        Ok(symbol)
    }
    fn symbol(&mut self, pairs: Pair<Rule>) -> Result<ASTNode> {
        Ok(ASTNode::symbol(raw_symbol(pairs)))
    }
}

#[inline]
fn raw_symbol(pairs: Pair<Rule>) -> &str {
    pairs.as_str().trim_matches('`')
}
