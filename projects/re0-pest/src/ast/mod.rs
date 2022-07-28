use crate::value::Atom;
use crate::value::NumberLiteral;
pub use crate::value::{get_flatten_vec, Dict};
use std::iter::{Chain, Cloned};
use std::slice::Iter;
use std::vec::IntoIter;

mod binary;
mod evaluate;
mod parser;

#[derive(Debug, Clone, Default)]
pub struct ASTNode {
    pub kind: ASTKind,
}

#[derive(Debug, Clone)]
pub enum ASTKind {
    Root(Vec<ASTNode>),
    Declare(DeclareStatement),
    IfStatement(Box<IfStatement>),
    Expression(Box<BinaryExpression>),
    Block(Vec<ASTNode>),
    Pair(Atom, Box<ASTNode>),
    Value(Atom),
    Never,
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
        let always_true = IfBranch { if_true: true, condition: ASTNode { kind: ASTKind::Boolean(true) }, body: self.otherwise.clone() };
        self.branch.iter().cloned().chain(vec![always_true].into_iter())
    }
}

#[derive(Debug, Clone)]
pub struct BinaryExpression {
    pub lhs: ASTNode,
    pub rhs: ASTNode,
    pub operator: String,
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

    pub fn symbol(symbol: &str) -> Self {
        Self { kind: ASTKind::Symbol(symbol.to_string()) }
    }

    pub fn pair(key: Atom, value: ASTNode) -> Self {
        Self { kind: ASTKind::Pair(key, box value) }
    }
}

impl From<ASTNode> for String {
    fn from(node: ASTNode) -> Self {
        match node.kind {
            ASTKind::Symbol(s) => s,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct DeclareStatement {
    keyword: String,
    symbol: String,
    modifiers: Vec<String>,
    children: Vec<ASTNode>,
}
