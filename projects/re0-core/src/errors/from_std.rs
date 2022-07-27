use std::num::{ParseFloatError, ParseIntError};

use crate::{
    errors::{Re0ErrorKind, Re0ErrorLevel},
    Re0Error,
};

impl From<ParseIntError> for Re0Error {
    fn from(e: ParseIntError) -> Self {
        Re0Error { kind: box Re0ErrorKind::SyntaxError(e.to_string()), level: Re0ErrorLevel::Error, file: None, position: None }
    }
}
impl From<ParseFloatError> for Re0Error {
    fn from(e: ParseFloatError) -> Self {
        Re0Error { kind: box Re0ErrorKind::SyntaxError(e.to_string()), level: Re0ErrorLevel::Error, file: None, position: None }
    }
}
