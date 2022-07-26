pub use self::dict::{Dict, get_flatten_vec};
pub use self::mode::{NameConfig, NameMode};

mod mode;
mod dict;


#[derive(Debug, Clone)]
pub struct World {
    mode: WorldConfig,
}

#[derive(Debug, Clone)]
pub struct WorldConfig {
    name: NameConfig,
}
