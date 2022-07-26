use std::{
    error::Error,
    fmt::{Display, Formatter},
};
use std::collections::btree_set::Range;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum Re0ErrorKind {
    UnknownError,
    InvalidEnumeration(String),
}
#[derive(Debug, Clone)]
pub struct Re0Error {
    kind: Re0ErrorKind,
    level: Re0ErrorLevel,
    file: Option<String>,
}

impl Default for Re0Error {
    fn default() -> Self {
        Self { kind: Re0ErrorKind::UnknownError, level: Re0ErrorLevel::Info, file: None }
    }
}

pub enum Re0ErrorLevel {
    Hide,
    Info,
    Warning,
    Error,
}

pub type Result<T> = std::result::Result<T, Re0Error>;

impl Error for Re0Error {}

impl Display for Re0Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

impl Re0Error {}

impl From<usize> for Re0ErrorLevel {
    fn from(n: usize) -> Self {
        match n {
            0 => Re0ErrorLevel::Hide,
            1 => Re0ErrorLevel::Info,
            2 => Re0ErrorLevel::Warning,
            3 => Re0ErrorLevel::Error,
            _ => Re0ErrorLevel::Info,
        }
    }
}

impl Re0Error {
    #[inline]
    pub fn with_level(self, level: impl Into<Re0ErrorLevel>) -> Self {
        Self { level: level.into(), ..Self }
    }
    #[inline]
    pub fn with_file_info(self, file: impl Into<String>) -> Self {
        Self { file: Some(file.into()), ..Self }
    }
    #[inline]
    pub fn with_file(self, file: PathBuf) -> Self {
        self.with_file_info(file.to_str().unwrap_or("").to_string())
    }
    #[inline]
    pub fn invalid_enumeration<S>(msg: S) -> Self
    where
        S: Into<String>,
    {
        Self { kind: Re0ErrorKind::InvalidEnumeration(msg.into()), ..Self::default() }
    }
}
