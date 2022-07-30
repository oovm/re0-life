use log::{error, log, warn};

use re0_pest::value::Value;

use crate::{world::WorldTemplate, GameVM};

impl WorldTemplate {
    /// `悟性`
    pub fn get_property(&self, name: &str) -> Value {
        match name {
            "生日" | "birth_day" | "birthday" => return Value::from(self.mode.time.format_time(&self.mode.time.birth_day)),
            &_ => {}
        }

        match self.property.get(name) {
            Some(s) => s.clone(),
            None => {
                warn!("{} 属性不存在", name);
                Value::Null
            }
        }
    }
    fn get_property_mut(&mut self, name: &str) -> &mut Value {
        if !self.property.contains_key(name) {
            warn!("{} 属性不存在, 新建该属性", name);
            self.property.insert(name.to_string(), Value::Null);
        }
        unsafe { self.property.get_mut(name).unwrap_unchecked() }
    }
    /// `悟性 = 10`
    pub fn set_property(&mut self, name: &str, value: Value) {
        *self.get_property_mut(name) = value;
    }
    /// `悟性 += 10`
    pub fn add_property(&mut self, name: &str, value: Value) {
        *self.get_property_mut(name) += value;
    }
    /// `悟性 -= 10`
    pub fn sub_property(&mut self, name: &str, value: Value) {
        *self.get_property_mut(name) -= value;
    }
}

impl GameVM {
    pub fn get_property(&self, name: &Value) -> Value {
        match name {
            Value::Symbol(s) => self.world.get_property(s),
            _ => {
                error!("{} 不是合法的属性名", name);
                Value::Null
            }
        }
    }
    pub fn set_property(&mut self, name: &Value, value: Value) {
        match name {
            Value::Symbol(s) => self.world.set_property(s, value),
            _ => error!("{} 不是合法的属性名", name),
        }
    }
    pub fn add_property(&mut self, name: &Value, value: Value) {
        match name {
            Value::Symbol(s) => self.world.add_property(s, value),
            _ => error!("{} 不是合法的属性名", name),
        }
    }
    pub fn sub_property(&mut self, name: &Value, value: Value) {
        match name {
            Value::Symbol(s) => self.world.sub_property(s, value),
            _ => error!("{} 不是合法的属性名", name),
        }
    }
}
