use crate::{Re0Error, Result};

pub use self::dict::{Dict, get_flatten_vec};

mod dict;
mod parser;

#[derive(Debug, Clone)]
pub struct ASTNode {
    kind: ASTKind,
    children: Vec<ASTNode>,
}

#[derive(Debug, Clone)]
pub enum ASTKind {
    Root,
    Declare(DeclareStatement),
    Number(NumberLiteral),
    Symbol(String),
}

impl ASTNode {
    pub fn root(children: Vec<ASTNode>) -> Self {
        Self { kind: ASTKind::Root, children }
    }

    pub fn declare_statement(keyword: &str, symbol: &str, modifiers: Vec<String>, children: Vec<ASTNode>) -> Self {
        let s = DeclareStatement { keyword: keyword.to_string(), symbol: symbol.to_string(), modifiers };
        Self { kind: ASTKind::Declare(s), children }
    }

    pub fn symbol(symbol: &str) -> Self {
        Self { kind: ASTKind::Symbol(symbol.to_string()), children: vec![] }
    }
}

impl From<ASTNode> for String {
    fn from(node: ASTNode) -> Self {
        match node.kind {
            ASTKind::Symbol(s) => {s}
            _ => unreachable!()
        }
    }
}

#[derive(Debug, Clone)]
pub struct DeclareStatement {
    keyword: String,
    symbol: String,
    modifiers: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct NumberLiteral {
    // Either
    value: std::result::Result<f64, i64>,
    suffix: String,
}

impl NumberLiteral {
    pub fn get_i64(&self) -> i64 {
        match self.value {
            Ok(n) => n as i64,
            Err(n) => n as i64,
        }
    }
    pub fn get_f64(&self) -> f64 {
        match self.value {
            Ok(n) => n as f64,
            Err(n) => n as f64,
        }
    }
    pub fn get_f32(&self) -> f32 {
        match self.value {
            Ok(n) => n as f32,
            Err(n) => n as f32,
        }
    }
    pub fn get_unit(&self) -> &str {
        self.suffix.as_str()
    }
}
