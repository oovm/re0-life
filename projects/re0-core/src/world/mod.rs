use rand::RngCore;
pub use self::dict::{Dict, get_flatten_vec};

mod mode;
mod dict;



#[derive(Debug, Clone)]
pub struct World {
    mode: WorldConfig,
}
#[derive(Debug, Clone)]
pub struct WorldConfig {
    name_type: NameMode,
    name_data: Dict<Vec<String>>
}
