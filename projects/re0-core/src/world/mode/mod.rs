use crate::world::{NameSetting, WorldMode};
use crate::world::{Dict, get_flatten_vec};

impl WorldMode {
    pub fn random_name(&self) -> String {
        self.name_type.random_name(&self.name_data)
    }
}


impl NameSetting {
    pub fn random_name(&self, data: &Dict<Vec<String>>) -> String {
        let preset = get_flatten_vec(data, &["预设", "preset"]);


        match self {
            // 姓 + 名
            NameSetting::Chinese => {
                "".to_string()
            }
        }
    }
}

