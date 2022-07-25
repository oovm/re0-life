
pub use self::dict::{Dict, get_flatten_vec};



mod mode;
mod dict;

#[derive(Debug, Clone)]
pub struct World {
    mode: WorldMode,
}
#[derive(Debug, Clone)]
pub struct WorldMode {
    name_type: NameSetting,
    name_data: Dict<Vec<String>>
}

#[derive(Debug, Clone)]
pub enum NameSetting {
    Chinese
}