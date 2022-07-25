use rand::{seq::SliceRandom, Rng, RngCore};

use crate::world::{get_flatten_vec, Dict, NameMode, WorldConfig};

impl WorldConfig {
    pub fn random_name(&self, rng: &mut impl RngCore) -> Option<String> {
        self.name_type.random_name(&self.name_data, rng)
    }
}

mod name;

pub struct NameConfig {
    mode: NameMode
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
