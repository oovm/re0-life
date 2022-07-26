pub enum Value {
    Number(f64, String),
}

pub struct NumberLiteral {
    // Either
    value: Result<f64, i64>,
    suffix: String,
}

impl NumberLiteral {
    pub fn get_value<T>(&self) -> T
    where
        T: From<f64>,
        T: From<i64>,
    {
        match self.value {
            Ok(n) => T::from(n),
            Err(n) => T::from(n),
        }
    }
    pub fn get_unit(&self) -> &str {
        self.suffix.as_str()
    }
}
