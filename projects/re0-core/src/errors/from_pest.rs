use re0_pest::{
    pest::error::{ErrorVariant, LineColLocation},
    Error, Rule,
};

use crate::{
    errors::{Re0ErrorKind, Re0ErrorLevel},
    Re0Error,
};

impl From<Error<Rule>> for Re0Error {
    fn from(e: Error<Rule>) -> Self {
        let kind = match e.variant {
            ErrorVariant::ParsingError { positives, negatives } => {
                format!(
                    "期望: {}\n实际: {}",
                    positives.iter().map(|s| format!("{s:?}")).intersperse(", ".to_string()).collect::<String>(),
                    negatives.iter().map(|s| format!("{s:?}")).intersperse(", ".to_string()).collect::<String>(),
                )
            }
            ErrorVariant::CustomError { message } => message,
        };
        let position = match e.line_col {
            LineColLocation::Pos(_) => None,
            LineColLocation::Span(start, _) => Some((start.0 as u32, start.1 as u32)),
        };
        Self { kind: box Re0ErrorKind::SyntaxError(kind), level: Re0ErrorLevel::Error, file: None, position }
    }
}
