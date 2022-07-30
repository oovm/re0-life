use std::collections::BTreeMap;

use re0_pest::value::Value;

pub use crate::world::{event::Event, item::Talent};

pub use self::mode::*;

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

impl WorldTemplate {
    pub fn parse_rarity(rarity: &str) -> usize {
        match rarity {
            "普通" | "白" => 100,
            "稀有" | "绿" => 50,
            "罕见" | "蓝" => 25,
            "史诗" | "紫" => 10,
            "传说" | "橙" => 5,
            "神话" | "红" => 1,
            _ => 100,
        }
    }
}
