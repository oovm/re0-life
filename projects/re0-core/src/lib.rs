#![feature(never_type)]
#![feature(box_syntax)]
#![feature(iter_intersperse)]

pub use errors::{Re0Error, Result};
pub use self::vm::{GameVM, Re0Function, Template};

mod errors;
pub mod ast;
pub mod world;
mod vm;
