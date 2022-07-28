#![feature(never_type)]
#![feature(box_syntax)]
#![feature(iter_intersperse)]
#![feature(once_cell)]

pub use self::vm::{GameVM, Re0Function, Template};
pub use errors::{Re0Error, Result};

mod errors;
mod vm;
pub mod world;
