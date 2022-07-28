use std::ops::{Add, AddAssign};

use chrono::NaiveDateTime;

use re0_pest::{
    ast::{ASTKind, ASTNode, BinaryExpression, IfStatement},
    value::Value,
};

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
    pub fn get_value_mut(&mut self, key: Value) -> &mut Value {
        todo!()
    }
}

pub trait Evaluate {
    fn evaluate(&self, vm: &mut GameVM) -> Result<Value>;
}

impl Evaluate for ASTNode {
    fn evaluate(&self, vm: &mut GameVM) -> Result<Value> {
        let out = match &self.kind {
            ASTKind::Root(_) | ASTKind::Declare(_) => unreachable!(),
            ASTKind::IfStatement(s) => s.evaluate(vm)?,
            ASTKind::Expression(s) => s.evaluate(vm)?,
            ASTKind::Block(s) => {
                let mut last = Value::Null;
                for child in s {
                    last = child.evaluate(vm)?;
                }
                return Ok(last);
            }
            ASTKind::Pair(_, _) => {
                todo!()
            }
            ASTKind::Value(v) => v.clone(),
            ASTKind::Never => unreachable!(),
        };
        Ok(out)
    }
}

impl Evaluate for IfStatement {
    fn evaluate(&self, vm: &mut GameVM) -> Result<Value> {
        let mut last = Value::Null;
        for branch in self.branches() {
            let is_true = branch.condition.evaluate(vm)?.is_true();
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
    fn evaluate(&self, vm: &mut GameVM) -> Result<Value> {
        let out = match self.operator.as_str() {
            ">" => Value::Boolean(self.lhs.evaluate(vm)? > self.rhs.evaluate(vm)?),
            "<" => Value::Boolean(self.lhs.evaluate(vm)? < self.rhs.evaluate(vm)?),
            ">=" => Value::Boolean(self.lhs.evaluate(vm)? >= self.rhs.evaluate(vm)?),
            "<=" => Value::Boolean(self.lhs.evaluate(vm)? <= self.rhs.evaluate(vm)?),
            "==" => Value::Boolean(self.lhs.evaluate(vm)? == self.rhs.evaluate(vm)?),
            "!=" => Value::Boolean(self.lhs.evaluate(vm)? != self.rhs.evaluate(vm)?),
            "+" => self.lhs.evaluate(vm)? + self.rhs.evaluate(vm)?,
            "+=" => Value::from(vm.get_value_mut(self.lhs.evaluate(vm)?).add_assign(self.rhs.evaluate(vm)?)),
            "-" => self.lhs.evaluate(vm)? - self.rhs.evaluate(vm)?,
            "-=" => vm.get_value_mut(self.lhs.evaluate(vm)?) -= self.rhs.evaluate(vm)?,
            "*" => self.lhs.evaluate(vm)? * self.rhs.evaluate(vm)?,
            "*=" => vm.get_value_mut(self.lhs.evaluate(vm)?) *= self.rhs.evaluate(vm)?,
            "/" => self.lhs.evaluate(vm)? / self.rhs.evaluate(vm)?,
            "/=" => self.lhs.evaluate(vm)? /= self.rhs.evaluate(vm)?,
            "%" => self.lhs.evaluate(vm)? % self.rhs.evaluate(vm)?,
            "&&" => self.lhs.evaluate(vm)? && self.rhs.evaluate(vm)?,
            "||" => self.lhs.evaluate(vm)? || self.rhs.evaluate(vm)?,
            _ => unreachable!(),
        };
        return Ok(out);
    }
}
