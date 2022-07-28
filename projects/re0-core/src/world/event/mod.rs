use chrono::NaiveDateTime;
use std::ops::Add;

use re0_pest::ast::{ASTKind, ASTNode, BinaryExpression, IfStatement};

use crate::{GameVM, Result};

pub struct Event {
    ///
    pub name: String,
    ///
    pub condition: Vec<ASTNode>,
    ///
    pub age_check: (usize, usize),
    ///
    pub effect: Vec<ASTNode>,
}

impl GameVM {
    pub fn perform_events(&mut self) {}
    pub fn perform_event(&mut self, event: &Event) {}
    pub fn perform_ast(&mut self, node: &ASTNode) {}
}

pub trait Evaluate {
    fn evaluate(&self, vm: &mut GameVM) -> Result<ASTNode>;
}

impl Evaluate for ASTNode {
    fn evaluate(&self, vm: &mut GameVM) -> Result<ASTNode> {
        let out = match &self.kind {
            ASTKind::Root(_) | ASTKind::Declare(_) => unreachable!(),
            ASTKind::IfStatement(s) => s.evaluate(vm)?,
            ASTKind::Expression(s) => s.evaluate(vm)?,
            ASTKind::Block(s) => {
                let mut children = vec![];
                for child in s {
                    children.push(child.evaluate(vm)?);
                }
                ASTNode::block(children)
            }
            ASTKind::Pair(_, _) => {
                todo!()
            }
            ASTKind::Value(_) | ASTKind::Symbol(_) | ASTKind::Boolean(_) | ASTKind::Never => self.clone(),
        };
        Ok(out)
    }
}

impl Evaluate for IfStatement {
    fn evaluate(&self, vm: &mut GameVM) -> Result<ASTNode> {
        let mut last = ASTNode::default();
        for branch in self.branches() {
            let is_true = branch.condition.evaluate(vm)?.is_true()?;
            let is_true = match branch.if_true {
                true => is_true,
                false => !is_true,
            };
            if !is_true {
                continue;
            }
            for child in branch.body {
                last = child.evaluate(vm)?
            }
            return Ok(last);
        }
        return unreachable!();
    }
}

impl Evaluate for BinaryExpression {
    fn evaluate(&self, vm: &mut GameVM) -> Result<ASTNode> {
        let out = match self.operator.as_str() {
            ">" => self.lhs.evaluate(vm)? > self.rhs.evaluate(vm)?,
            "<" => self.lhs.evaluate(vm)? < self.rhs.evaluate(vm)?,
            ">=" => self.lhs.evaluate(vm)? >= self.rhs.evaluate(vm)?,
            "<=" => self.lhs.evaluate(vm)? <= self.rhs.evaluate(vm)?,
            "==" => self.lhs.evaluate(vm)? == self.rhs.evaluate(vm)?,
            "!=" => self.lhs.evaluate(vm)? != self.rhs.evaluate(vm)?,
            "+" => self.lhs.evaluate(vm)? + self.rhs.evaluate(vm)?,
            "-" => self.lhs.evaluate(vm)? - self.rhs.evaluate(vm)?,
            "*" => self.lhs.evaluate(vm)? * self.rhs.evaluate(vm)?,
            "/" => self.lhs.evaluate(vm)? / self.rhs.evaluate(vm)?,
            "%" => self.lhs.evaluate(vm)? % self.rhs.evaluate(vm)?,
            "&&" => self.lhs.evaluate(vm)? && self.rhs.evaluate(vm)?,
            "||" => self.lhs.evaluate(vm)? || self.rhs.evaluate(vm)?,
            _ => unreachable!(),
        };
        return Ok(out);
    }
}
