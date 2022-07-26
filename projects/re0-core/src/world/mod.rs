pub use crate::value::{get_flatten_vec, Dict};

pub use self::mode::*;

mod mode;
mod parser;

#[derive(Debug, Clone)]
pub struct World {
    mode: WorldConfig,
}

#[derive(Debug, Clone, Default)]
pub struct WorldConfig {
    name: NameConfig,
    time: TimeManager,
}
