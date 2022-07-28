use crate::Result;
use crate::Rule;
use pest::error::{Error, ErrorVariant};
use pest::iterators::Pair;

pub use self::atom::Value;
pub use self::collection::{get_flatten_vec, Dict};
mod atom;
mod boolean;
mod collection;
mod number;

pub(crate) fn error_span<T>(s: Pair<Rule>, message: String) -> Result<T> {
    let error = ErrorVariant::CustomError { message };
    Err(Error::new_from_span(error, s.as_span()))
}
