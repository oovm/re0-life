use std::collections::BTreeMap;

mod mode;


#[derive(Debug, Clone)]
pub struct World {
    mode: WorldMode,
}
#[derive(Debug, Clone)]
pub struct WorldMode {
    name: NameSetting,
    name_data: NameData
}

pub type NameData = BTreeMap<String, Vec<String>>;

#[derive(Debug, Clone)]
pub enum NameSetting {
    Chinese
}