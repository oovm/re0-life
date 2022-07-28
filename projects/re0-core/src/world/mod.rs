pub use self::mode::*;
use re0_pest::value::Value;
use std::collections::BTreeMap;

mod event;
mod mode;
mod property;

#[derive(Debug, Clone)]
pub struct World {
    pub mode: WorldConfig,
    property: BTreeMap<String, Value>,
}

#[derive(Debug, Clone, Default)]
pub struct WorldConfig {
    name: NameConfig,
    time: TimeManager,
}
