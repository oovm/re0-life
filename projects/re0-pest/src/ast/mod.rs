pub use self::{
    atom_value::{Atom, NumberLiteral},
    collection::{get_flatten_vec, Dict},
};

mod atom_value;
mod collection;
mod parser;
mod evaluate;

#[derive(Debug, Clone, Default)]
pub struct ASTNode {
    pub kind: ASTKind,
}

#[derive(Debug, Clone)]
pub enum ASTKind {
    Root(Vec<ASTNode>),
    Declare(DeclareStatement),
    IfStatement(Box<IfStatement>),
    Expression(Box<Expression>),
    Block(Vec<ASTNode>),
    Pair(Atom, Box<ASTNode>),
    Number(NumberLiteral),
    Symbol(String),
    Boolean(bool),
    Never,
}

#[derive(Debug, Clone)]
pub struct IfStatement {
    if_true: bool,
    condition: ASTNode,
    children: Vec<ASTNode>,
}

#[derive(Debug, Clone)]
pub struct Expression {
    pub left: ASTNode,
    pub right: ASTNode,
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

    pub fn if_statement(if_true: bool, condition: ASTNode, children: Vec<ASTNode>) -> Self {
        Self { kind: ASTKind::IfStatement(box IfStatement { if_true, condition, children }) }
    }

    pub fn binary_expression(left: ASTNode, right: ASTNode, operator: &str) -> Self {
        Self { kind: ASTKind::Expression(box Expression { left, right, operator: operator.to_string() }) }
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

    pub fn number<N>(input: N) -> Self
    where
        N: Into<NumberLiteral>,
    {
        Self { kind: ASTKind::Number(input.into()) }
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


