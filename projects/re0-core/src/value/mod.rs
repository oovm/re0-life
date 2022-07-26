use crate::{Re0Error, Result};
pub use self::dict::{Dict, get_flatten_vec};
mod dict;


pub enum Value {
    Number(f64, String),
}

pub struct NumberLiteral {
    // Either
    value: std::result::Result<f64, i64>,
    suffix: String,
}

impl NumberLiteral {
    pub fn get_i64(&self) -> i64 {
        match self.value {
            Ok(n) => n as i64,
            Err(n) => n as i64,
        }
    }
    pub fn get_f64(&self) -> f64 {
        match self.value {
            Ok(n) => n as f64,
            Err(n) => n as f64,
        }
    }
    pub fn get_f32(&self) -> f32 {
        match self.value {
            Ok(n) => n as f32,
            Err(n) => n as f32,
        }
    }
    pub fn get_unit(&self) -> &str {
        self.suffix.as_str()
    }
}


