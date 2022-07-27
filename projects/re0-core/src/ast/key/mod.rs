
#[derive(Debug, Clone)]
pub enum Atom {
    String(String),
    Integer(i64),
    Decimal(f64),
}

impl From<&str> for Atom {
    fn from(key: &str) -> Self {
        Self::String(key.to_string())
    }
}

impl Atom {
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

impl From<i64>for Atom {
    fn from(n: i64) -> Self {
        Self::Integer(n)
    }
}

impl From<f64>for Atom {
    fn from(n: f64) -> Self {
        Self::Decimal(n)
    }
}