use crate::ast::{ASTKind, ASTNode};
use std::fmt::{Debug, Formatter};

impl Debug for ASTNode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.kind.fmt(f)
    }
}

impl Debug for ASTKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ASTKind::Root(v) => f.debug_list().entries(v.iter()).finish()?,
            ASTKind::Declare(v) => Debug::fmt(v, f)?,
            ASTKind::IfStatement(v) => Debug::fmt(v, f)?,
            ASTKind::Expression(v) => Debug::fmt(v, f)?,
            ASTKind::Block(v) => Debug::fmt(v, f)?,
            ASTKind::Dict(v) => Debug::fmt(v, f)?,
            ASTKind::Pair(k, v) => Debug::fmt(v, f)?,
            ASTKind::Value(v) => Debug::fmt(v, f)?,
            ASTKind::Never => {}
        }
        Ok(())
    }
}
