use std::ops::{Add, AddAssign, SubAssign};

use chrono::NaiveDateTime;

use re0_pest::{
    ast::{ASTKind, ASTNode, BinaryExpression, IfStatement},
    value::Value,
};

use crate::{GameVM, Re0Error, Result};

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
    fn value_mut(&mut self, v: &Value) -> Result<&mut Value> {
        match v {
            Value::Symbol(s) => match self.world.property.get_mut(s) {
                Some(s) => Ok(s),
                None => Err(Re0Error::simple_error(" {} 属性不存在").with_level(1)),
            },
            _ => Err(Re0Error::simple_error("{} 不是一个属性名").with_level(2)),
        }
    }
    fn try_add(&mut self, lhs: Value, rhs: Value) -> Result<Value> {
        Ok(Value::from(self.value_mut(&lhs)?.add_assign(rhs)))
    }
    fn try_sub(&mut self, lhs: Value, rhs: Value) -> Result<Value> {
        Ok(Value::from(self.value_mut(&lhs)?.sub_assign(rhs)))
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
        let lhs = self.lhs.evaluate(vm)?;
        let rhs = self.rhs.evaluate(vm)?;
        let out = match self.operator.as_str() {
            ">" => Value::from(lhs > rhs),
            "<" => Value::from(lhs < rhs),
            ">=" => Value::from(lhs >= rhs),
            "<=" => Value::from(lhs <= rhs),
            "==" => Value::from(lhs == rhs),
            "!=" => Value::from(lhs != rhs),
            "+" => lhs + rhs,
            "+=" => vm.try_add(lhs, rhs)?,
            "-" => lhs - rhs,
            "-=" => vm.try_sub(lhs, rhs)?,
            // "*" => self.lhs.evaluate(vm)? * self.rhs.evaluate(vm)?,
            // "*=" => vm.get_value_mut(self.lhs.evaluate(vm)?) *= self.rhs.evaluate(vm)?,
            // "/" => self.lhs.evaluate(vm)? / self.rhs.evaluate(vm)?,
            // "/=" => self.lhs.evaluate(vm)? /= self.rhs.evaluate(vm)?,
            // "%" => self.lhs.evaluate(vm)? % self.rhs.evaluate(vm)?,
            // "&&" => self.lhs.evaluate(vm)? && self.rhs.evaluate(vm)?,
            // "||" => self.lhs.evaluate(vm)? || self.rhs.evaluate(vm)?,
            _ => unreachable!(),
        };
        return Ok(out);
    }
}
