use std::str::FromStr;

use rand::{seq::SliceRandom, Rng, RngCore};

use crate::world::{get_flatten_vec, Dict, WorldConfig};

pub struct NameConfig {
    mode: NameMode,
    preset: Vec<String>,
}

impl NameConfig {
    pub fn new(preset: &Dict<Vec<String>>) -> Self {



        Self { mode, preset }
    }
    pub fn set_mode(&mut self, mode: &str) {
        self.mode = NameMode::from_str(mode).unwrap();
    }
}

#[derive(Debug, Clone)]
pub enum NameMode {
    Chinese,
}

impl FromStr for NameMode {
    type Err = !;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "chinese" => NameMode::Chinese,
            _ => NameMode::Chinese,
        })
    }
}

impl NameConfig {
    pub fn random_name(&self, rng: &mut impl RngCore) -> Option<String> {
        self.name_type.random_name(&self.name_data, rng)
    }
}

impl NameMode {
    pub fn random_name_preset(&self, data: &Dict<Vec<String>>, rng: &mut impl RngCore) -> Option<String> {
        let preset = get_flatten_vec(data, &["预设", "preset"]);
        preset.choose(rng).cloned()
    }
    pub fn random_name(&self, data: &Dict<Vec<String>>, rng: &mut impl RngCore) -> Option<String> {
        let surname = get_flatten_vec(data, &["姓氏", "姓", "last_name"]);
        let mid_name = get_flatten_vec(data, &["中间名", "middle_name"]);
        let first_name = get_flatten_vec(data, &["名字", "名", "first_name"]);
        let name = match self {
            // 姓 + 名
            NameMode::Chinese => format!("{}{}", surname.choose(rng)?, first_name.choose(rng)?),
        };
        Some(name)
    }
}
