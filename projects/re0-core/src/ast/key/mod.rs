
#[derive(Debug, Clone)]
pub enum KeyLiteral {
    String(String),
    Integer(i64),
    Decimal(f64),
}

impl From<&str> for KeyLiteral {
    fn from(key: &str) -> Self {
        Self::String(key.to_string())
    }
}

impl KeyLiteral {
    pub fn as_str(&self) -> &str {
        match self {
            Self::String(s) => s.as_str(),
            Self::Integer(_) => unreachable!(),
            Self::Decimal(_) => unreachable!(),
        }
    }
    pub fn as_i64(&self) -> i64 {
        match self {
            Self::String(s) => s.parse::<i64>().unwrap(),
            Self::Integer(n) => *n,
            Self::Decimal(_) => unreachable!(),
        }
    }
}