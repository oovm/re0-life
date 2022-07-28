use std::cmp::Ordering;
use std::ops::Add;

use crate::ast::{ASTKind, ASTNode};

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
        match (&self.kind, &other.kind) {
            (ASTKind::Value(lhs), ASTKind::Value(rhs)) => lhs.partial_cmp(rhs),
            _ => None,
        }
    }

    fn lt(&self, other: &Self) -> bool {
        match (&self.kind, &other.kind) {
            (ASTKind::Value(lhs), ASTKind::Value(rhs)) => lhs < rhs,
            _ => false,
        }
    }

    fn le(&self, other: &Self) -> bool {
        match (&self.kind, &other.kind) {
            (ASTKind::Value(lhs), ASTKind::Value(rhs)) => lhs <= rhs,
            _ => false,
        }
    }

    fn gt(&self, other: &Self) -> bool {
        match (&self.kind, &other.kind) {
            (ASTKind::Value(lhs), ASTKind::Value(rhs)) => lhs > rhs,
            _ => false,
        }
    }

    fn ge(&self, other: &Self) -> bool {
        match (&self.kind, &other.kind) {
            (ASTKind::Value(lhs), ASTKind::Value(rhs)) => lhs >= rhs,
            _ => false,
        }
    }
}
