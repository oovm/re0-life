use crate::Result;
use crate::Rule;
use pest::error::{Error, ErrorVariant};
use pest::iterators::Pair;

pub use self::atom::Atom;
pub use self::collection::{get_flatten_vec, Dict};
pub use self::number::NumberLiteral;

mod atom;
mod collection;
mod number;

pub(crate) fn error_span<T>(s: Pair<Rule>, message: String) -> Result<T> {
    let error = ErrorVariant::CustomError { message };
    Err(Error::new_from_span(error, s.as_span()))
}
