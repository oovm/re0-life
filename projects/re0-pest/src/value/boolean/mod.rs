use crate::ast::{ASTKind, ASTNode};
use crate::value::Value;

impl From<()> for Value {
    fn from(_: ()) -> Self {
        Value::Null
    }
}

impl From<bool> for Value {
    fn from(b: bool) -> Self {
        Value::Boolean(b)
    }
}

impl Value {
    pub fn is_true(&self) -> bool {
        match self {
            Value::Boolean(true) => true,
            _ => false,
        }
    }
}

impl ASTNode {
    pub fn is_true(&self) -> bool {
        match self.kind {
            ASTKind::Value(ref v) => v.is_true(),
            _ => false,
        }
    }
}
