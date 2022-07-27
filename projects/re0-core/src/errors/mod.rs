use std::{
    error::Error,
    fmt::{Display, Formatter},
};
use std::path::PathBuf;

mod from_std;
mod from_pest;
mod display;

#[derive(Debug, Clone)]
pub enum Re0ErrorKind {
    UnknownError,
    SimpleError(String),
    SyntaxError(String)
}
#[derive(Debug, Clone)]
pub struct Re0Error {
    kind: Box<Re0ErrorKind>,
    level: Re0ErrorLevel,
    file: Option<String>,
    position: Option<(u32, u32)>
}


#[derive(Debug, Clone)]
pub enum Re0ErrorLevel {
    Hide,
    Info,
    Warning,
    Error,
}

pub type Result<T> = std::result::Result<T, Re0Error>;

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
        Self { level: level.into(), ..self }
    }
    #[inline]
    pub fn with_file_info(self, file: impl Into<String>) -> Self {
        Self { file: Some(file.into()), ..self }
    }
    #[inline]
    pub fn with_file(self, file: PathBuf) -> Self {
        self.with_file_info(file.to_str().unwrap_or("").to_string())
    }
    #[inline]
    pub fn with_line_column(self, line: u32, column: u32) -> Self {
        Self { position: Some((line, column)), ..self }
    }
    #[inline]
    pub fn simple_error<S>(msg: S) -> Self
    where
        S: Into<String>,
    {
        let kind = box Re0ErrorKind::SimpleError(msg.into());
        Self { kind, ..Self::default() }
    }
}
