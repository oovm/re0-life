

use crate::ast::{ASTNode};

#[derive(Debug, Clone)]
pub struct BinaryExpression {
    pub lhs: ASTNode,
    pub rhs: ASTNode,
    pub operator: String,
}
