#![feature(never_type)]

pub use errors::{Re0Error, Result};
pub use self::vm::{GameVM, Re0Function, Template};

mod errors;
pub mod value;
pub mod world;
mod vm;
