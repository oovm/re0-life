pub use self::{
    dict::{get_flatten_vec, Dict},
    key::Atom,
    numeric::NumberLiteral
};

mod dict;
mod key;
mod parser;
mod numeric;

#[derive(Debug, Clone, Default)]
pub struct ASTNode {
    pub kind: ASTKind,
}

#[derive(Debug, Clone)]
pub enum ASTKind {
    Root(Vec<ASTNode>),
    Declare(DeclareStatement),
    Block(Vec<ASTNode>),
    Key(Atom),
    Number(NumberLiteral),
    Symbol(String),
    Never,
}

pub struct IfStatement {
    if_true: bool,
    children: Vec<ASTNode>,
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
        Self { kind: ASTKind::Declare(s)}
    }

    pub fn block(children: Vec<ASTNode>) -> Self {
        Self { kind: ASTKind::Block(children) }
    }

    pub fn symbol(symbol: &str) -> Self {
        Self { kind: ASTKind::Symbol(symbol.to_string()) }
    }

    pub fn key<K>(input: K) -> Self
    where
        K: Into<Atom>,
    {
        Self { kind: ASTKind::Key(input.into()) }
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

