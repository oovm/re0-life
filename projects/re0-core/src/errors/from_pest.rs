use re0_pest::{pest::error::LineColLocation, Error, Rule};

use crate::{
    errors::{Re0ErrorKind, Re0ErrorLevel},
    Re0Error,
};

impl From<Error<Rule>> for Re0Error {
    fn from(e: Error<Rule>) -> Self {
        let kind = Re0ErrorKind::SyntaxError(e.to_string());
        match e.line_col {
            LineColLocation::Pos(_) => Self { kind: Box::new(kind), level: Re0ErrorLevel::Error, file: None, position: (0, 0) },
            LineColLocation::Span(start, _) => Self {
                kind: Box::new(kind),
                level: Re0ErrorLevel::Hide,
                file: None,
                position: (start.0 as u32, start.1 as u32),
            },
        }
    }
}
