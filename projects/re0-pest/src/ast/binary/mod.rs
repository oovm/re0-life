use std::cmp::Ordering;
use std::ops::Add;

use crate::ast::ASTNode;

impl Add<Self> for ASTNode {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        todo!()
    }
}

impl PartialEq<Self> for ASTNode {
    fn eq(&self, other: &Self) -> bool {
        todo!()
    }

    fn ne(&self, other: &Self) -> bool {
        todo!()
    }
}

impl PartialOrd<Self> for ASTNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        todo!()
    }

    fn lt(&self, other: &Self) -> bool {
        match (self, other) {
            (ASTNode::NumberLiteral(lhs), ASTNode::NumberLiteral(rhs)) => lhs.value < rhs.value,
            _ => false,
        }
    }

    fn le(&self, other: &Self) -> bool {
        todo!()
    }

    fn gt(&self, other: &Self) -> bool {
        todo!()
    }

    fn ge(&self, other: &Self) -> bool {
        todo!()
    }
}
