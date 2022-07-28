use re0_pest::value::Value;

use crate::{world::World, GameVM};

impl World {
    /// `悟性`
    pub fn get_property(&self, name: &str) -> Value {
        match self.property.get(name) {
            Some(_) => {}
            None => {}
        }
    }
    /// `悟性 = 10`
    pub fn set_property(&mut self, name: &str, value: Value) {
        self.property.insert(name.to_string(), value);
    }
    /// `悟性 += 10`
    pub fn add_property(&mut self, name: &str, value: Value) {
        match self.property.get_mut(name) {
            Some(v) => *v += value,
            None => {
                self.property.insert(name.to_string(), value);
            }
        }
    }
    /// `悟性 -= 10`
    pub fn sub_property(&mut self, name: &str, value: Value) {
        match self.property.get_mut(name) {
            Some(v) => *v -= value,
            None => {
                self.property.insert(name.to_string(), value);
            }
        }
    }
}

impl GameVM {
    pub fn get_property(&self, name: &Value) -> Value {
        match name {
            Value::Symbol(s) => {}
            _ => Value::Null,
        }
    }
    pub fn set_property(&mut self, name: &Value, value: Value) {
        self.world.set_property(name, value);
    }
    pub fn add_property(&mut self, name: &Value, value: Value) {
        self.world.add_property(name, value);
    }
    pub fn sub_property(&mut self, name: &Value, value: Value) {
        self.world.sub_property(name, value);
    }
}
