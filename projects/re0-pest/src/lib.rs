#![feature(box_syntax)]
#![feature(once_cell)]

pub use pest::error::Error;

pub use re0::{Re0Parser, Rule};

mod ast;
mod re0;

pub type Result<T> = std::result::Result<T, Error<Rule>>;
