use std::{
    error::Error,
    fmt::{Display, Formatter},
};

#[derive(Debug, Copy, Clone)]
pub enum Re0ErrorKind {
    UnknownError,
}
#[derive(Debug, Copy, Clone)]
pub struct Re0Error {
    kind: Re0ErrorKind,
}

pub type Result<T> = std::result::Result<T, Re0Error>;

impl Error for Re0Error {}

impl Display for Re0Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
