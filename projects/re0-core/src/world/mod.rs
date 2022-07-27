pub use crate::value::{Dict, get_flatten_vec};

pub use self::mode::*;

mod mode;

#[derive(Debug, Clone)]
pub struct World {
    mode: WorldConfig,
}

#[derive(Debug, Clone, Default)]
pub struct WorldConfig {
    name: NameConfig,
    time: TimeManager,
}
