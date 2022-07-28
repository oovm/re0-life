pub use self::mode::*;

mod event;
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
