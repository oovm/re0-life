pub use self::mode::*;
use crate::world::event::Event;
use re0_pest::value::Value;
use std::collections::BTreeMap;

mod event;
mod mode;
mod parsing;
mod property;

#[derive(Debug, Clone)]
pub struct WorldTemplate {
    pub mode: WorldConfig,
    property: BTreeMap<String, Value>,
    events: BTreeMap<String, Event>,
}

#[derive(Debug, Clone, Default)]
pub struct WorldConfig {
    name: NameConfig,
    time: TimeManager,
}
