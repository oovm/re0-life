use std::str::FromStr;

use distributions::Bernoulli;
use rand::{distributions,  Rng, seq::SliceRandom};
use rand::distributions::Distribution;

use crate::world::{Dict, get_flatten_vec, WorldConfig};

#[derive(Debug, Clone)]
pub struct NameConfig {
    mode: NameMode,
    preset: Vec<String>,
    last_name: Vec<String>,
    mid_name: Vec<String>,
    first_name: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum NameMode {
    Chinese,
}

impl Default for NameMode {
    fn default() -> Self {
        Self::Chinese
    }
}

impl FromStr for NameMode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let out = match s {
            "中国" | "chinese" => NameMode::Chinese,
            _ => return Err(()),
        };
        Ok(out)
    }
}

impl NameConfig {
    pub fn new(data: &Dict<Vec<String>>) -> Self {
        Self {
            mode: NameMode::Chinese,
            preset: get_flatten_vec(data, &["预设", "preset"]),
            last_name: get_flatten_vec(data, &["姓氏", "姓", "last_name"]),
            mid_name: get_flatten_vec(data, &["中间名", "middle_name"]),
            first_name: get_flatten_vec(data, &["名字", "名", "first_name"]),
        }
    }
    pub fn set_mode(&mut self, mode: &str) {
        match NameMode::from_str(mode) {
            Ok(o) => self.mode = o,
            Err(_) => {}
        }
    }
}

impl NameConfig {
    pub fn count_preset(&self) -> u32 {
        self.preset.len() as u32
    }
    pub fn count_all(&self) -> u32 {
        let all = self.preset.len() + self.last_name.len();
        all as u32
    }

    pub fn random_name(&self, rng: &mut impl Rng) -> Option<String> {
        let mut ratio = Bernoulli::from_ratio(self.count_preset(), self.count_all()).ok()?;
        if ratio.sample(rng) { self.random_name_preset(rng) } else { self.random_name_combine(rng) }
    }
    pub fn random_name_preset(&self, rng: &mut impl Rng) -> Option<String> {
        self.preset.choose(rng).cloned()
    }
    pub fn random_name_combine(&self, rng: &mut impl Rng) -> Option<String> {
        let name = match self.mode {
            // 姓 + 名
            NameMode::Chinese => format!("{}{}", self.last_name.choose(rng)?, self.first_name.choose(rng)?),
        };
        Some(name)
    }
}
