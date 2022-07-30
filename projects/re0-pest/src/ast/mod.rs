pub use self::binary::BinaryExpression;
use crate::value::Value;
pub use crate::value::{get_flatten_vec, Dict};
use std::collections::{BTreeMap, HashMap};
use std::hash::Hash;
use std::iter::{Chain, Cloned, FromIterator};
use std::slice::Iter;
use std::vec::IntoIter;

mod binary;
mod evaluate;
mod parser;

#[derive(Clone, Default)]
pub struct ASTNode {
    pub kind: ASTKind,
}

#[derive(Clone)]
pub enum ASTKind {
    Root(Vec<ASTNode>),
    Declare(DeclareStatement),
    IfStatement(Box<IfStatement>),
    Expression(Box<BinaryExpression>),
    Block(Vec<ASTNode>),
    FunctionCall(Box<ASTFunction>),
    Dict(HashMap<Value, ASTNode>),
    Value(Value),
    Never,
}

#[derive(Debug, Clone)]
pub struct ASTFunction {
    pub name: String,
    pub args: Vec<ASTNode>,
}

/// 如果 {
/// }
/// 否则 {
/// }
///
/// 若非 {
/// }
/// 又或 {
///
/// }
/// 否则 {
///
/// }
#[derive(Debug, Clone)]
pub struct IfStatement {
    branch: Vec<IfBranch>,
    otherwise: Vec<ASTNode>,
}
#[derive(Debug, Clone)]
pub struct IfBranch {
    pub if_true: bool,
    pub condition: ASTNode,
    pub body: Vec<ASTNode>,
}

impl IfStatement {
    pub fn branches(&self) -> Chain<Cloned<Iter<'_, IfBranch>>, IntoIter<IfBranch>> {
        let always_true = IfBranch { if_true: true, condition: ASTNode::TRUE, body: self.otherwise.clone() };
        self.branch.iter().cloned().chain(vec![always_true].into_iter())
    }
}

impl Default for ASTKind {
    fn default() -> Self {
        Self::Never
    }
}

impl ASTNode {
    pub fn root(children: Vec<ASTNode>) -> Self {
        Self { kind: ASTKind::Root(children) }
    }

    pub fn declare_statement(keyword: &str, symbol: &str, modifiers: Vec<String>, children: Vec<ASTNode>) -> Self {
        let s = DeclareStatement { keyword: keyword.to_string(), symbol: symbol.to_string(), modifiers, children };
        Self { kind: ASTKind::Declare(s) }
    }

    pub fn if_simple(if_true: bool, condition: ASTNode, children: Vec<ASTNode>) -> Self {
        let branch = IfBranch { if_true, condition, body: children };
        Self { kind: ASTKind::IfStatement(box IfStatement { branch: vec![branch], otherwise: vec![] }) }
    }

    pub fn binary_expression(left: ASTNode, right: ASTNode, operator: &str) -> Self {
        Self { kind: ASTKind::Expression(box BinaryExpression { lhs: left, rhs: right, operator: operator.to_string() }) }
    }

    pub fn block(children: Vec<ASTNode>) -> Self {
        Self { kind: ASTKind::Block(children) }
    }

    pub fn dict(kvs: Vec<(Value, ASTNode)>) -> Self {
        Self { kind: ASTKind::Dict(HashMap::from_iter(kvs.into_iter())) }
    }

    pub fn function(name: &str, args: Vec<ASTNode>) -> Self {
        Self { kind: ASTKind::FunctionCall(box ASTFunction { name: name.to_string(), args }) }
    }

    pub fn symbol(symbol: &str) -> Self {
        Self { kind: ASTKind::Value(Value::Symbol(symbol.to_string())) }
    }

    pub const NULL: Self = Self { kind: ASTKind::Value(Value::Null) };
    pub const TRUE: Self = Self { kind: ASTKind::Value(Value::Boolean(true)) };
    pub const FALSE: Self = Self { kind: ASTKind::Value(Value::Boolean(false)) };
}

#[derive(Debug, Clone)]
pub struct DeclareStatement {
    keyword: String,
    symbol: String,
    modifiers: Vec<String>,
    children: Vec<ASTNode>,
}
