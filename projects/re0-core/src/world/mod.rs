pub use self::mode::*;
pub use crate::world::{event::Event, item::Talent};
use re0_pest::value::Value;
use std::collections::BTreeMap;

mod event;
mod item;
mod mode;
mod parsing;
mod property;

#[derive(Debug, Clone)]
pub struct WorldTemplate {
    pub mode: WorldConfig,
    property: BTreeMap<String, Value>,
    talents: BTreeMap<String, Talent>,
    events: BTreeMap<String, Event>,
}

#[derive(Debug, Clone, Default)]
pub struct WorldConfig {
    name: NameConfig,
    time: TimeManager,
}
