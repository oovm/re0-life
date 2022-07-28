use chrono::NaiveDateTime;

use re0_pest::ast::{ASTKind, ASTNode, IfStatement};

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
    fn is_true(&self, vm: &mut GameVM) -> Result<bool> {
        match self.evaluate(vm)?.kind {
            ASTKind::Boolean(v) => Ok(v),
            _ => unreachable!(),
        }
    }
}

impl Evaluate for ASTNode {
    fn evaluate(&self, vm: &mut GameVM) -> Result<ASTNode> {
        let out = match &self.kind {
            ASTKind::Root(_) | ASTKind::Declare(_) => unreachable!(),
            ASTKind::IfStatement(s) => s.evaluate(vm)?,
            ASTKind::Expression(_) => {}
            ASTKind::Block(s) => {
                let mut children = vec![];
                for child in s {
                    children.push(child.evaluate(vm)?);
                }
                ASTNode::block(children)
            }
            ASTKind::Pair(_, _) => {}
            ASTKind::Number(_) | ASTKind::Symbol(_) | ASTKind::Boolean(_) | ASTKind::Never => self.clone(),
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
